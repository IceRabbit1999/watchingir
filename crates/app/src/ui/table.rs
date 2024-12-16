use std::sync::Arc;

use common::data::matches::MatchDetailResponse;
use egui_extras::{Column, TableBuilder};

pub struct MatchDetailTable {
    pub matches: Vec<MatchDetailResponse>,
}

impl MatchDetailTable {
    pub fn new() -> Self {
        Self { matches: Vec::new() }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Latest Match Details");
                if ui.button("Update").clicked() {
                    // self.update_match_detail();
                }
                ui.separator();
                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::remainder().at_least(100.0))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            egui::ScrollArea::horizontal().show(ui, |ui| {
                                self.table_ui(ui);
                            });
                        })
                    })
            })
        });
    }

    fn table_ui(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        let text_height = egui::TextStyle::Body.resolve(ui.style()).size.max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .max_scroll_height(available_height);

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Match ID");
                });
            })
            .body(|body| {
                body.rows(text_height, 10, |mut row| {
                    let row_index = row.index();

                    row.col(|ui| {
                        ui.label(row_index.to_string());
                    });

                    row.col(|ui| {
                        ui.add(egui::Label::new("Thousands of rows of even height"));
                    });
                });
            });
    }
}
