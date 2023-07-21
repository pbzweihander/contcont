use axum::{routing, Json, Router};
use time::OffsetDateTime;

use crate::{config::CONFIG, handler::AppState};

use super::GetOpenedResp;

pub(super) fn create_router() -> Router<AppState> {
    Router::new().route("/opened", routing::get(get_opened))
}

async fn get_opened() -> Json<GetOpenedResp> {
    let now = OffsetDateTime::now_utc();
    let opened = now >= CONFIG.voting_open_at && now <= CONFIG.voting_close_at;
    Json(GetOpenedResp {
        opened,
        open_at: CONFIG.voting_open_at,
        close_at: CONFIG.voting_close_at,
    })
}
