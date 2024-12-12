mod modal;
mod panel;

use eframe::egui;
use tokio::runtime::Runtime;

use crate::ui::panel::LeftPanel;

pub fn launch() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native("Watchingir", options, Box::new(|_cc| Ok(Box::<App>::default())))
}

struct App {
    rt: Runtime,
    left_panel: LeftPanel,
}

impl Default for App {
    fn default() -> Self {
        Self {
            rt: tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap(),
            left_panel: LeftPanel::init(),
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });

        self.left_panel.init_key(ctx);
    }
}
