use axum::Json;
use axum::{extract::Query, response::IntoResponse};
use or_status_code::OrInternalServerError;
use serde::{Deserialize, Serialize};

use crate::host::axum::extractors::metrics_repository::MetricsRepositoryExtractor;
use crate::host::repository::metrics::MetricsRepository;

use super::ApiResult;

#[derive(Deserialize)]
pub struct PopularQuery {
    #[serde(rename = "l")]
    #[serde(default = "default_limit")]
    pub limit: u8,
}

fn default_limit() -> u8 {
    10
}

#[derive(Serialize)]
pub struct PopularResponse {
    #[serde(rename = "u")]
    users: Vec<UserResponse>,
}

#[derive(Serialize)]
pub struct UserResponse {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "u")]
    username: String,
    #[serde(rename = "s")]
    score: i32,
}

pub async fn popular(
    metrics_repository: MetricsRepositoryExtractor,
    Query(query): Query<PopularQuery>,
) -> ApiResult<impl IntoResponse> {
    let users = metrics_repository
        .popular(query.limit as i32)
        .await
        .or_internal_server_error()?;

    let response_body = PopularResponse {
        users: users
            .into_iter()
            .map(|user| UserResponse {
                id: user.id,
                username: user.username,
                score: user.score,
            })
            .collect(),
    };

    Ok(Json(response_body))
}