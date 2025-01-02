pub struct DragFriend {
    friends: Vec<Friend>,
}

pub struct Friend {
    account_id: i64,
    remark: String,
    is_favorite: bool,
}

struct Location {
    col: usize,
    row: usize,
}

// impl DragFriend {
//     pub fn show(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             // Normal friends panel
//             ui.group(|ui| {
//                 ui.heading("Normal Friends");
//                 if ui.button("Add Normal Friend").clicked() {
//                     self.friends.push(Friend {
//                         account_id: 0,
//                         remark: String::new(),
//                         is_favorite: false,
//                     });
//                 }

//                 egui::ScrollArea::vertical().show(ui, |ui| {
//                     let mut to_favorite = None;
//                     for (idx, friend) in self.friends.iter_mut().enumerate() {
//                         if !friend.is_favorite {
//                             ui.horizontal(|ui| {
//                                 ui.label(format!("ID: {} - {}", friend.account_id,
// friend.remark));                                 let response = ui.button("⇒").drag_value();
//                                 if response.dragged() {
//                                     to_favorite = Some(idx);
//                                 }
//                             });
//                         }
//                     }
//                     if let Some(idx) = to_favorite {
//                         self.friends[idx].is_favorite = true;
//                     }
//                 });
//             });

//             // Favorite friends panel
//             ui.group(|ui| {
//                 ui.heading("Favorite Friends");
//                 if ui.button("Add Favorite Friend").clicked() {
//                     self.friends.push(Friend {
//                         account_id: 0,
//                         remark: String::new(),
//                         is_favorite: true,
//                     });
//                 }

//                 egui::ScrollArea::vertical().show(ui, |ui| {
//                     let mut to_normal = None;
//                     for (idx, friend) in self.friends.iter_mut().enumerate() {
//                         if friend.is_favorite {
//                             ui.horizontal(|ui| {
//                                 let response = ui.button("⇐").drag_value();
//                                 ui.label(format!("ID: {} - {}", friend.account_id,
// friend.remark));                                 if response.dragged() {
//                                     to_normal = Some(idx);
//                                 }
//                             });
//                         }
//                     }
//                     if let Some(idx) = to_normal {
//                         self.friends[idx].is_favorite = false;
//                     }
//                 });
//             });
//         });
//     }
// }
