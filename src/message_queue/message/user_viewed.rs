use metrics_client::{MetricsClient, ViewUserRequest};

use super::{error::HandleError, Message};

pub struct UserViewed {
    pub id: String,
}

impl UserViewed {
    pub async fn handle(self) -> Result<(), HandleError> {
        MetricsClient::default()
            .view_user(ViewUserRequest {
                user_id: self.id,
            })
            .await
            .map_err(HandleError::from)
    }
}

impl Into<Message> for UserViewed {
    fn into(self) -> Message {
        Message::UserViewed(self)
    }
}