use std::{collections::VecDeque, sync::Arc};

use common::data::{
    constant,
    matches::{MatchDetailView, PlayerDetail},
};
use eframe::egui;
use egui::{mutex::Mutex, Id, Modal};
use egui_extras::{Column, TableBuilder};
use tracing::{error, info};

use super::{mapper::id2name, Component, GameConstant};
use crate::{
    message::Task,
    state::{self, AppState},
};

const MAX_MATCHES: usize = 10;

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
        constant: &Arc<Mutex<GameConstant>>,
    ) {
        egui::SidePanel::left("current_config").show(ctx, |ui| {
            ui.heading("Watchingir");
            ui.separator();
            ui.strong("Current Steam API Key:");
            ui.text_edit_singleline(&mut state.steam_api_key);
            ui.strong("Current STATAZ API Key:");
            ui.text_edit_multiline(&mut state.stratz_api_key);
            ui.strong("Current Account ID:");
            let mut account_id_str = state.account_id.to_string();
            if ui.text_edit_singleline(&mut account_id_str).changed() {
                state.account_id = account_id_str.parse().expect("Invalid account id");
            }
            ui.add_space(30.0);
            self.menu.show_menu(ui);
        });
        if state.steam_api_key.is_empty() || state.stratz_api_key.is_empty() {
            Modal::new(Id::new("steam_api_key_modal")).show(ctx, |ui| {
                ui.set_width(300.0);
                ui.heading("Enter Your Steam API Key");
                ui.text_edit_singleline(&mut state.steam_api_key);

                ui.heading("Enter Your Stratz API Key");
                ui.text_edit_singleline(&mut state.stratz_api_key);
                ui.separator();
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Confirm").clicked() {
                            info!("Initialize steam API key to: {}", state.steam_api_key);
                            info!("Initialize stratz API key to: {}", state.stratz_api_key);
                        }
                    },
                )
            });
        }
    }
}

pub struct MainPanel {
    matches: VecDeque<MatchDetailView>,
    selected_index: Option<usize>,
    task_tx: std::sync::mpsc::Sender<Task>,
}

impl Component for MainPanel {
    fn ui(
        &mut self,
        ctx: &egui::Context,
        _state: &mut AppState,
        constant: &Arc<Mutex<GameConstant>>,
    ) {
        self.table_ui(ctx, constant);
    }
}

impl MainPanel {
    pub fn new(task_tx: std::sync::mpsc::Sender<Task>) -> Self {
        Self {
            matches: VecDeque::with_capacity(MAX_MATCHES),
            selected_index: None,
            task_tx,
        }
    }

    pub fn update_match_detail(
        &mut self,
        matches: VecDeque<MatchDetailView>,
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
        constant: &Arc<Mutex<GameConstant>>,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Latest Match Details");
                if ui.button("Update").clicked() {
                    self.trigger_update_match_detail();
                }
                ui.separator();
                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::remainder().at_least(00.0))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            egui::ScrollArea::horizontal().show(ui, |ui| {
                                if !self.matches.is_empty() {
                                    self.rows(ui);
                                }
                            });

                            ui.add_space(30.0);

                            if self.selected_index.is_some() {
                                self.player_detail(ui, constant);
                            }
                        })
                    });
            });
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
            .column(Column::remainder())
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
                        if ui.button("Click to see player detail").clicked() {
                            self.selected_index = Some(row_index);
                        }
                    });
                });
            });
    }

    fn player_detail(
        &mut self,
        ui: &mut egui::Ui,
        constant: &Arc<Mutex<GameConstant>>,
    ) {
        ui.heading("Player Detail");
        ui.separator();

        let guard = constant.lock();
        if let Some(index) = &self.selected_index {
            let player = self.matches[*index].player_detail();
            ui.group(|ui| {
                let hero_name = id2name(player.hero_id, &guard.heroes_map);
                ui.heading("Hero");
                ui.horizontal(|ui| {
                    ui.label(format!("Hero: {}", hero_name));
                    ui.label(format!("Hero Variant: {}", player.hero_variant));
                    ui.label(format!("Level: {}", player.level));
                    ui.label(format!("Last Hits: {}", player.last_hits));
                    ui.label(format!("Denies: {}", player.denies));
                });
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.heading("Items");
                ui.horizontal_wrapped(|ui| {
                    let items = [
                        player.item_0,
                        player.item_1,
                        player.item_2,
                        player.item_3,
                        player.item_4,
                        player.item_5,
                        player.backpack_0,
                        player.backpack_1,
                        player.backpack_2,
                        player.item_neutral,
                        player.aghanims_scepter,
                        player.aghanims_shard,
                        player.moonshard,
                    ];
                    for (i, item) in items.iter().enumerate() {
                        ui.label(format!("Item {}: {}", i, item));
                    }
                });
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.heading("Performance");
            });
        }
    }
}

pub struct Menu {
    latest_matches: bool,
    friends: bool,
}

impl Menu {
    fn init() -> Self {
        Self {
            latest_matches: true,
            friends: false,
        }
    }

    fn show_menu(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::SidePanel::left("menu").default_width(ui.available_width()).show_inside(ui, |ui| {
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
