use axum::{routing, Router};

use crate::{config::CONFIG, handler::AppState};

pub(super) fn create_router() -> Router<AppState> {
    Router::new().route("/name", routing::get(get_name))
}

async fn get_name() -> String {
    CONFIG.contest_name.clone()
}
