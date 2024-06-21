use axum::Json;
use axum::{extract::Query, response::IntoResponse};
use or_status_code::OrInternalServerError;
use serde::{Deserialize, Serialize};

use crate::host::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::host::repository::users::UserRepository;

use super::ApiResult;

#[derive(Deserialize)]
pub struct ListUsersQuery {
    #[serde(rename = "uids")]
    user_ids: Option<Vec<i32>>,
}

#[derive(Serialize)]
pub struct ListUsersResponse {
    #[serde(rename = "u")]
    users: Vec<UserResponse>,
}

#[derive(Serialize)]
pub struct UserResponse {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "u")]
    username: String,
}

pub async fn list_users(
    user_repository: UserRepositoryExtractor,
    Query(query): Query<ListUsersQuery>
) -> ApiResult<impl IntoResponse> {
    let users = user_repository
        .list(query.user_ids)
        .await
        .or_internal_server_error()?;

    let response_body = ListUsersResponse {
        users: users
            .into_iter()
            .map(|user| UserResponse {
                id: user.id,
                username: user.username,
            })
            .collect(),
    };

    Ok(Json(response_body))
}