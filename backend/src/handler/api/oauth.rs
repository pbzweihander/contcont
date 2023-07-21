use async_trait::async_trait;
use axum::{
    extract::{self, rejection::TypedHeaderRejectionReason, FromRequestParts},
    headers,
    http::{header, request::Parts, HeaderMap, StatusCode},
    response::Redirect,
    routing, Json, RequestPartsExt, Router, TypedHeader,
};
use jsonwebtoken::Validation;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use url::Url;

use crate::{config::CONFIG, entity::instance, handler::AppState, utils::detect_instance};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    handle: String,
    instance: String,
    exp: i64,
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => {
                        (StatusCode::UNAUTHORIZED, "user not authorized")
                    }
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "failed to authorize"),
                },
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "failed to authorize"),
            })?;
        let session_cookie = cookies
            .get("SESSION")
            .ok_or((StatusCode::UNAUTHORIZED, "user not authorized"))?;

        let mut jwt_validation = Validation::default();
        jwt_validation.validate_exp = true;
        let user_data =
            jsonwebtoken::decode::<User>(session_cookie, &CONFIG.jwt_secret.1, &jwt_validation)
                .map_err(|_| (StatusCode::UNAUTHORIZED, "user not authorized"))?;
        let user = user_data.claims;

        Ok(user)
    }
}

pub(super) fn create_router() -> Router<AppState> {
    Router::new()
        .route("/authorize", routing::post(post_authorize))
        .route("/redirect", routing::get(get_redirect))
}

#[derive(Debug, Clone, Deserialize)]
struct PostAuthorizeReq {
    instance: String,
}

#[derive(Debug, Clone, Serialize)]
struct PostAuthorizeResp {
    url: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MisskeyAppCreateReq {
    name: String,
    description: String,
    permission: Vec<String>,
    callback_url: Url,
}

#[derive(Debug, Clone, Deserialize)]
struct MisskeyAppCreateResp {
    id: String,
    secret: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MisskeySessionGenerateReq {
    app_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
struct MisskeySessionGenerateResp {
    url: String,
}

async fn post_authorize(
    extract::State(state): extract::State<AppState>,
    Json(req): Json<PostAuthorizeReq>,
) -> Result<(HeaderMap, Json<PostAuthorizeResp>), (StatusCode, &'static str)> {
    let instance_url = Url::parse(&format!("https://{}", req.instance)).map_err(|err| {
        tracing::error!(%err, "failed to parse instance URL");
        (StatusCode::BAD_REQUEST, "failed to parse instance URL")
    })?;
    let instance_type = detect_instance(instance_url)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to detect instance type");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to detect instance type",
            )
        })?
        .ok_or((StatusCode::BAD_REQUEST, "failed to detect instance type"))?;

