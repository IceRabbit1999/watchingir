use eframe::{
    egui,
    egui::{Id, Modal},
};
use tracing::info;

pub struct SteamApiKeyModal {
    pub key: String,
    pub completed: bool,
}

impl SteamApiKeyModal {
    pub fn init_key(
        &mut self,
        ctx: &egui::Context,
    ) {
        if !self.completed {
            let _modal = Modal::new(Id::new("steam_api_key_modal")).show(ctx, |ui| {
                ui.set_width(300.0);
                ui.heading("Enter Your Steam API Key");

                ui.text_edit_singleline(&mut self.key);

                ui.separator();
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Submit").clicked() {
                            info!("Initialize steam API key to: {}", self.key);
                            self.completed = true;
                        }
                    },
                )
            });
        }
    }
}

impl Default for SteamApiKeyModal {
    fn default() -> Self {
        Self {
            key: Default::default(),
            completed: false,
        }
    }
}
