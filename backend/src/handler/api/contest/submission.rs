use std::io::Cursor;

use axum::{body::Bytes, extract, http::StatusCode, routing, Json, Router};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    TransactionTrait,
};
use serde::Deserialize;
use thumbnailer::{create_thumbnails, ThumbnailSize};
use time::OffsetDateTime;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    config::CONFIG,
    entity::{art, literature},
    handler::{api::oauth::User, AppState},
};

use super::{ArtMetadata, GetOpenedResp};

pub(super) fn create_router() -> Router<AppState> {
    Router::new()
        .route("/opened", routing::get(get_opened))
        .route("/literature", routing::post(post_literature))
        .route("/art", routing::post(post_art))
}

async fn get_opened() -> Json<GetOpenedResp> {
    let now = OffsetDateTime::now_utc();
    let opened = now >= CONFIG.submission_open_at && now <= CONFIG.submission_close_at;
    Json(GetOpenedResp {
        opened,
        open_at: CONFIG.submission_open_at,
        close_at: CONFIG.submission_close_at,
    })
}

#[derive(Deserialize)]
struct PostLiteratureReq {
    title: String,
    text: String,
}

async fn post_literature(
    user: User,
    extract::State(state): extract::State<AppState>,
    Json(req): Json<PostLiteratureReq>,
) -> Result<Json<literature::Model>, (StatusCode, &'static str)> {
    if req.title.graphemes(true).count() > 100 || req.text.graphemes(true).count() > 7000 {
        return Err((StatusCode::BAD_REQUEST, "too long text"));
    }

    let now = OffsetDateTime::now_utc();
    if now < CONFIG.submission_open_at || now > CONFIG.submission_close_at {
        return Err((StatusCode::BAD_REQUEST, "submission not available"));
    }

    let tx = state.db.begin().await.map_err(|err| {
        tracing::error!(%err, "failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to begin transaction",
        )
    })?;

    let existing_literature = literature::Entity::find()
        .filter(
            literature::Column::AuthorHandle
                .eq(&user.handle)
                .and(literature::Column::AuthorInstance.eq(&user.instance)),
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
    if existing_literature > 0 {
        return Err((StatusCode::CONFLICT, "already submitted user"));
    }

    let literature_activemodel = literature::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(req.title),
        text: ActiveValue::Set(req.text),
        author_handle: ActiveValue::Set(user.handle),
        author_instance: ActiveValue::Set(user.instance),
    };

    let literature = literature_activemodel.insert(&tx).await.map_err(|err| {
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

    Ok(Json(literature))
}

#[derive(Deserialize)]
struct PostArtQuery {
    title: String,
}

async fn post_art(
    user: User,
    extract::State(state): extract::State<AppState>,
    extract::Query(query): extract::Query<PostArtQuery>,
    req: Bytes,
) -> Result<Json<ArtMetadata>, (StatusCode, &'static str)> {
    if req.len() > 1000 * 1000 * 10 {
        return Err((StatusCode::BAD_REQUEST, "too big data"));
    }

    let now = OffsetDateTime::now_utc();
    if now < CONFIG.submission_open_at || now > CONFIG.submission_close_at {
        return Err((StatusCode::BAD_REQUEST, "submission not available"));
    }

    let tx = state.db.begin().await.map_err(|err| {
        tracing::error!(%err, "failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to begin transaction",
        )
    })?;

    let existing_art = art::Entity::find()
        .filter(
            art::Column::AuthorHandle
                .eq(&user.handle)
                .and(art::Column::AuthorInstance.eq(&user.instance)),
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
    if existing_art > 0 {
        return Err((StatusCode::CONFLICT, "already submitted user"));
    }

    let mut thumbnails = create_thumbnails(
        Cursor::new(req.to_vec()),
        mime::IMAGE_PNG,
        [ThumbnailSize::Medium],
    )
    .map_err(|err| {
        tracing::error!(%err, "failed to generate thumbnail");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to generate thumbnail",
        )
    })?;
    let thumbnail = thumbnails.pop().unwrap();

    let mut thumbnail_data = Cursor::new(Vec::new());
    thumbnail.write_png(&mut thumbnail_data).map_err(|err| {
        tracing::error!(%err, "failed to write thumbnail data");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to write thumbnail data",
        )
    })?;

    let art_activemodel = art::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(query.title),
        data: ActiveValue::Set(req.to_vec()),
        thumbnail_data: ActiveValue::Set(thumbnail_data.into_inner()),
        author_handle: ActiveValue::Set(user.handle),
        author_instance: ActiveValue::Set(user.instance),
    };

    let art = art_activemodel.insert(&tx).await.map_err(|err| {
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

    Ok(Json(ArtMetadata {
        id: art.id,
        title: art.title,
        author_handle: art.author_handle,
        author_instance: art.author_instance,
    }))
}
