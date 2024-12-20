mod panel;

use std::{collections::HashMap, sync::Arc};

use eframe::{egui, Result};
use egui::mutex::Mutex;
use panel::MainPanel;
use server::courier::{self, Courier};
use snafu::ResultExt;
use tokio::runtime::Runtime;
use tracing::{error, info};

use crate::{message::Task, state::AppState, ui::panel::LeftPanel};

pub trait Component {
    fn ui(
        &mut self,
        ctx: &egui::Context,
        state: &mut AppState,
    );
}

pub fn launch() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native("Watchingir", options, Box::new(|_cc| Ok(Box::<App>::default())))
}

struct App {
    rt: Runtime,
    state: AppState,
    task_rx: std::sync::mpsc::Receiver<Task>,
    left_panel: LeftPanel,
    main_panel: Arc<Mutex<MainPanel>>,
    courier: Arc<Courier>,
    constant: GameConstant,
}

impl Default for App {
    fn default() -> Self {
        let state = AppState::try_from_config().unwrap_or_default();
        info!("Loading AppState: {:?}", state);
        let courier = Courier::default();
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            rt: tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap(),
            state,
            task_rx: rx,
            left_panel: LeftPanel::new(tx.clone()),
            main_panel: Arc::new(Mutex::new(MainPanel::new(tx))),
            courier: Arc::new(courier),
            constant: GameConstant::default(),
        }
    }
}

impl eframe::App for App {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        if !self.constant_ready() {}

        while let Ok(task) = self.task_rx.try_recv() {
            match task {
                Task::UpdateMatchDetail => {
                    self.latest_match_detail();
                }
            }
        }

        self.left_panel.ui(ctx, &mut self.state);
        self.main_panel.lock().ui(ctx, &mut self.state);
    }
}

impl App {
    fn constant_ready(&self) -> bool {
        self.constant.is_loaded
    }

    #[tracing::instrument(skip(self))]
    fn latest_match_detail(&mut self) {
        let courier = Arc::clone(&self.courier);
        let steam_api_key = self.state.steam_api_key.clone();
        let account_id = self.state.account_id;
        let main_panel = Arc::clone(&self.main_panel);
        self.rt.spawn(async move {
            let res = courier.latest_match_detail(&steam_api_key, account_id).await;
            match res {
                Ok(match_detail_response) => {
                    let view = match_detail_response.views(account_id);
                    let mut guard = main_panel.lock();
                    guard.update_match_detail(view);
                }
                Err(e) => {
                    error!("Failed to get match detail: {}", e);
                }
            }
        });
    }

    fn fetch_constant(&mut self) -> Result<(), common::Error> {
        let courier = Arc::clone(&self.courier);
        let key = self.state.stratz_api_key.clone();
        self.rt.spawn(async move {
            let res = courier.cache(&key).await;
        });

        Ok(())
    }
}

#[derive(Default)]
struct GameConstant {
    items_map: HashMap<i32, String>,
    heroes_map: HashMap<i32, String>,
    is_loaded: bool,
}

impl GameConstant {
    pub fn from_config() -> Result<Self, common::Error> {
        let (items_map, heroes_map) = Self::read_json()?;

        Ok(Self {
            items_map,
            heroes_map,
            is_loaded: true,
        })
    }

    fn read_json() -> Result<(HashMap<i32, String>, HashMap<i32, String>), common::Error> {
        let items_json = std::fs::read_to_string("config/items.json").whatever_context("Read items.json failed")?;
        let heroes_json = std::fs::read_to_string("config/heroes.json").whatever_context("Read heroes.json failed")?;

        let items: Vec<(i32, String)> = serde_json::from_str(&items_json).whatever_context("Parse items.json failed")?;
        let heroes: Vec<(i32, String)> = serde_json::from_str(&heroes_json).whatever_context("Parse heroes.json failed")?;

        let items_map = items.into_iter().collect();
        let heroes_map = heroes.into_iter().collect();

        Ok((items_map, heroes_map))
    }
}
