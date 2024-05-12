use axum::extract::Path;
use axum::{http::StatusCode, Json};
use axum_extra::protobuf::Protobuf;
use projects::client::axum::extractors::ProjectsClient;

use crate::host::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::host::util::or_status_code::{OrInternalServerError, OrNotFound};
use crate::host::repository::users::UserRepository;

use super::request::ProjectRequest;
use super::response::GetUserResponse;

pub async fn get_by_id(
    user_repository: UserRepositoryExtractor,
    Path(id): Path<i32>
) -> Result<Json<GetUserResponse>, StatusCode> {
    let user = user_repository
        .get_by_id(id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    Ok(Json(
        GetUserResponse {
            id: user.id,
            username: user.username,
            projects: Vec::new(),
        }
    ))
}

pub async fn get_by_username(
    user_repository: UserRepositoryExtractor,
    Path(username): Path<String>
) -> Result<Json<GetUserResponse>, StatusCode> {
    let user = user_repository
        .get_by_username(&username)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    Ok(Json(
        GetUserResponse {
            id: user.id,
            username: user.username,
            projects: Vec::new(),
        }
    ))
}

pub async fn add_project(
    client: ProjectsClient,
    user_repository: UserRepositoryExtractor,
    Path(user_id): Path<i32>,
    Protobuf(request): Protobuf<ProjectRequest>
) -> Result<StatusCode, StatusCode> {
    let project = client
        .get_project_by_id(request.project_id)
        .await
        .or_internal_server_error()?;

    if project.user_id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let user = user_repository
        .get_by_id(user_id)
        .await
        .or_internal_server_error()?;

    if let Some(user) = user {
        if !user.projects.contains(&project.id) {
            user_repository
                .add_project(project.user_id, &project.id)
                .await
                .or_internal_server_error()?;
        }

        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}