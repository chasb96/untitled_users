use axum::http::HeaderMap;
use axum::{extract::Query, response::IntoResponse};
use json_or_protobuf::JsonOrProtobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::repository::users::UserRepository;

use super::ApiResult;

#[derive(Deserialize)]
pub struct ListUsersQuery {
    #[serde(rename = "uids")]
    user_ids: Vec<String>,
}

#[derive(Serialize, Message)]
pub struct ListUsersResponse {
    #[serde(rename = "u")]
    #[prost(message, repeated, tag = "1")]
    users: Vec<UserResponse>,
}

#[derive(Serialize, Message)]
pub struct UserResponse {
    #[serde(rename = "id")]
    #[prost(string, tag = "1")]
    id: String,
    #[serde(rename = "u")]
    #[prost(string, tag = "2")]
    username: String,
}

pub async fn list_users(
    user_repository: UserRepositoryExtractor,
    headers: HeaderMap,
    Query(query): Query<ListUsersQuery>
) -> ApiResult<impl IntoResponse> {
    let users = user_repository
        .list(&query.user_ids)
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

    Ok(JsonOrProtobuf::from_accept_header(response_body, &headers))
}