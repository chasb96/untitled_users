use axum::response::IntoResponse;
use axum::http::StatusCode;
use json_or_protobuf::JsonOrProtobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

use crate::host::axum::extractors::message_queue::MessageQueueExtractor;
use crate::host::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::host::message_queue::UserCreated;
use crate::host::repository::users::{NewUser, UserRepository, USERS_ID_LENGTH};

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
    #[prost(string, tag = "1")]
    pub id: String,
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

    let user_id = Alphanumeric.sample_string(&mut rand::thread_rng(), USERS_ID_LENGTH);

    user_repository
        .create(NewUser {
            id: &user_id,
            username: &request.username,
        })
        .await
        .or_internal_server_error()?;

    message_queue
        .send(UserCreated {
            id: user_id.clone(),
            username: request.username
        })
        .await;

    Ok(JsonOrProtobuf::new(
        CreateUserResponse {
            id: user_id,
        },
        &content_type
    ).unwrap())
}