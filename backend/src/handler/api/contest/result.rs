use axum::{extract, http::StatusCode, routing, Json, Router};
use sea_orm::{
    sea_query::{Alias, Expr},
    ColumnTrait, EntityTrait, FromQueryResult, PartialModelTrait, QueryOrder, QuerySelect,
};
use serde::Serialize;
use time::OffsetDateTime;

use crate::{
    config::CONFIG,
    entity::{art, art_vote, literature, literature_vote},
    handler::AppState,
};

pub(super) fn create_router() -> Router<AppState> {
    Router::new()
        .route("/opened", routing::get(get_opened))
        .route("/literature", routing::get(get_literature))
        .route("/art", routing::get(get_art))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetOpenedResp {
    opened: bool,
    #[serde(with = "time::serde::rfc3339")]
    open_at: OffsetDateTime,
}

async fn get_opened() -> Json<GetOpenedResp> {
    let now = OffsetDateTime::now_utc();
    let opened = now > CONFIG.voting_close_at;
    Json(GetOpenedResp {
        opened,
        open_at: CONFIG.voting_close_at,
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WithVoteCount<M> {
    #[serde(flatten)]
    metadata: M,
    vote_count: i64,
}

impl<M> FromQueryResult for WithVoteCount<M>
where
    M: FromQueryResult,
{
    fn from_query_result(res: &sea_orm::QueryResult, pre: &str) -> Result<Self, migration::DbErr> {
        let metadata = M::from_query_result(res, pre)?;
        let vote_count = res.try_get(pre, "vote_count")?;
        Ok(Self {
            metadata,
            vote_count,
        })
    }
}

async fn get_literature(
    extract::State(state): extract::State<AppState>,
) -> Result<Json<Vec<WithVoteCount<literature::Metadata>>>, (StatusCode, &'static str)> {
    if !CONFIG.literature_enabled {
        return Err((StatusCode::BAD_REQUEST, "literature not enabled"));
    }

    let now = OffsetDateTime::now_utc();
    if now <= CONFIG.voting_close_at {
        return Err((StatusCode::BAD_REQUEST, "voting not ended"));
    }

    let literatures = literature::Metadata::select_cols(
        literature::Entity::find()
            .left_join(literature_vote::Entity)
            .select_only(),
    )
    .column_as(literature_vote::Column::Id.count(), "vote_count")
    .group_by(literature::Column::Id)
    .order_by_desc(Expr::custom_keyword(Alias::new("vote_count")))
    .order_by_asc(literature::Column::Id)
    .into_model::<WithVoteCount<literature::Metadata>>()
    .all(&*state.db)
    .await
    .map_err(|err| {
        tracing::error!(?err, "failed to query database");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to query database",
        )
    })?;

    Ok(Json(literatures))
}

async fn get_art(
    extract::State(state): extract::State<AppState>,
) -> Result<Json<Vec<WithVoteCount<art::Metadata>>>, (StatusCode, &'static str)> {
    if !CONFIG.art_enabled {
        return Err((StatusCode::BAD_REQUEST, "art not enabled"));
    }

    let now = OffsetDateTime::now_utc();
    if now <= CONFIG.voting_close_at {
        return Err((StatusCode::BAD_REQUEST, "voting not ended"));
    }

    let arts = art::Metadata::select_cols(
        art::Entity::find()
            .left_join(art_vote::Entity)
            .select_only(),
    )
    .column_as(art_vote::Column::Id.count(), "vote_count")
    .group_by(art::Column::Id)
    .order_by_desc(Expr::custom_keyword(Alias::new("vote_count")))
    .order_by_asc(art::Column::Id)
    .into_model::<WithVoteCount<art::Metadata>>()
    .all(&*state.db)
    .await
    .map_err(|err| {
        tracing::error!(?err, "failed to query database");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to query database",
        )
    })?;

    Ok(Json(arts))
}
