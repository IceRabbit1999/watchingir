use std::sync::OnceLock;

use config::Config;
use serde::Deserialize;
use tracing::info;

static SETTING: OnceLock<Setting> = OnceLock::new();

pub fn global_setting() -> &'static Setting {
    SETTING.get_or_init(Setting::from_config)
}

#[derive(Deserialize)]
pub struct Setting {
    pub steam_api_key: String,
}

impl Setting {
    fn from_config() -> Self {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "dev".to_string());
        let setting = Config::builder()
            .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(true))
            .build()
            .expect("load config error");
        info!("Loading configuration in {run_mode} mode...");
        setting.try_deserialize().expect("parse config error")
    }
}
