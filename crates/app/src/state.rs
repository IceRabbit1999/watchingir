use std::io::Write;

use config::Config;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use tracing::{error, info};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct AppState {
    pub steam_api_key: String,
    pub account_id: i64,
}

impl AppState {
    pub fn try_from_config() -> Option<Self> {
        let state = Config::builder()
            .add_source(config::File::with_name("config/save.toml").required(true))
            .build()
            .expect("load config error");
        let res = state.try_deserialize::<AppState>();
        match res {
            Ok(state) => Some(state),
            Err(e) => {
                info!("load config error: {}", e);
                None
            }
        }
    }

    fn save(&self) -> Result<(), common::Error> {
        let toml_string = toml::to_string(&self).whatever_context("Can't transform AppState to toml string")?;
        let mut file = std::fs::File::create("config/save.toml").whatever_context("Create file: config/save.toml failed")?;
        file.write_all(toml_string.as_bytes())
            .whatever_context("Write AppState to config/save.toml failed")?;
        Ok(())
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        info!("drop AppState");
        if let Err(e) = self.save() {
            error!("save config error: {}", e);
        }
    }
}
