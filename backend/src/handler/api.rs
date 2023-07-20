use axum::{routing, Router};

mod contest;

pub fn create_router() -> Router {
    let contest = contest::create_router();

    Router::new()
        .route("/healthz", routing::get(get_healthz))
        .nest("/contest", contest)
}

async fn get_healthz() -> &'static str {
    "ok"
}
