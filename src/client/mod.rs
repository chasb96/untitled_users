mod request;
mod response;
mod error;
pub mod axum;

use std::env;
use prost::Message;
use reqwest::{header::{ACCEPT, CONTENT_TYPE}, Client};

pub use request::ProjectRequest;
pub use request::CreateUserRequest;
pub use response::CreateUserResponse;
use response::ListUsersResponse;
pub use response::SearchResponse;
pub use response::SearchRecord;
pub use error::Error;

pub struct UsersClient {
    http_client: Client,
    base_url: String,
}

impl UsersClient {
    pub fn new(http_client: Client, base_url: String) -> Self {
        Self {
            http_client,
            base_url,
        }
    }

    pub async fn list_users(&self, user_ids: Option<Vec<String>>) -> Result<ListUsersResponse, Error> {
        let response = self.http_client
            .get(format!("{}/users", self.base_url))
            .query(&[("uids", user_ids)])
            .header(ACCEPT, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(response::ListUsersResponse::decode(response)?)
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<CreateUserResponse, Error> {
        let response = self.http_client
            .post(format!("{}/users", self.base_url))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(CreateUserResponse::decode(response)?)
    }

    pub async fn add_project(&self, user_id: String, request: ProjectRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/users/{}/projects", self.base_url, user_id))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?;

        Ok(())
    }

    pub async fn search(&self, query: &str) -> Result<SearchResponse, Error> {
        let response = self.http_client
            .get(format!("{}/users/search?q={}", self.base_url, query))
            .header(ACCEPT, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(SearchResponse::decode(response)?)
    }
}

impl Default for UsersClient {
    fn default() -> Self {
        let base_url = env::var("USERS_BASE_URL")
            .unwrap_or("http://users".to_string())
            .trim_end_matches('/')
            .to_string();

        Self { 
            http_client: Default::default(),
            base_url
        }
    }
}