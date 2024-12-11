use common::setting::global_setting;

pub struct Courier {
    client: reqwest::Client,
    key: String,
}

impl Courier {
    pub fn from_setting() -> Self {
        let setting = global_setting();
        let client = reqwest::Client::new();
        Self {
            client,
            key: setting.steam_api_key.clone(),
        }
    }
}

#[cfg(test)]
mod tests {}
