mod panel;

use std::sync::Arc;

use eframe::egui;
use egui::mutex::Mutex;
use panel::MainPanel;
use server::courier::Courier;
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
}

impl Default for App {
    fn default() -> Self {
        let state = AppState::try_from_config().unwrap_or_default();
        info!("Loading AppState: {:?}", state);
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            rt: tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap(),
            state,
            task_rx: rx,
            left_panel: LeftPanel::new(tx.clone()),
            main_panel: Arc::new(Mutex::new(MainPanel::new(tx))),
            courier: Arc::new(Courier::default()),
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
                    self.latest_match_detail();
                }
            }
        }

        self.left_panel.ui(ctx, &mut self.state);
        self.main_panel.lock().ui(ctx, &mut self.state);
    }
}

impl App {
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
}
