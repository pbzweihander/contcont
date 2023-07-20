use axum::{routing, Json, Router};

use self::oauth::User;

use super::AppState;

mod contest;
mod oauth;

pub(super) fn create_router() -> Router<AppState> {
    let contest = contest::create_router();
    let oauth = oauth::create_router();

    Router::new()
        .route("/healthz", routing::get(get_healthz))
        .route("/user", routing::get(get_user))
        .nest("/contest", contest)
        .nest("/oauth", oauth)
}

async fn get_healthz() -> &'static str {
    "ok"
}

async fn get_user(user: User) -> Json<User> {
    Json(user)
}
