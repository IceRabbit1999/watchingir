mod friend;
mod mapper;
mod panel;

use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    sync::Arc,
};

use common::data::matches::{MatchDetail, MatchDetailView};
use eframe::{egui, Result};
use egui::{
    mutex::{Mutex, RwLock},
    FontData, FontDefinitions, FontFamily,
};
use panel::MainPanel;
use server::courier::Courier;
use snafu::ResultExt;
use tokio::runtime::Runtime;
use tracing::{error, info, warn};

use crate::{
    error::{JsonSnafu, ReadFileSnafu, ServerSnafu, WriteFileSnafu},
    message::Task,
    state::AppState,
    ui::panel::LeftPanel,
};

pub trait Component {
    fn ui(
        &mut self,
        ctx: &egui::Context,
        state: &mut AppState,
        constant: &Arc<RwLock<GameConstant>>,
    );
}

pub fn launch() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native("Watchingir", options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
}

struct App {
    rt: Runtime,
    state: AppState,
    task_rx: std::sync::mpsc::Receiver<Task>,
    left_panel: LeftPanel,
    main_panel: Arc<Mutex<MainPanel>>,
    courier: Arc<Courier>,
    constant: Arc<RwLock<GameConstant>>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Get font from: https://github.com/lxgw/LxgwWenKai
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "kx_font".to_owned(),
            Arc::new(FontData::from_static(include_bytes!("../../../../assets/LXGWWenKai-Regular.ttf"))),
        );

        fonts.families.get_mut(&FontFamily::Proportional).unwrap().push("kx_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        let state = AppState::try_from_config().unwrap_or_default();
        info!("Loading AppState: {:?}", state);
        let courier = Courier::default();
        let (tx, rx) = std::sync::mpsc::channel();
        let constant = GameConstant::from_config().unwrap_or_default();
        info!("Loading GameConstant: {}", constant);
        Self {
            rt: tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap(),
            state,
            task_rx: rx,
            left_panel: LeftPanel::new(tx.clone()),
            main_panel: Arc::new(Mutex::new(MainPanel::new(tx))),
            courier: Arc::new(courier),
            constant: Arc::new(RwLock::new(constant)),
        }
    }
}

impl eframe::App for App {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        while let Ok(task) = self.task_rx.try_recv() {
            match task {
                Task::UpdateMatchDetail => {
                    if !self.constant_ready() {
                        let result = GameConstant::from_config();
                        match result {
                            Ok(constant) => {
                                self.set_constant(constant);
                            }
                            Err(e) => {
                                warn!("{:?}", e);
                                info!("No game constant cache, fetching from remote");
                                self.fetch_constant();
                            }
                        }
                    }

                    self.latest_match_detail();
                }
            }
        }

        self.left_panel.ui(ctx, &mut self.state, &self.constant);
        self.main_panel.lock().ui(ctx, &mut self.state, &self.constant);
    }
}

impl App {
    // constant

    fn constant_ready(&self) -> bool {
        self.constant.read().is_loaded
    }

    fn set_constant(
        &mut self,
        constant: GameConstant,
    ) {
        *self.constant.write() = constant;
    }

    #[tracing::instrument(skip(self))]
    fn latest_match_detail(&mut self) {
        let courier = Arc::clone(&self.courier);
        let steam_api_key = self.state.steam_api_key.clone();
        let account_ids = self.state.friends.clone();
        let main_panel = Arc::clone(&self.main_panel);
        self.rt.spawn(async move {
            let res = courier
                .latest_friends_match_detail(&steam_api_key, &account_ids)
                .await
                .context(ServerSnafu);
            match res {
                Ok(match_detail_response) => {
                    let view = match_detail_response
                        .into_iter()
                        .map(|(detail, account_id)| MatchDetailView::from_match_detail(detail, account_id, &account_ids).unwrap())
                        .collect::<VecDeque<MatchDetailView>>();

                    let mut guard = main_panel.lock();
                    guard.update_match_detail(view);
                }
                Err(e) => {
                    error!("Failed to get match detail: {:?}", e);
                }
            }
        });
    }

    fn fetch_constant(&mut self) {
        let courier = Arc::clone(&self.courier);
        let key = self.state.stratz_api_key.clone();
        let game_constant = Arc::clone(&self.constant);
        self.rt.spawn(async move {
            let res = courier.constant(&key).await;
            match res {
                Ok(constant) => {
                    let (items, heroes) = constant.spilt();
                    let mut guard = game_constant.write();
                    guard.items_map = items;
                    guard.heroes_map = heroes;
                    guard.is_loaded = true;
                    info!("Game constant fetched successfully");
                }
                Err(e) => {
                    error!("Failed to get constant: {:?}", e);
                }
            }
        });
    }
}

#[derive(Default)]
pub struct GameConstant {
    items_map: HashMap<i32, String>,
    heroes_map: HashMap<i32, String>,
    is_loaded: bool,
}

impl Display for GameConstant {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "GameConstant {{ items: {}, heroes: {}, is_loaded: {} }}",
            self.items_map.len(),
            self.heroes_map.len(),
            self.is_loaded
        )
    }
}

impl Drop for GameConstant {
    fn drop(&mut self) {
        info!("Drop GameConstant, write to config/items.json and config/heroes.json");
        if let Err(e) = self.save_before_drop() {
            error!("Save game constant error: {}", e);
        }
    }
}

impl GameConstant {
    pub fn from_config() -> Result<Self, crate::Error> {
        let (items_map, heroes_map) = Self::read_json()?;

        Ok(Self {
            items_map,
            heroes_map,
            is_loaded: true,
        })
    }

    fn read_json() -> Result<(HashMap<i32, String>, HashMap<i32, String>), crate::Error> {
        let items_json = std::fs::read_to_string("config/items.json").context(ReadFileSnafu { filename: "items.json" })?;
        let heroes_json = std::fs::read_to_string("config/heroes.json").context(ReadFileSnafu { filename: "heroes.json" })?;

        let items: HashMap<i32, String> = serde_json::from_str(&items_json).context(JsonSnafu)?;
        let heroes: HashMap<i32, String> = serde_json::from_str(&heroes_json).context(JsonSnafu)?;

        Ok((items, heroes))
    }

    fn save_before_drop(&self) -> Result<(), crate::Error> {
        let items = serde_json::to_string_pretty(&self.items_map).context(JsonSnafu)?;
        let heroes = serde_json::to_string_pretty(&self.heroes_map).context(JsonSnafu)?;

        std::fs::write("config/items.json", items).context(WriteFileSnafu {
            filename: "config/items.json",
        })?;
        std::fs::write("config/heroes.json", heroes).context(WriteFileSnafu {
            filename: "config/heroes.json",
        })?;

        Ok(())
    }
}
