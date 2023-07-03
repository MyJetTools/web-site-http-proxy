use std::sync::Arc;

use rust_extensions::AppStates;

use crate::{http::CachedPages, settings::Settings};

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub cached_pages: CachedPages,
    pub settings: Settings,
}

impl AppContext {
    pub fn new(settings: Settings) -> Self {
        AppContext {
            app_states: Arc::new(AppStates::create_initialized()),
            cached_pages: CachedPages::new(settings.cache_seconds),
            settings,
        }
    }
}
