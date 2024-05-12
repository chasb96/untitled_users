mod request;
mod error;
pub mod axum;

use std::env;
use prost::Message;
use reqwest::{header::CONTENT_TYPE, Client};

pub use request::ProjectRequest;
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

    pub async fn add_project(&self, user_id: i32, request: ProjectRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/users/{}/projects", self.base_url, user_id))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?;

        Ok(())
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