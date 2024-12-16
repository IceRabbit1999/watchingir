use std::io::Write;

use config::Config;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use tracing::{error, info};

#[derive(Deserialize, Serialize, Default)]
pub struct AppState {
    steam_api_key: String,
    account_id: String,
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
        // overwrite `config/dev.toml` with current state
        let toml_string = toml::to_string(&self).whatever_context("")?;
        println!("{}", toml_string);
        let mut file = std::fs::File::create("config/save.toml").whatever_context("")?;
        file.write_all(toml_string.as_bytes()).whatever_context("")?;
        Ok(())
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        tracing::info!("drop AppState");
        if let Err(e) = self.save() {
            error!("save config error: {}", e);
        }
    }
}
