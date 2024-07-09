use prost::Message;
use redis::AsyncCommands;

use crate::repository::{error::QueryError, postgres::PostgresDatabase, redis::RedisCache};

use super::{MetricsRepository, User};

pub struct MetricsCachingRepository<T> {
    cache: RedisCache,
    repository: T,
}

#[derive(Message)]
struct Popular {
    #[prost(message, repeated, tag = "1")]
    users: Vec<User>,
}

impl<T> MetricsRepository for MetricsCachingRepository<T> 
where   
    T: MetricsRepository,
{
    async fn increment_view_count(&self, user_id: &str) -> Result<(), QueryError> {
        self.repository
            .increment_view_count(user_id)
            .await
    }

    async fn popular(&self, limit: i32) -> Result<Vec<User>, QueryError> {        
        let mut conn = self.cache
            .connection_pool
            .get()
            .await?;

        if let Some(bytes) = conn.get("popular").await? {
            let bytes: Vec<u8> = bytes;

            return Ok(
                Popular::decode(bytes.as_slice())?
                    .users
                    .into_iter()
                    .take(limit as usize)
                    .collect()
            );
        }

        let users = self.repository
            .popular(255)
            .await?;

        conn.set_ex(
                "popular", 
                Popular {
                    users: users.clone(),
                }.encode_to_vec(), 
                3600
            )
            .await?;

        return Ok(
            users
                .into_iter()
                .take(limit as usize)
                .collect()
        );
    }
}

impl Default for MetricsCachingRepository<PostgresDatabase> {
    fn default() -> Self {
        Self {
            cache: RedisCache::default(),
            repository: PostgresDatabase::default(),
        }
    }
}