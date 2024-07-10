use search_client::{CreateUserRequest, SearchClient};

use super::{error::HandleError, Message};

pub struct UserCreated {
    pub id: String,
    pub username: String,
}

impl UserCreated {
    pub async fn handle(&self) -> Result<(), HandleError> {
        SearchClient::default()
            .create_user(CreateUserRequest {
                user_id: self.id
                    .to_string()
                    .clone(),
                username: self.username
                    .to_string()
                    .clone(),
            })
            .await
            .map_err(HandleError::from)
    }
}

impl Into<Message> for UserCreated {
    fn into(self) -> Message {
        Message::UserCreated(self)
    }
}