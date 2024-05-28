use crate::host::repository::metrics::{MetricsRepository, MetricsRepositoryOption};

use super::{error::HandleError, Message};

pub struct UserViewed {
    pub id: i32,
}

impl UserViewed {
    pub async fn handle(&self) -> Result<(), HandleError> {
        MetricsRepositoryOption::default()
            .increment_view_count(self.id)
            .await
            .map_err(HandleError::from)
    }
}

impl Into<Message> for UserViewed {
    fn into(self) -> Message {
        Message::UserViewed(self)
    }
}