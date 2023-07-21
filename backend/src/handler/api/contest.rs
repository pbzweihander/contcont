use axum::{body::Bytes, extract, http::StatusCode, routing, Json, Router};
use sea_orm::EntityTrait;
use serde::Serialize;
use time::OffsetDateTime;

use crate::{
    config::CONFIG,
    entity::{art, literature},
    handler::AppState,
};

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
        .route("/literature/:id", routing::get(get_literature))
        .route("/art/:id", routing::get(get_art))
        .route("/art/title/:id", routing::get(get_art_title))
        .nest("/submission", submission)
        .nest("/voting", voting)
}

async fn get_name() -> String {
    CONFIG.contest_name.clone()
}

async fn get_literature(
    extract::Path(id): extract::Path<i32>,
    extract::State(state): extract::State<AppState>,
) -> Result<Json<literature::Model>, (StatusCode, &'static str)> {
    let literature = literature::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "literature not found"))?;

    Ok(Json(literature))
}

async fn get_art(
    extract::Path(id): extract::Path<i32>,
    extract::State(state): extract::State<AppState>,
) -> Result<Bytes, (StatusCode, &'static str)> {
    let art = art::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "art not found"))?;

    Ok(Bytes::from(art.data))
}

async fn get_art_title(
    extract::Path(id): extract::Path<i32>,
    extract::State(state): extract::State<AppState>,
) -> Result<String, (StatusCode, &'static str)> {
    let art = art::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "art not found"))?;

    Ok(art.title)
}
