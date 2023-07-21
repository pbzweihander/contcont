use axum::{routing, Router};
use serde::Serialize;
use time::OffsetDateTime;

use crate::{config::CONFIG, handler::AppState};

mod submission;
mod voting;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetOpenedResp {
    opened: bool,
    #[serde(with = "time::serde::rfc3339")]
    open_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    close_at: OffsetDateTime,
}

pub(super) fn create_router() -> Router<AppState> {
    let submission = submission::create_router();
    let voting = voting::create_router();

    Router::new()
        .route("/name", routing::get(get_name))
        .nest("/submission", submission)
        .nest("/voting", voting)
}

async fn get_name() -> String {
    CONFIG.contest_name.clone()
}
