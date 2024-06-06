use prost::Message;

#[derive(Message)]
pub struct ProjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: String,
}

#[derive(Message)]
pub struct CreateUserRequest {
    #[prost(string, tag = "1")]
    pub username: String,
}