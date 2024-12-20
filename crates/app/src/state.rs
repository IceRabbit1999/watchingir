use std::io::Write;

use config::Config;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use tracing::{error, info};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct AppState {
    pub steam_api_key: String,
    pub stratz_api_key: String,
    pub account_id: i64,
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

    fn save(&self) -> Result<(), common::Error> {
        let toml_string = toml::to_string(&self).whatever_context("Can't transform AppState to toml string")?;
        info!("Current AppState: \n{}", toml_string);
        let mut file = std::fs::File::create("config/save.toml").whatever_context("Create file: config/save.toml failed")?;
        file.write_all(toml_string.as_bytes())
            .whatever_context("Write AppState to config/save.toml failed")?;
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
