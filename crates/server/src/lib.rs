mod courier;

use std::sync::Arc;

use common::setting::global_setting;

use crate::courier::Courier;

pub struct AppState {
    courier: Arc<Courier>,
}

impl AppState {
    pub fn from_setting() -> Self {
        let courier = Courier::from_setting();
        Self { courier: Arc::new(courier) }
    }
}
