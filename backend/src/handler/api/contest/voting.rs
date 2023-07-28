use axum::{extract, http::StatusCode, routing, Json, Router};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    TransactionTrait,
};
use serde::Serialize;
use time::OffsetDateTime;

use crate::{
    config::CONFIG,
    entity::{art, art_vote, literature, literature_vote},
    handler::{api::oauth::User, AppState},
};

use super::GetOpenedResp;

pub(super) fn create_router() -> Router<AppState> {
    Router::new()
        .route("/opened", routing::get(get_opened))
        .route(
            "/literature/:id",
            routing::get(get_literature).post(post_literature),
        )
        .route("/art/:id", routing::get(get_art).post(post_art))
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetVoteResp {
    voted: bool,
    vote_count: u64,
}

async fn get_literature(
    user: User,
    extract::State(state): extract::State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<Json<GetVoteResp>, (StatusCode, &'static str)> {
    if !CONFIG.literature_enabled {
        return Err((StatusCode::BAD_REQUEST, "literature not enabled"));
    }

    let tx = state.db.begin().await.map_err(|err| {
        tracing::error!(%err, "failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to begin transaction",
        )
    })?;

    let voted = literature_vote::Entity::find()
        .filter(
            literature_vote::Column::Handle
                .eq(&user.handle)
                .and(literature_vote::Column::Instance.eq(&user.instance))
                .and(literature_vote::Column::LiteratureId.eq(id)),
        )
        .count(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;

    let vote_count = literature_vote::Entity::find()
        .filter(
            literature_vote::Column::Handle
                .eq(&user.handle)
                .and(literature_vote::Column::Instance.eq(&user.instance)),
        )
        .count(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;

    Ok(Json(GetVoteResp {
        voted: voted > 0,
        vote_count,
    }))
}

async fn post_literature(
    user: User,
    extract::State(state): extract::State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<(), (StatusCode, &'static str)> {
    if !CONFIG.literature_enabled {
        return Err((StatusCode::BAD_REQUEST, "literature not enabled"));
    }

    let now = OffsetDateTime::now_utc();
    if now < CONFIG.voting_open_at || now > CONFIG.voting_close_at {
        return Err((StatusCode::BAD_REQUEST, "voting not available"));
    }

    let tx = state.db.begin().await.map_err(|err| {
        tracing::error!(%err, "failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to begin transaction",
        )
    })?;

    let literature = literature::Entity::find_by_id(id)
        .count(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;
    if literature == 0 {
        return Err((StatusCode::NOT_FOUND, "literature not found"));
    }

    let existing_vote = literature_vote::Entity::find()
        .filter(
            literature_vote::Column::Handle
                .eq(&user.handle)
                .and(literature_vote::Column::Instance.eq(&user.instance))
                .and(literature_vote::Column::LiteratureId.eq(id)),
        )
        .count(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;
    if existing_vote > 0 {
        return Err((StatusCode::CONFLICT, "already voted"));
    }

    let existing_vote_count = literature_vote::Entity::find()
        .filter(
            literature_vote::Column::Handle
                .eq(&user.handle)
                .and(literature_vote::Column::Instance.eq(&user.instance)),
        )
        .count(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;

    if existing_vote_count >= 5 {
        return Err((StatusCode::CONFLICT, "too many vote"));
    }

    let literature_vote_activemodel = literature_vote::ActiveModel {
        id: ActiveValue::NotSet,
        handle: ActiveValue::Set(user.handle),
        instance: ActiveValue::Set(user.instance),
        literature_id: ActiveValue::Set(id),
    };

    literature_vote_activemodel
        .insert(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to insert to database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to insert to database",
            )
        })?;

    tx.commit().await.map_err(|err| {
        tracing::error!(%err, "failed to commit to database");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to commit to database",
        )
    })?;

    Ok(())
}

async fn get_art(
    user: User,
    extract::State(state): extract::State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<Json<GetVoteResp>, (StatusCode, &'static str)> {
    if !CONFIG.art_enabled {
        return Err((StatusCode::BAD_REQUEST, "art not enabled"));
    }

    let tx = state.db.begin().await.map_err(|err| {
        tracing::error!(%err, "failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to begin transaction",
        )
    })?;

    let voted = art_vote::Entity::find()
        .filter(
            art_vote::Column::Handle
                .eq(&user.handle)
                .and(art_vote::Column::Instance.eq(&user.instance))
                .and(art_vote::Column::ArtId.eq(id)),
        )
        .count(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;

    let vote_count = art_vote::Entity::find()
        .filter(
            art_vote::Column::Handle
                .eq(&user.handle)
                .and(art_vote::Column::Instance.eq(&user.instance)),
        )
        .count(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;

    Ok(Json(GetVoteResp {
        voted: voted > 0,
        vote_count,
    }))
}

async fn post_art(
    user: User,
    extract::State(state): extract::State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<(), (StatusCode, &'static str)> {
    if !CONFIG.art_enabled {
        return Err((StatusCode::BAD_REQUEST, "art not enabled"));
    }

    let now = OffsetDateTime::now_utc();
    if now < CONFIG.voting_open_at || now > CONFIG.voting_close_at {
        return Err((StatusCode::BAD_REQUEST, "voting not available"));
    }

    let tx = state.db.begin().await.map_err(|err| {
        tracing::error!(%err, "failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to begin transaction",
        )
    })?;

    let art = art::Entity::find_by_id(id)
        .count(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;
    if art == 0 {
        return Err((StatusCode::NOT_FOUND, "art not found"));
    }

    let existing_vote = art_vote::Entity::find()
        .filter(
            art_vote::Column::Handle
                .eq(&user.handle)
                .and(art_vote::Column::Instance.eq(&user.instance))
                .and(art_vote::Column::ArtId.eq(id)),
        )
        .count(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;
    if existing_vote > 0 {
        return Err((StatusCode::CONFLICT, "already voted"));
    }

    let existing_vote_count = art_vote::Entity::find()
        .filter(
            art_vote::Column::Handle
                .eq(&user.handle)
                .and(art_vote::Column::Instance.eq(&user.instance)),
        )
        .count(&tx)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;

    if existing_vote_count >= 5 {
        return Err((StatusCode::CONFLICT, "too many vote"));
    }

    let art_vote_activemodel = art_vote::ActiveModel {
        id: ActiveValue::NotSet,
        handle: ActiveValue::Set(user.handle),
        instance: ActiveValue::Set(user.instance),
        art_id: ActiveValue::Set(id),
    };

    art_vote_activemodel.insert(&tx).await.map_err(|err| {
        tracing::error!(%err, "failed to insert to database");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to insert to database",
        )
    })?;

    tx.commit().await.map_err(|err| {
        tracing::error!(%err, "failed to commit to database");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to commit to database",
        )
    })?;

    Ok(())
}
