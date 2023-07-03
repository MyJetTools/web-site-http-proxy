use std::sync::Arc;

use rust_extensions::MyTimerTick;

use crate::app::AppContext;

pub struct UpdateCacheTimer {
    app: Arc<AppContext>,
}

impl UpdateCacheTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for UpdateCacheTimer {
    async fn tick(&self) {
        while let Some(path) = self.app.cached_pages.get_expired_cache_item().await {
            self.app.cached_pages.remove(&path).await;
        }
    }
}
