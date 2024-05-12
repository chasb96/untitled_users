use prost::Message;

#[derive(Message)]
pub struct ProjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: String,
}