use serde::Serialize;

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: i32,
    pub username: String,
    pub projects: Vec<UserProjectResponse>,
}

#[derive(Serialize)]
pub struct UserProjectResponse {
    pub project_id: String,
    pub project_name: String,
}