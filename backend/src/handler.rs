use std::sync::Arc;

use axum::Router;
use sea_orm::DatabaseConnection;
use tower_http::services::ServeDir;

use crate::config::CONFIG;

mod api;

#[derive(Debug, Clone)]
struct AppState {
    http_client: reqwest::Client,
    db: Arc<DatabaseConnection>,
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
        .nest_service("/", ServeDir::new(&CONFIG.static_files_directory_path))
}
