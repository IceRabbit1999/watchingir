use common::data::matches::MatchDetailView;
use eframe::egui;
use egui::{Id, Modal};
use egui_extras::{Column, TableBuilder};
use tracing::{error, info};

use super::Component;
use crate::{message::Task, state::AppState};

pub struct LeftPanel {
    /// LeftTopPanel
    menu: Menu,
    _task_tx: std::sync::mpsc::Sender<Task>,
}

impl LeftPanel {
    pub fn new(task_tx: std::sync::mpsc::Sender<Task>) -> Self {
        Self {
            menu: Menu::init(),
            _task_tx: task_tx,
        }
    }
}

impl Component for LeftPanel {
    fn ui(
        &mut self,
        ctx: &egui::Context,
        state: &mut AppState,
    ) {
        egui::SidePanel::left("current_config").show(ctx, |ui| {
            ui.heading("Watchingir");
            ui.separator();
            ui.strong("Current Steam API Key:");
            ui.text_edit_singleline(&mut state.steam_api_key);
            ui.strong("Current Account ID:");
            let mut account_id_str = state.account_id.to_string();
            if ui.text_edit_singleline(&mut account_id_str).changed() {
                state.account_id = account_id_str.parse().expect("Invalid account id");
            }
            ui.add_space(30.0);
            self.menu.show_menu(ui);
        });
        if state.steam_api_key.is_empty() {
            Modal::new(Id::new("steam_api_key_modal")).show(ctx, |ui| {
                ui.set_width(300.0);
                ui.heading("Enter Your Steam API Key");

                ui.text_edit_singleline(&mut state.steam_api_key);

                ui.separator();
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Submit").clicked() {
                            info!("Initialize steam API key to: {}", state.steam_api_key);
                        }
                    },
                )
            });
        }
    }
}

pub struct MainPanel {
    matches: Vec<MatchDetailView>,
    task_tx: std::sync::mpsc::Sender<Task>,
}

impl Clone for MainPanel {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Component for MainPanel {
    fn ui(
        &mut self,
        ctx: &egui::Context,
        _state: &mut AppState,
    ) {
        self.table_ui(ctx);
    }
}

impl MainPanel {
    pub fn new(task_tx: std::sync::mpsc::Sender<Task>) -> Self {
        Self {
            matches: Vec::new(),
            task_tx,
        }
    }

    pub fn update_match_detail(
        &mut self,
        matches: Vec<MatchDetailView>,
    ) {
        self.matches = matches;
    }

    fn trigger_update_match_detail(&mut self) {
        if let Err(e) = self.task_tx.send(Task::UpdateMatchDetail) {
            error!("Send Task::UpdateMatchDetail error: {}", e);
        }
    }

    fn table_ui(
        &mut self,
        ctx: &egui::Context,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Latest Match Details");
                if ui.button("Update").clicked() {
                    self.trigger_update_match_detail();
                }
                ui.separator();
                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::remainder().at_least(100.0))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            egui::ScrollArea::horizontal().show(ui, |ui| {
                                if !self.matches.is_empty() {
                                    self.rows(ui);
                                }
                            });
                        })
                    })
            })
        });
    }

    fn rows(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        let text_height = egui::TextStyle::Body.resolve(ui.style()).size.max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .max_scroll_height(available_height);

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Win");
                });
                header.col(|ui| {
                    ui.strong("Start Time");
                });
                header.col(|ui| {
                    ui.strong("Duration");
                });
                header.col(|ui| {
                    ui.strong("Game Mode");
                });
                header.col(|ui| {
                    ui.strong("Player Detail");
                });
            })
            .body(|body| {
                body.rows(text_height, 1, |mut row| {
                    let row_index = row.index();
                    row.col(|ui| {
                        ui.label(self.matches[row_index].win_col());
                    });

                    row.col(|ui| {
                        ui.label(self.matches[row_index].start_time_col());
                    });

                    row.col(|ui| {
                        ui.label(self.matches[row_index].duration_col());
                    });

                    row.col(|ui| {
                        ui.label(self.matches[row_index].game_mode_col());
                    });

                    row.col(|ui| {
                        ui.label(self.matches[row_index].player_detail_col());
                    });
                });
            });
    }
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
