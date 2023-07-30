use axum::{body::Bytes, extract, http::StatusCode, routing, Json, Router};
use rand::{rngs::StdRng, seq::SliceRandom};
use rand_seeder::Seeder;
use sea_orm::{EntityTrait, QueryOrder};
use serde::Serialize;
use time::OffsetDateTime;

use crate::{
    config::CONFIG,
    entity::{art, literature},
    handler::AppState,
};

use super::oauth::User;

mod result;
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
    let result = result::create_router();
    let submission = submission::create_router();
    let voting = voting::create_router();

    Router::new()
        .route("/name", routing::get(get_name))
        .route("/enabled", routing::get(get_enabled))
        .route(
            "/literature/metadata",
            routing::get(get_literature_metadata_list),
        )
        .route("/literature/:id", routing::get(get_literature))
        .route("/art/:id", routing::get(get_art))
        .route("/art/thumbnail/:id", routing::get(get_art_thumbnail))
        .route("/art/metadata", routing::get(get_art_metadata_list))
        .route("/art/metadata/:id", routing::get(get_art_metadata))
        .nest("/result", result)
        .nest("/submission", submission)
        .nest("/voting", voting)
}

async fn get_name() -> String {
    CONFIG.contest_name.clone()
}

#[derive(Serialize)]
struct GetEnabledResp {
    literature: bool,
    art: bool,
}

async fn get_enabled() -> Json<GetEnabledResp> {
    Json(GetEnabledResp {
        literature: CONFIG.literature_enabled,
        art: CONFIG.art_enabled,
    })
}

async fn get_literature_metadata_list(
    user: Option<User>,
    extract::State(state): extract::State<AppState>,
) -> Result<Json<Vec<literature::Metadata>>, (StatusCode, &'static str)> {
    if !CONFIG.literature_enabled {
        return Err((StatusCode::BAD_REQUEST, "literature not enabled"));
    }

    let mut literatures = literature::Entity::find()
        .order_by_desc(literature::Column::Id)
        .into_partial_model::<literature::Metadata>()
        .all(&*state.db)
        .await
        .map_err(|err| {
            tracing::error!(?err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;

    if let Some(user) = user {
        let mut rng: StdRng =
            Seeder::from(&format!("{}@{}", user.handle, user.instance)).make_rng();
        literatures.shuffle(&mut rng);
    }

    Ok(Json(literatures))
}

async fn get_literature(
    extract::Path(id): extract::Path<i32>,
    extract::State(state): extract::State<AppState>,
) -> Result<Json<literature::Model>, (StatusCode, &'static str)> {
    if !CONFIG.literature_enabled {
        return Err((StatusCode::BAD_REQUEST, "literature not enabled"));
    }

    let literature = literature::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|err| {
            tracing::error!(?err, "failed to query database");
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
    if !CONFIG.art_enabled {
        return Err((StatusCode::BAD_REQUEST, "art not enabled"));
    }

    let art = art::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|err| {
            tracing::error!(?err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "art not found"))?;

    Ok(Bytes::from(art.data))
}

async fn get_art_thumbnail(
    extract::Path(id): extract::Path<i32>,
    extract::State(state): extract::State<AppState>,
) -> Result<Bytes, (StatusCode, &'static str)> {
    if !CONFIG.art_enabled {
        return Err((StatusCode::BAD_REQUEST, "art not enabled"));
    }

    let art = art::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|err| {
            tracing::error!(?err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "art not found"))?;

    Ok(Bytes::from(art.thumbnail_data))
}

async fn get_art_metadata_list(
    user: Option<User>,
    extract::State(state): extract::State<AppState>,
) -> Result<Json<Vec<art::Metadata>>, (StatusCode, &'static str)> {
    if !CONFIG.art_enabled {
        return Err((StatusCode::BAD_REQUEST, "art not enabled"));
    }

    let mut arts = art::Entity::find()
        .order_by_desc(art::Column::Id)
        .into_partial_model::<art::Metadata>()
        .all(&*state.db)
        .await
        .map_err(|err| {
            tracing::error!(?err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;

    if let Some(user) = user {
        let mut rng: StdRng =
            Seeder::from(&format!("{}@{}", user.handle, user.instance)).make_rng();
        arts.shuffle(&mut rng);
    }

    Ok(Json(arts))
}

async fn get_art_metadata(
    extract::Path(id): extract::Path<i32>,
    extract::State(state): extract::State<AppState>,
) -> Result<Json<art::Metadata>, (StatusCode, &'static str)> {
    if !CONFIG.art_enabled {
        return Err((StatusCode::BAD_REQUEST, "art not enabled"));
    }

    let art = art::Entity::find_by_id(id)
        .into_partial_model::<art::Metadata>()
        .one(&*state.db)
        .await
        .map_err(|err| {
            tracing::error!(?err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "art not found"))?;

    Ok(Json(art))
}
