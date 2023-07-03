use std::{sync::Arc, time::Duration};

use rust_extensions::MyTimer;

mod app;
mod background;
mod http;
mod operations;
mod settings;

#[tokio::main]
async fn main() {
    let settings = settings::Settings::read_from_file(".web-site-http-proxy".to_string())
        .await
        .unwrap();

    let app_ctx = app::AppContext::new(settings);

    let app_ctx = Arc::new(app_ctx);
    crate::http::start_up(&app_ctx).await;

    let mut timer_update_cache = MyTimer::new(Duration::from_secs(10));

    timer_update_cache.register_timer(
        "CacheUpdate",
        Arc::new(background::UpdateCacheTimer::new(app_ctx.clone())),
    );

    timer_update_cache.start(app_ctx.app_states.clone(), my_logger::LOGGER.clone());

    app_ctx.app_states.wait_until_shutdown().await;
}
