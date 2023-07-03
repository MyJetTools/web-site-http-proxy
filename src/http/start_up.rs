use std::{net::SocketAddr, sync::Arc};

use my_http_server::MyHttpServer;

use crate::app::AppContext;

use super::proxy_middleware;

pub async fn start_up(app: &Arc<AppContext>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8000)));

    let proxy_middleware = proxy_middleware::ProxyMiddleware::new(app.clone());

    http_server.add_middleware(Arc::new(proxy_middleware));
    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());
}
