use eframe::egui;

use crate::ui::modal::SteamApiKeyModal;

pub struct LeftPanel {
    /// LeftTopPanel
    key_modal: SteamApiKeyModal,
    menu: Menu,
}
impl LeftPanel {
    pub fn init() -> Self {
        Self {
            key_modal: SteamApiKeyModal::default(),
            menu: Menu::init(),
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
    ) {
        self.show_key(ctx);
        // self.menu.show_menu(ctx);
    }

    // LeftTopPanel

    pub fn init_key(
        &mut self,
        ctx: &egui::Context,
    ) {
        self.key_modal.init_key(ctx);
    }

    fn show_key(
        &mut self,
        ctx: &egui::Context,
    ) {
        egui::SidePanel::left("current_key").show(ctx, |ui| {
            ui.heading("Watchingir");
            ui.separator();
            ui.label("Current Steam API Key:");
            ui.text_edit_singleline(&mut self.key_modal.key);
            ui.add_space(30.0);
            self.menu.show_menu(ui);
        });
    }

    // LeftMenuPanel
}

pub struct Menu {}

impl Menu {
    fn init() -> Self {
        Self {}
    }

    fn show_menu(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::SidePanel::left("menu").show_inside(ui, |ui| {
            ui.heading("Menu");
            ui.separator();
            ui.label("Menu Item 1");
            ui.label("Menu Item 2");
            ui.label("Menu Item 3");
        });
    }
}
