use axum::extract::Path;
use axum::http::StatusCode;
use axum_extra::protobuf::Protobuf;
use or_status_code::{OrInternalServerError, OrNotFound};
use prost::Message;

use crate::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::repository::users::UserRepository;

#[derive(Message)]
pub struct GetUserResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub username: String,
    #[prost(optional, string, tag = "3")]
    pub profile_picture: Option<String>,
}

pub async fn get_by_id(
    user_repository: UserRepositoryExtractor,
    Path(id): Path<String>
) -> Result<Protobuf<GetUserResponse>, StatusCode> {
    let user = user_repository
        .get_by_id(&id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    Ok(Protobuf(
        GetUserResponse {
            id: user.user_id,
            username: user.username,
            profile_picture: user.profile_picture,
        }
    ))
}

pub async fn get_by_username(
    user_repository: UserRepositoryExtractor,
    Path(username): Path<String>
) -> Result<Protobuf<GetUserResponse>, StatusCode> {
    let user = user_repository
        .get_by_username(&username)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    Ok(Protobuf(
        GetUserResponse {
            id: user.user_id,
            username: user.username,
            profile_picture: user.profile_picture,
        }
    ))
}