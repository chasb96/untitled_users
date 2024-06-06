use axum::response::IntoResponse;
use axum::http::StatusCode;
use json_or_protobuf::JsonOrProtobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::host::axum::extractors::message_queue::MessageQueueExtractor;
use crate::host::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::host::message_queue::UserCreated;
use crate::host::repository::users::UserRepository;

use super::ApiResult;

#[derive(Deserialize, Message)]
pub struct CreateUserRequest {
    #[serde(rename = "u")]
    #[prost(string, tag = "1")]
    pub username: String,
}

#[derive(Serialize, Message)]
pub struct CreateUserResponse {
    #[serde(rename = "uid")]
    #[prost(int32, tag = "1")]
    pub id: i32,
}

pub async fn create_user(
    user_repository: UserRepositoryExtractor,
    message_queue: MessageQueueExtractor,
    request: JsonOrProtobuf<CreateUserRequest>,
) -> ApiResult<impl IntoResponse> {
    let (request, content_type) = request.decompose();

    let existing = user_repository
        .get_by_username(&request.username)
        .await
        .or_internal_server_error()?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let id = user_repository
        .create(&request.username)
        .await
        .or_internal_server_error()?;

    message_queue
        .send(UserCreated {
            id,
            username: request.username
        })
        .await;

    Ok(JsonOrProtobuf::new(
        CreateUserResponse {
            id,
        },
        &content_type
    ).unwrap())
}