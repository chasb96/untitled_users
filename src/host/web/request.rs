use prost::Message;

#[derive(Message)]
pub struct ProjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: String,
    #[prost(string, tag = "2")]
    pub project_name: String,
}