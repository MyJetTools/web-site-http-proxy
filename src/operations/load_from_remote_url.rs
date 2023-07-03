use std::sync::Arc;

use my_http_server::*;

use crate::app::AppContext;

pub async fn load_from_remote_url(
    app: &Arc<AppContext>,
    path: &str,
) -> Result<HttpOkResult, HttpFailResult> {
    let remote_url = if path == "/" {
        build_file_path(&app.settings.base_url, &app.settings.root_page)
    } else {
        build_file_path(&app.settings.base_url, path)
    };

    match flurl::FlUrl::new(remote_url.as_str()).get().await {
        Ok(result) => {
            if result.get_status_code() >= 300 {
                println!("Error: {} -> {}", remote_url, result.get_status_code());
                return Err(HttpFailResult::as_not_found("Not Found".to_string(), false));
            }
            let content_type = WebContentType::detect_by_extension(path);
            let content = result.receive_body().await.unwrap();
            app.cached_pages
                .update(path, content_type, content.clone())
                .await;

            return Ok(HttpOkResult {
                write_telemetry: false,
                output: HttpOutput::Content {
                    headers: None,
                    content_type: WebContentType::detect_by_extension(path),
                    content,
                },
            });
        }
        Err(err) => {
            println!("Error: {} -> {:?}", remote_url, err);

            let err = HttpFailResult {
                status_code: 503,
                content: "Service Unavailable".as_bytes().to_vec(),
                content_type: WebContentType::Text,
                write_telemetry: false,
                write_to_log: true,
            };
            return Err(err);
        }
    }
}

fn build_file_path(root_url: &str, path_and_query: &str) -> String {
    if path_and_query.starts_with('/') {
        if root_url.ends_with("/") {
            return format!("{}{}", &root_url[..root_url.len() - 1], path_and_query);
        }

        return format!("{}{}", root_url, path_and_query);
    } else {
        if root_url.ends_with("/") {
            return format!("{}{}", root_url, path_and_query);
        }

        return format!("{}/{}", root_url, path_and_query);
    }
}
