use std::{collections::HashMap, sync::Arc};

use my_http_server::WebContentType;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::RwLock;

pub struct CachedPageContent {
    pub content_type: Option<WebContentType>,
    pub content: Vec<u8>,
    pub due_date: DateTimeAsMicroseconds,
}

pub struct CachedPages {
    pub cached_pages: RwLock<HashMap<String, Arc<CachedPageContent>>>,
    pub cached_seconds: i64,
}

impl CachedPages {
    pub fn new(cached_seconds: i64) -> Self {
        CachedPages {
            cached_pages: RwLock::new(HashMap::new()),
            cached_seconds,
        }
    }

    pub async fn get_cached_page(&self, path: &str) -> Option<Arc<CachedPageContent>> {
        let cached_pages = self.cached_pages.read().await;
        cached_pages.get(path).cloned()
    }

    pub async fn update(&self, path: &str, content_type: Option<WebContentType>, content: Vec<u8>) {
        let mut due_date = DateTimeAsMicroseconds::now();

        due_date.add_seconds(self.cached_seconds);
        let mut write = self.cached_pages.write().await;

        write.insert(
            path.to_string(),
            Arc::new(CachedPageContent {
                content,
                due_date,
                content_type,
            }),
        );
    }

    pub async fn get_expired_cache_item(&self) -> Option<String> {
        let now = DateTimeAsMicroseconds::now();

        let cached_pages = self.cached_pages.read().await;

        for (path, itm) in &*cached_pages {
            if itm.due_date.unix_microseconds < now.unix_microseconds {
                return Some(path.to_string());
            }
        }

        None
    }
    pub async fn remove(&self, path: &str) {
        let mut write = self.cached_pages.write().await;
        write.remove(path);
    }
}
