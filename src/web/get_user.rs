use auth::client::axum::extractors::{Authenticate, ClaimsUser};
use axum::extract::Path;
use axum::{http::StatusCode, Json};
use or_status_code::{OrInternalServerError, OrNotFound};
use serde::Serialize;

use crate::axum::extractors::message_queue::MessageQueueExtractor;
use crate::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::message_queue::UserViewed;
use crate::repository::users::UserRepository;

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: String,
    pub username: String,
    pub projects: Vec<UserProjectResponse>,
}

#[derive(Serialize)]
pub struct UserProjectResponse {
    pub project_id: String,
    pub project_name: String,
}

pub async fn get_by_id(
    user_repository: UserRepositoryExtractor,
    Authenticate(claims_user): Authenticate<Option<ClaimsUser>>,
    message_queue: MessageQueueExtractor,
    Path(id): Path<String>
) -> Result<Json<GetUserResponse>, StatusCode> {
    let user = user_repository
        .get_by_id(&id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if claims_user.is_none() || claims_user.unwrap().id != user.id.clone() {
        message_queue
            .send(UserViewed {
                id: user.id.clone(),
            })
            .await;
    } 

    Ok(Json(
        GetUserResponse {
            id: user.id,
            username: user.username,
            projects: user.projects
                .into_iter()
                .map(|project| UserProjectResponse {
                    project_id: project.project_id,
                    project_name: project.project_name,
                })
                .collect()
        }
    ))
}

pub async fn get_by_username(
    user_repository: UserRepositoryExtractor,
    Authenticate(claims_user): Authenticate<Option<ClaimsUser>>,
    metrics_queue: MessageQueueExtractor,
    Path(username): Path<String>
) -> Result<Json<GetUserResponse>, StatusCode> {
    let user = user_repository
        .get_by_username(&username)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if claims_user.is_none() || claims_user.unwrap().id != user.id.clone() {
        metrics_queue
            .send(UserViewed {
                id: user.id.clone(),
            })
            .await;
    } 

    Ok(Json(
        GetUserResponse {
            id: user.id,
            username: user.username,
            projects: user.projects
                .into_iter()
                .map(|project| UserProjectResponse {
                    project_id: project.project_id,
                    project_name: project.project_name,
                })
                .collect()
        }
    ))
}