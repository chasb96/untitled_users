use axum::extract::Path;
use axum::http::StatusCode;
use axum_extra::protobuf::Protobuf;
use or_status_code::{OrInternalServerError, OrNotFound};
use projects_client::axum::extractors::ProjectsClient;

use crate::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::repository::users::UserRepository;

use prost::Message;

#[derive(Message)]
pub struct ProjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: String,
    #[prost(string, tag = "2")]
    pub project_name: String,
}

pub async fn add_project(
    client: ProjectsClient,
    user_repository: UserRepositoryExtractor,
    Path(_user_id): Path<String>,
    Protobuf(request): Protobuf<ProjectRequest>
) -> Result<StatusCode, StatusCode> {
    let project = client
        .get_project_by_id(&request.project_id)
        .await
        .or_internal_server_error()?;

    let user = user_repository
        .get_by_id(&project.user_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if !user.projects.iter().any(|user_project| &user_project.project_id == &project.id) {
        user_repository
            .add_project(&project.user_id, &project.id, &project.name)
            .await
            .or_internal_server_error()?;
    }

    Ok(StatusCode::OK)
}