use std::sync::Arc;

use axum::{
    extract::DefaultBodyLimit, http::Request, middleware::Next, response::Response, Router,
};
use sea_orm::DatabaseConnection;
use tower_http::services::{ServeDir, ServeFile};

use crate::config::CONFIG;

mod api;

#[derive(Debug, Clone)]
struct AppState {
    http_client: reqwest::Client,
    db: Arc<DatabaseConnection>,
}

async fn server_header_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut resp = next.run(req).await;
    resp.headers_mut().insert(
        "server",
        format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .parse()
            .unwrap(),
    );
    resp
}

pub fn create_router(db: DatabaseConnection) -> Router {
    let state = AppState {
        http_client: reqwest::Client::new(),
        db: Arc::new(db),
    };

    let api = api::create_router();

    Router::new()
        .nest("/api", api)
        .with_state(state)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 50))
        .nest_service(
            "/",
            ServeDir::new(&CONFIG.static_files_directory_path).fallback(ServeFile::new(
                CONFIG.static_files_directory_path.join("index.html"),
            )),
        )
        .layer(axum::middleware::from_fn(server_header_middleware))
}
