use prost::Message;
use crate::axum::extractors::user_repository::UserRepositoryExtractor;
use axum_extra::protobuf::Protobuf;
use axum::response::IntoResponse;
use super::ApiResult;
use axum::extract::Path;
use crate::repository::users::UserRepository;
use or_status_code::OrNotFound;
use or_status_code::OrInternalServerError;
use axum::http::StatusCode;

#[derive(Message)]
pub struct UpdateUserRequest {
    #[prost(string, tag = "1")]
    pub username: String,
    #[prost(optional, string, tag = "2")]
    pub profile_picture: Option<String>,
}

pub async fn update_user(
    user_repository: UserRepositoryExtractor,
    Path(user_id): Path<String>,
    Protobuf(request): Protobuf<UpdateUserRequest>,
) -> ApiResult<impl IntoResponse> {
    let mut user = user_repository
        .get_by_id(&user_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    user.username = request.username;
    user.profile_picture = request.profile_picture;

    user_repository
        .update(&user)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::NO_CONTENT)
}