use error::HandleError;

use super::UserViewed;
use super::UserCreated;

mod error;
pub mod user_viewed;
pub mod user_created;

pub trait Queueable:  {
    async fn handle(self) -> Result<(), HandleError>;
}

pub enum Message {
    UserViewed(UserViewed),
    UserCreated(UserCreated),
}

impl Queueable for Message {
    async fn handle(self) -> Result<(), HandleError> {
        match self {
            Self::UserViewed(message) => message.handle().await,
            Self::UserCreated(message) => message.handle().await,
        }
    }
}