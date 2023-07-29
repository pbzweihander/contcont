use async_trait::async_trait;
use axum::{
    extract::{self, rejection::TypedHeaderRejectionReason, FromRequestParts},
    headers,
    http::{header, request::Parts, HeaderMap, StatusCode},
    response::Redirect,
    routing, Json, RequestPartsExt, Router, TypedHeader,
};
use jsonwebtoken::Validation;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use url::Url;

use crate::{config::CONFIG, entity::instance, handler::AppState, utils::detect_instance};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub handle: String,
    pub instance: String,
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

#[derive(Deserialize)]
struct PostAuthorizeReq {
    instance: String,
}

#[derive(Serialize)]
struct PostAuthorizeResp {
    url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MisskeyAppCreateReq {
    name: String,
    description: String,
    permission: Vec<String>,
    callback_url: Url,
}

#[derive(Deserialize)]
struct MisskeyAppCreateResp {
    id: String,
    secret: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MisskeySessionGenerateReq {
    app_secret: String,
}

#[derive(Deserialize)]
struct MisskeySessionGenerateResp {
    url: String,
}

#[derive(Serialize)]
struct MastodonPostAppReq {
    client_name: String,
    redirect_uris: Url,
    scopes: String,
    website: Url,
}

#[derive(Deserialize)]
struct MastodonPostAppResp {
    client_id: String,
    client_secret: String,
}

async fn post_authorize(
    extract::State(state): extract::State<AppState>,
    Json(req): Json<PostAuthorizeReq>,
) -> Result<(HeaderMap, Json<PostAuthorizeResp>), (StatusCode, &'static str)> {
    let (_, instance_name) = req.instance.rsplit_once('@').unwrap_or(("", &req.instance));
    let instance_url = Url::parse(&format!("https://{}", instance_name)).map_err(|err| {
        tracing::error!(?err, "failed to parse instance URL");
        (StatusCode::BAD_REQUEST, "failed to parse instance URL")
    })?;
    let instance_type = detect_instance(&state.http_client, instance_url)
        .await
        .map_err(|err| {
            tracing::error!(?err, "failed to detect instance type");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to detect instance type",
            )
        })?
        .ok_or((StatusCode::BAD_REQUEST, "failed to detect instance type"))?;

    let redirect_url = CONFIG.base_url.join("/api/oauth/redirect").map_err(|err| {
        tracing::error!(?err, "failed to generate redirect URL");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to generate redirect URL",
        )
    })?;

    let tx = state.db.begin().await.map_err(|err| {
        tracing::error!(?err, "failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to begin transaction",
        )
    })?;

    let instance = instance::Entity::find_by_id(instance_name)
        .one(&tx)
        .await
        .map_err(|err| {
            tracing::error!(?err, "failed to query database");
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
                .post(format!("https://{}/api/app/create", instance_name))
                .json(&MisskeyAppCreateReq {
                    name: format!("{}/{}", env!("CARGO_PKG_NAME"), CONFIG.contest_name),
                    description: "contest controller".to_string(),
                    permission: Vec::new(),
                    callback_url: redirect_url,
                })
                .send()
                .await
                .map_err(|err| {
                    tracing::error!(?err, "failed to request to Misskey instance");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to request to Misskey instance",
                    )
                })?
                .json::<MisskeyAppCreateResp>()
                .await
                .map_err(|err| {
                    tracing::error!(?err, "failed to parse Misskey response");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to parse Misskey response",
                    )
                })?;

            let instance_activemodel = instance::ActiveModel {
                hostname: ActiveValue::Set(instance_name.to_string()),
                client_id: ActiveValue::Set(resp.id),
                client_secret: ActiveValue::Set(resp.secret),
            };

