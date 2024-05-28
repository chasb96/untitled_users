use error::HandleError;

use super::UserViewed;

mod error;
pub mod user_viewed;

pub enum Message {
    UserViewed(UserViewed),
}

impl Message {
    pub async fn handle(&self) -> Result<(), HandleError> {
        match self {
            Self::UserViewed(message) => message.handle().await,
        }
    }
}