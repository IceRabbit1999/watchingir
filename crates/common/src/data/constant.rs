use std::io::Write;

use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::error::SaveStateSnafu;

#[derive(Serialize)]
pub struct ConstantRequest {
    query: String,
}

impl Default for ConstantRequest {
    fn default() -> Self {
        // The GraphQL request is now only used to fetch constants like items and heroes,
        // so here hardcode the query for simplicity.
        // Will consider making some changes if more api on https://api.stratz.com/graphiql is needed
        Self {
            query: r#"
{
  constants {
    items(language: S_CHINESE) {
      id
      language {
        displayName
      }
    }
    heroes(language: S_CHINESE) {
      id
      language {
        displayName
      }
    }
  }
}"#
            .to_owned(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ConstantResponse {
    data: ConstantData,
}

impl ConstantResponse {
    pub fn save_to_file(&self) -> Result<(), crate::Error> {
        let items = self
            .data
            .constants
            .items
            .iter()
            .map(|item| {
                serde_json::json!({
                    "id": item.id,
                    "name": item.language.displayName.clone().unwrap_or(String::from("Unknown"))
                })
            })
            .collect::<Vec<_>>();

        let heroes = self
            .data
            .constants
            .heroes
            .iter()
            .map(|hero| {
                serde_json::json!({
                    "id": hero.id,
                    "name": hero.language.displayName.clone().unwrap_or(String::from("Unknown"))
                })
            })
            .collect::<Vec<_>>();

        let items_josn = serde_json::to_string_pretty(&items).boxed().context(SaveStateSnafu)?;
        let heroes_json = serde_json::to_string_pretty(&heroes).boxed().context(SaveStateSnafu)?;

        let mut items_file = std::fs::File::create("config/items.json").boxed().context(SaveStateSnafu)?;
        let mut heroes_file = std::fs::File::create("config/heroes.json").boxed().context(SaveStateSnafu)?;

        items_file.write_all(items_josn.as_bytes()).boxed().context(SaveStateSnafu)?;
        heroes_file.write_all(heroes_json.as_bytes()).boxed().context(SaveStateSnafu)?;
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct ConstantData {
    constants: Constant,
}

#[derive(Deserialize, Debug)]
struct Constant {
    items: Vec<Item>,
    heroes: Vec<Hero>,
}

#[derive(Deserialize, Debug)]
struct Item {
    id: i32,
    language: Language,
}

#[derive(Deserialize, Debug)]
struct Hero {
    id: i32,
    language: Language,
}

#[derive(Deserialize, Debug)]
struct Language {
    displayName: Option<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn query() {
        let request = super::ConstantRequest::default();
        let json = serde_json::to_string(&request).unwrap();
        println!("{}", json);
    }
}
