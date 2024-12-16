use eframe::egui;
use tracing::info;

use crate::ui::{modal::SteamApiKeyModal, table::MatchDetailTable};

pub struct LeftPanel {
    /// LeftTopPanel
    pub key_modal: SteamApiKeyModal,
    menu: Menu,
    pub account_id: String,
}

pub struct MainPanel {
    pub match_detail: MatchDetailTable,
}

impl MainPanel {
    pub fn new() -> Self {
        Self {
            match_detail: MatchDetailTable::new(),
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
    ) {
        self.match_detail.show(ctx);
    }
}

impl LeftPanel {
    pub fn new() -> Self {
        Self {
            key_modal: SteamApiKeyModal::default(),
            menu: Menu::init(),
            account_id: Default::default(),
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
    ) {
        self.show_config(ctx);
    }

    // LeftTopPanel

    pub fn init_key(
        &mut self,
        ctx: &egui::Context,
    ) {
        self.key_modal.init_key(ctx);
    }

    fn show_config(
        &mut self,
        ctx: &egui::Context,
    ) {
        egui::SidePanel::left("current_config").show(ctx, |ui| {
            ui.heading("Watchingir");
            ui.separator();
            ui.label("Current Steam API Key:");
            ui.text_edit_singleline(&mut self.key_modal.key);
            ui.text_edit_singleline(&mut self.account_id);
            ui.add_space(30.0);
            self.menu.show_menu(ui);
        });
    }

    // LeftMenuPanel
}

pub struct Menu {
    latest_matches: bool,
    friends: bool,
}

impl Menu {
    fn init() -> Self {
        Self {
            latest_matches: false,
            friends: false,
        }
    }

    fn show_menu(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::SidePanel::left("menu").show_inside(ui, |ui| {
            ui.heading("Menu");
            ui.separator();

            if ui.toggle_value(&mut self.latest_matches, "Latest Matches").clicked() {
                self.friends = false;
            }

            if ui.toggle_value(&mut self.friends, "Friends").clicked() {
                self.latest_matches = false;
            }
        });
    }
}
