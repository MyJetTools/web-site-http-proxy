use std::sync::Arc;

use rust_extensions::{date_time::DateTimeAsMicroseconds, MyTimerTick};

use crate::{app::AppContext, operations};

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
        let started = DateTimeAsMicroseconds::now();
        while let Some(path) = self.app.cached_pages.get_expired_cache_item().await {
            let now = DateTimeAsMicroseconds::now();

            if now.duration_since(started).as_positive_or_zero().as_secs() > 30 {
                break;
            }

            match operations::load_from_remote_url(&self.app, path.as_str()).await {
                Ok(_) => {}
                Err(_) => {
                    self.app.cached_pages.remove(&path).await;
                }
            }
        }
    }
}
