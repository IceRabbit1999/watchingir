mod modal;
mod panel;
mod table;

use std::{str::FromStr, sync::Arc};

use eframe::egui;
use panel::MainPanel;
use server::courier::Courier;
use tokio::runtime::Runtime;

use crate::{state::AppState, ui::panel::LeftPanel};

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
    left_panel: LeftPanel,
    main_panel: MainPanel,
    courier: Arc<Courier>,
}

impl Default for App {
    fn default() -> Self {
        let state = AppState::try_from_config().unwrap_or_default();

        Self {
            rt: tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap(),
            state,
            left_panel: LeftPanel::new(),
            main_panel: MainPanel::new(),
            courier: Arc::new(Courier::new()),
        }
    }
}

impl eframe::App for App {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        // Left Panel
        self.left_panel.show(ctx);

        self.main_panel.show(ctx);

        self.left_panel.init_key(ctx);
    }
}

impl App {
    async fn update_match_detail(&mut self) {
        let key = &self.left_panel.key_modal.key;
        let account_id = i64::from_str(&self.left_panel.account_id).unwrap();
        let courier = Arc::clone(&self.courier);
        let matches = self
            .rt
            .block_on(async move { courier.latest_match_detail(key, account_id).await.unwrap() });
        self.main_panel.match_detail.matches = vec![matches];
    }
}
