use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use rand::distributions::{Alphanumeric, DistString};

use crate::axum::extractors::message_queue::MessageQueueExtractor;
use crate::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::message_queue::UserCreated;
use crate::repository::users::{NewUser, UserRepository, USERS_ID_LENGTH};

use super::ApiResult;

#[derive(Message)]
pub struct CreateUserRequest {
    #[prost(string, tag = "1")]
    pub username: String,
}

#[derive(Message)]
pub struct CreateUserResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

pub async fn create_user(
    user_repository: UserRepositoryExtractor,
    message_queue: MessageQueueExtractor,
    Protobuf(request): Protobuf<CreateUserRequest>,
) -> ApiResult<impl IntoResponse> {
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
            user_id: &user_id,
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

    Ok(Protobuf(
        CreateUserResponse {
            id: user_id,
        }
    ))
}