use axum::response::IntoResponse;
use axum_extra::extract::Query;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use serde::Deserialize;

use crate::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::repository::users::UserRepository;

use super::ApiResult;

#[derive(Deserialize)]
pub struct ListUsersQuery {
    #[serde(rename = "uids")]
    user_ids: Option<Vec<String>>,
}

#[derive(Message)]
pub struct ListUsersResponse {
    #[prost(message, repeated, tag = "1")]
    users: Vec<UserResponse>,
}

#[derive(Message)]
pub struct UserResponse {
    #[prost(string, tag = "1")]
    id: String,
    #[prost(string, tag = "2")]
    username: String,
}

pub async fn list_users(
    user_repository: UserRepositoryExtractor,
    Query(query): Query<ListUsersQuery>
) -> ApiResult<impl IntoResponse> {
    let users = user_repository
        .list(&query.user_ids.unwrap_or(Vec::new()))
        .await
        .or_internal_server_error()?;

    let response_body = ListUsersResponse {
        users: users
            .into_iter()
            .map(|user| UserResponse {
                id: user.user_id,
                username: user.username,
            })
            .collect(),
    };

    Ok(Protobuf(response_body))
}