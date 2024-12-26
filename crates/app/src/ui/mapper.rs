use std::collections::HashMap;

pub fn id2name(
    id: i32,
    map: &HashMap<i32, String>,
) -> String {
    let name = map.get(&id).cloned().unwrap_or(String::from("Unknown"));
    if &name == "Unknown" {
        return String::from(" ");
    }
    name
}
