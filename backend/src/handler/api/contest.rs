use axum::{routing, Router};

use crate::config::CONFIG;

pub fn create_router() -> Router {
    Router::new().route("/name", routing::get(get_name))
}

async fn get_name() -> String {
    CONFIG.contest_name.clone()
}
