use serde::{Deserialize, Serialize};

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
}
             "#
            .to_owned(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ConstantResponse {
    data: ConstantData,
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
