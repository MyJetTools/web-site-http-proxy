use std::sync::Arc;

use my_http_server::*;

use crate::app::AppContext;

pub struct ProxyMiddleware {
    app: Arc<AppContext>,
}

impl ProxyMiddleware {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }

    async fn handle_get_request(&self, ctx: &HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        let path = ctx.request.get_path();

        if let Some(content_from_cache) = self.app.cached_pages.get_cached_page(path).await {
            return Ok(HttpOkResult {
                write_telemetry: false,
                output: HttpOutput::Content {
                    headers: None,
                    content_type: content_from_cache.content_type.clone(),
                    content: content_from_cache.content.clone(),
                },
            });
        }

        crate::operations::load_from_remote_url(&self.app, path).await
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for ProxyMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let method = ctx.request.get_method();

        if method == "GET" {
            return self.handle_get_request(ctx).await;
        }

        get_next.next(ctx).await
    }
}
