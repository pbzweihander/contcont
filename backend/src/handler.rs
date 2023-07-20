use axum::Router;
use tower_http::services::ServeDir;

use crate::config::CONFIG;

mod api;

pub fn create_router() -> Router {
    let api = api::create_router();

    Router::new()
        .nest("/api", api)
        .nest_service("/", ServeDir::new(&CONFIG.static_files_directory))
}
