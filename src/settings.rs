use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub cache_seconds: i64,
    pub base_url: String,
    pub root_page: String,
}
