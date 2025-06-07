use axum::response::IntoResponse;
use axum_extra::extract::Query;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use serde::Deserialize;

use crate::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::repository::users::UserRepository;

use super::ApiResult;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ListUsersQuery {
    #[serde(rename = "uids")]
    user_ids: Option<Vec<String>>,
}

#[derive(Message)]
pub struct MapUsersResponse {
    #[prost(map = "string, message", tag = "1")]
    users: HashMap<String, UserResponse>,
}

#[derive(Message, PartialEq)]
pub struct UserResponse {
    #[prost(string, tag = "1")]
    username: String,
    #[prost(optional, string, tag = "2")]
    profile_picture: Option<String>,
}

pub async fn map_users(
    user_repository: UserRepositoryExtractor,
    Query(query): Query<ListUsersQuery>
) -> ApiResult<impl IntoResponse> {
    let users = user_repository
        .list(&query.user_ids.unwrap_or(Vec::new()))
        .await
        .or_internal_server_error()?;

    let response_body = MapUsersResponse {
        users: users
            .into_iter()
            .map(|user| (
                user.user_id, 
                UserResponse { 
                    username: user.username, 
                    profile_picture: user.profile_picture,
                }
            ))
            .collect(),
    };

    Ok(Protobuf(response_body))
}