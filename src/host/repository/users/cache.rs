use axum::body::Bytes;
use prost::Message;
use redis::AsyncCommands;

use crate::host::repository::{error::QueryError, redis::RedisCache, users::{User, UserRepository}};

pub struct UserCachingRepository<T> {
    cache: RedisCache,
    repository: T,
}

impl<T> UserRepository for UserCachingRepository<T> 
where   
    T: UserRepository,
{
    async fn create(&self, username: &str) -> Result<i32, QueryError> {
        self.repository
            .create(username)
            .await
    }

    async fn list(&self, user_ids: Option<Vec<i32>>) -> Result<Vec<User>, QueryError> {
        self.repository
            .list(user_ids)
            .await
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<User>, QueryError> {
        let cache_key = format!("user:{}", id);

        let mut conn = self.cache
            .connection_pool
            .get()
            .await?;

        if let Some(bytes) = conn.get(&cache_key).await? {
            let bytes: Vec<u8> = bytes;

            return Ok(Some(User::decode::<&[u8]>(&bytes)?))
        }

        if let Some(user) = self.repository.get_by_id(id).await? {
            let _: () = conn.set(cache_key, user.encode_to_vec()).await?;

            return Ok(Some(user))
        }

        Ok(None)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        let cache_key = format!("user:{}", username);

        let mut conn = self.cache
            .connection_pool
            .get()
            .await?;

        if let Some(bytes) = conn.get(&cache_key).await? {
            let user = User::decode::<Bytes>(bytes)?;

            return Ok(Some(user))
        }

        if let Some(user) = self.repository.get_by_username(username).await? {
            let _: () = conn.set(cache_key, user.encode_to_vec()).await?;

            return Ok(Some(user))
        }

        Ok(None)
    }

    async fn add_project(&self, user_id: i32, project_id: &str, project_name: &str) -> Result<(), QueryError> {
        self.repository
            .add_project(user_id, project_id, project_name)
            .await?;

        let user = self.repository
            .get_by_id(user_id)
            .await?;

        let user = match user {
            Some(user) => user,
            None => return Ok(()),
        };

        let mut conn = self.cache
            .connection_pool
            .get()
            .await?;

        let _: () = conn.del(format!("user:{}", user.username)).await?;
        let _: () = conn.del(format!("user:{}", user.id)).await?;

        Ok(())
    }
}

impl<T> Default for UserCachingRepository<T> 
where
    T: Default,
{
    fn default() -> Self {
        Self {
            cache: RedisCache::default(),
            repository: T::default(),
        }
    }
}