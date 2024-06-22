use deadpool::managed::{Manager, RecycleResult, Metrics};
use redis::{aio::MultiplexedConnection, Client, RedisError};
use sqlx::{PgConnection, Error, Connection};

pub struct PostgresConnectionManager {
    pub connection_string: String
}

impl Manager for PostgresConnectionManager {
    type Type = PgConnection;
    type Error = Error;
    
    async fn create(&self) -> Result<PgConnection, Self::Error> {
        PgConnection::connect(&self.connection_string).await
    }
    
    async fn recycle(&self, _: &mut PgConnection, _: &Metrics) -> RecycleResult<Self::Error> {
        Ok(())
    }
}

pub struct RedisConnectionManager {
    pub client: Client
}

impl Manager for RedisConnectionManager {
    type Type = MultiplexedConnection;
    type Error = RedisError;
    
    async fn create(&self) -> Result<MultiplexedConnection, Self::Error> {
        self.client
            .get_multiplexed_async_connection()
            .await
    }
    
    async fn recycle(&self, _: &mut MultiplexedConnection, _: &Metrics) -> RecycleResult<Self::Error> {
        Ok(())
    }
}