    let redirect_url = CONFIG.base_url.join("/api/oauth/redirect").map_err(|err| {
        tracing::error!(%err, "failed to generate redirect URL");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to generate redirect URL",
        )
    })?;

    let instance = instance::Entity::find_by_id(&req.instance)
        .one(&*state.db)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to query database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database",
            )
        })?;
    if instance_type == "misskey" || instance_type == "cherrypick" || instance_type == "castella" {
        let instance = if let Some(instance) = instance {
            instance
        } else {
            let resp = state
                .http_client
                .post(format!("https://{}/api/app/create", req.instance))
                .json(&MisskeyAppCreateReq {
                    name: env!("CARGO_PKG_NAME").to_string(),
                    description: "contest controller".to_string(),
                    permission: Vec::new(),
                    callback_url: redirect_url,
                })
                .send()
                .await
                .map_err(|err| {
                    tracing::error!(%err, "failed to request to Misskey instance");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to request to Misskey instance",
                    )
                })?
                .json::<MisskeyAppCreateResp>()
                .await
                .map_err(|err| {
                    tracing::error!(%err, "failed to parse Misskey response");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to parse Misskey response",
                    )
                })?;

            let instance_activemodel = instance::ActiveModel {
                hostname: ActiveValue::Set(req.instance.clone()),
                client_id: ActiveValue::Set(resp.id),
                client_secret: ActiveValue::Set(resp.secret),
            };
            let instance = instance_activemodel
                .insert(&*state.db)
                .await
                .map_err(|err| {
                    tracing::error!(%err, "failed to insert to database");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to insert to database",
                    )
                })?;

            instance
        };

        let resp = state
            .http_client
            .post(format!(
                "https://{}/api/auth/session/generate",
                req.instance
            ))
            .json(&MisskeySessionGenerateReq {
                app_secret: instance.client_secret,
            })
            .send()
            .await
            .map_err(|err| {
                tracing::error!(%err, "failed to request to Misskey instance");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to request to Misskey instance",
                )
            })?
            .json::<MisskeySessionGenerateResp>()
            .await
            .map_err(|err| {
                tracing::error!(%err, "failed to parse Misskey response");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to parse Misskey response",
                )
            })?;

        let mut header_map = HeaderMap::new();
        header_map.insert(
            header::SET_COOKIE,
            format!(
                "LOGIN_SESSION=misskey_{}; SameSite=Lax; Path=/",
                instance.client_id
            )
            .parse()
            .map_err(|err| {
                tracing::error!(%err, "failed to generate session cookie value");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to generate session cookie value",
                )
            })?,
        );

        Ok((header_map, Json(PostAuthorizeResp { url: resp.url })))
    } else {
        // Mastodon
        if let Some(_instance) = instance {
            todo!()
        } else {
            todo!()
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum GetRedirectQuery {
    Misskey { token: String },
    Mastodon { state: String },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MisskeyUserkeyReq {
    app_secret: String,
    token: String,
}

#[derive(Debug, Clone, Deserialize)]
struct MisskeyUser {
    username: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MisskeyUserkeyResp {
    user: MisskeyUser,
}

async fn get_redirect(
    extract::Query(query): extract::Query<GetRedirectQuery>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
    extract::State(state): extract::State<AppState>,
) -> Result<(HeaderMap, Redirect), (StatusCode, &'static str)> {
    let session = cookies
        .get("LOGIN_SESSION")
        .ok_or((StatusCode::BAD_REQUEST, "session not found"))?;

    let mut user = if let Some(client_id) = session.strip_prefix("misskey_") {
        let token = if let GetRedirectQuery::Misskey { token } = query {
            token
        } else {
            return Err((StatusCode::BAD_REQUEST, "token not found"));
        };

        let instance = instance::Entity::find()
            .filter(instance::Column::ClientId.eq(client_id))
            .one(&*state.db)
            .await
            .map_err(|err| {
                tracing::error!(%err, "failed to query database");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to query database",
                )
            })?
            .ok_or((StatusCode::NOT_FOUND, "instance not found"))?;

        let resp = state
            .http_client
            .post(format!(
                "https://{}/api/auth/session/userkey",
                instance.hostname
            ))
            .json(&MisskeyUserkeyReq {
                app_secret: instance.client_secret,
                token,
            })
            .send()
            .await
            .map_err(|err| {
                tracing::error!(%err, "failed to request to Misskey instance");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to request to Misskey instance",
                )
            })?
            .json::<MisskeyUserkeyResp>()
            .await
            .map_err(|err| {
                tracing::error!(%err, "failed to parse Misskey response");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to parse Misskey response",
                )
            })?;

        User {
            handle: resp.user.username,
            instance: instance.hostname,
            exp: 0,
        }
    } else {
        // Mastodon
        let _state = if let GetRedirectQuery::Mastodon { state } = query {
            state
        } else {
            return Err((StatusCode::BAD_REQUEST, "state not found"));
        };

        todo!()
    };

    let now = OffsetDateTime::now_utc();
    let exp = (now + Duration::days(1)).unix_timestamp();

    user.exp = exp;

    let session_token = jsonwebtoken::encode(&Default::default(), &user, &CONFIG.jwt_secret.0)
        .map_err(|err| {
            tracing::error!(%err, "failed to generate JWT token");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to generate JWT token",
            )
        })?;

    let cookie = format!("SESSION={}; SameSite=Lax; Path=/", session_token);

    let mut header_map = HeaderMap::new();
    header_map.insert(
        header::SET_COOKIE,
        cookie.parse().map_err(|err| {
            tracing::error!(%err, "failed to generate session cookie value");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to generate session cookie value",
            )
        })?,
    );

    Ok((header_map, Redirect::to("/")))
}
