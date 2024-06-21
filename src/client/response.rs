use prost::Message;

#[derive(Message)]
pub struct SearchResponse {
    #[prost(message, repeated, tag = "1")]
    pub records: Vec<SearchRecord>,
}

#[derive(Message)]
pub struct SearchRecord {
    #[prost(int32, tag = "1")]
    pub user_id: i32,
    #[prost(string, tag = "2")]
    pub username: String,
    #[prost(float, tag = "3")]
    pub score: f32,
}

#[derive(Message)]
pub struct CreateUserResponse {
    #[prost(int32, tag = "1")]
    pub id: i32,
}

#[derive(Message)]
pub struct ListUsersResponse {
    #[prost(message, repeated, tag = "1")]
    users: Vec<UserResponse>,
}

#[derive(Message)]
pub struct UserResponse {
    #[prost(int32, tag = "1")]
    id: i32,
    #[prost(string, tag = "2")]
    username: String,
}