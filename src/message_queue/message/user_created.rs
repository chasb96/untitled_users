use crate::{axum::extractors::search_repository::SearchRepositoryExtractor, repository::search::SearchRepository};

use super::{error::HandleError, Message};

pub struct UserCreated {
    pub id: String,
    pub username: String,
}

impl UserCreated {
    pub async fn handle(&self) -> Result<(), HandleError> {
        SearchRepositoryExtractor::default()
            .create(&self.id, &self.username)
            .await
            .map_err(HandleError::from)
    }
}

impl Into<Message> for UserCreated {
    fn into(self) -> Message {
        Message::UserCreated(self)
    }
}