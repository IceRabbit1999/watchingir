use config::Config;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use tracing::{error, info};

use crate::error::{TomlSnafu, WriteFileSnafu};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct AppState {
    pub steam_api_key: String,
    pub stratz_api_key: String,
    pub friends: Vec<i64>,
}

impl AppState {
    pub fn try_from_config() -> Option<Self> {
        let state = Config::builder()
            .add_source(config::File::with_name("config/save.toml").required(true))
            .build();
        if state.is_err() {
            return None;
        }

        let res = state.unwrap().try_deserialize::<AppState>();
        match res {
            Ok(state) => Some(state),
            Err(_) => None,
        }
    }

    fn save(&self) -> Result<(), crate::Error> {
        let toml_string = toml::to_string_pretty(&self).context(TomlSnafu)?;
        info!("Current AppState: \n{}", toml_string);
        std::fs::write("config/save.toml", toml_string).context(WriteFileSnafu {
            filename: "config/save.toml",
        })?;
        Ok(())
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        info!("Drop AppState, save current state to config/save.toml");
        if let Err(e) = self.save() {
            error!("save config error: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save() {
        let state = AppState::try_from_config();
        println!("{:?}", state);
    }
}