            instance_activemodel.insert(&tx).await.map_err(|err| {
                tracing::error!(?err, "failed to insert to database");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to insert to database",
                )
            })?
        };

        tx.commit().await.map_err(|err| {
            tracing::error!(?err, "failed to commit to database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to commit to database",
            )
        })?;

        let resp = state
            .http_client
            .post(format!(
                "https://{}/api/auth/session/generate",
                instance_name
            ))
            .json(&MisskeySessionGenerateReq {
                app_secret: instance.client_secret,
            })
            .send()
            .await
            .map_err(|err| {
                tracing::error!(?err, "failed to request to Misskey instance");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to request to Misskey instance",
                )
            })?
            .json::<MisskeySessionGenerateResp>()
            .await
            .map_err(|err| {
                tracing::error!(?err, "failed to parse Misskey response");
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
                tracing::error!(?err, "failed to generate session cookie value");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to generate session cookie value",
                )
            })?,
        );

        Ok((header_map, Json(PostAuthorizeResp { url: resp.url })))
    } else {
        // Mastodon
        let instance = if let Some(instance) = instance {
            instance
        } else {
            let resp = state
                .http_client
                .post(format!("https://{}/api/v1/apps", instance_name))
                .json(&MastodonPostAppReq {
                    client_name: format!("{}/{}", env!("CARGO_PKG_NAME"), CONFIG.contest_name),
                    redirect_uris: redirect_url.clone(),
                    scopes: String::new(),
                    website: CONFIG.base_url.clone(),
                })
                .send()
                .await
                .map_err(|err| {
                    tracing::error!(?err, "failed to request to Mastodon instance");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to request to Mastodon instance",
                    )
                })?
                .json::<MastodonPostAppResp>()
                .await
                .map_err(|err| {
                    tracing::error!(?err, "failed to parse Mastodon response");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to parse Mastodon response",
                    )
                })?;

            let instance_activemodel = instance::ActiveModel {
                hostname: ActiveValue::Set(instance_name.to_string()),
                client_id: ActiveValue::Set(resp.client_id),
                client_secret: ActiveValue::Set(resp.client_secret),
            };

            instance_activemodel.insert(&tx).await.map_err(|err| {
                tracing::error!(?err, "failed to insert to database");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to insert to database",
                )
            })?
        };

        tx.commit().await.map_err(|err| {
            tracing::error!(?err, "failed to commit to database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to commit to database",
            )
        })?;

        let login_state = format!(
            "{}_{}",
            random_string::generate(64, "abcdefghijklmnopqrstuvwxyz0123456789"),
            instance.client_id
        );

        let mut url =
            Url::parse(&format!("https://{}/oauth/authorize", instance_name)).map_err(|err| {
                tracing::error!(?err, "failed to parse redirect URL");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to parse redirect URL",
                )
            })?;
        url.query_pairs_mut()
            .append_pair("client_id", &instance.client_id)
            .append_pair("scope", "")
            .append_pair("redirect_uri", redirect_url.as_str())
            .append_pair("response_type", "code")
            .append_pair("state", &login_state);

        let mut header_map = HeaderMap::new();
        header_map.insert(
            header::SET_COOKIE,
            format!("LOGIN_SESSION={}; SameSite=Lax; Path=/", login_state)
                .parse()
                .map_err(|err| {
                    tracing::error!(?err, "failed to generate session cookie value");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to generate session cookie value",
                    )
                })?,
        );

        Ok((
            header_map,
            Json(PostAuthorizeResp {
                url: url.to_string(),
            }),
        ))
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum GetRedirectQuery {
    Misskey { token: String },
    Mastodon { state: String, code: String },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MisskeyUserkeyReq {
    app_secret: String,
    token: String,
}

#[derive(Deserialize)]
struct MisskeyUser {
    username: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MisskeyUserkeyResp {
    user: MisskeyUser,
}

#[derive(Serialize)]
struct MastodonOauthTokenReq {
    grant_type: String,
    redirect_uri: Url,
    client_id: String,
    client_secret: String,
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct MastodonOauthTokenResp {
    access_token: String,
}

#[derive(Deserialize)]
struct MastodonVerifyCredentialsResp {
    username: String,
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
                tracing::error!(?err, "failed to query database");
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
                tracing::error!(?err, "failed to request to Misskey instance");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to request to Misskey instance",
                )
            })?
            .json::<MisskeyUserkeyResp>()
            .await
            .map_err(|err| {
                tracing::error!(?err, "failed to parse Misskey response");
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
        let (query_state, code) = if let GetRedirectQuery::Mastodon { state, code } = query {
            (state, code)
        } else {
            return Err((StatusCode::BAD_REQUEST, "state not found"));
        };

        if query_state != session {
            return Err((StatusCode::BAD_REQUEST, "invalid state"));
        }

        let client_id = if let Some((_, client_id)) = session.split_once('_') {
            client_id
        } else {
            return Err((StatusCode::BAD_REQUEST, "invalid state"));
        };

        let instance = instance::Entity::find()
            .filter(instance::Column::ClientId.eq(client_id))
            .one(&*state.db)
            .await
            .map_err(|err| {
                tracing::error!(?err, "failed to query database");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to query database",
                )
            })?
            .ok_or((StatusCode::NOT_FOUND, "instance not found"))?;

        let resp = state
            .http_client
            .post(format!("https://{}/oauth/token", instance.hostname))
            .json(&MastodonOauthTokenReq {
                grant_type: "authorization_code".to_string(),
                redirect_uri: CONFIG.base_url.join("/api/oauth/redirect").map_err(|err| {
                    tracing::error!(?err, "failed to generate redirect URL");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to generate redirect URL",
                    )
                })?,
                client_id: instance.client_id,
                client_secret: instance.client_secret,
                code,
                state: query_state,
            })
            .send()
            .await
            .map_err(|err| {
                tracing::error!(?err, "failed to request to Mastodon instance");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to request to Mastodon instance",
                )
            })?
            .json::<MastodonOauthTokenResp>()
            .await
            .map_err(|err| {
                tracing::error!(?err, "failed to parse Mastodon response");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to parse Mastodon response",
                )
            })?;

        let resp = state
            .http_client
            .get(format!(
                "https://{}/api/v1/accounts/verify_credentials",
                instance.hostname
            ))
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", resp.access_token),
            )
            .send()
            .await
            .map_err(|err| {
                tracing::error!(?err, "failed to request to Mastodon instance");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to request to Mastodon instance",
                )
            })?
            .json::<MastodonVerifyCredentialsResp>()
            .await
            .map_err(|err| {
                tracing::error!(?err, "failed to parse Mastodon response");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to parse Mastodon response",
                )
            })?;

        User {
            handle: resp.username,
            instance: instance.hostname,
            exp: 0,
        }
    };

    let now = OffsetDateTime::now_utc();
    let exp = (now + Duration::days(1)).unix_timestamp();

    user.exp = exp;

    let session_token = jsonwebtoken::encode(&Default::default(), &user, &CONFIG.jwt_secret.0)
        .map_err(|err| {
            tracing::error!(?err, "failed to generate JWT token");
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
            tracing::error!(?err, "failed to generate session cookie value");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to generate session cookie value",
            )
        })?,
    );

    Ok((header_map, Redirect::to("/")))
}
