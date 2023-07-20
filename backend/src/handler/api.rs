use axum::{routing, Router};

pub fn create_router() -> Router {
    Router::new().route("/healthz", routing::get(get_healthz))
}

async fn get_healthz() -> &'static str {
    "ok"
}
