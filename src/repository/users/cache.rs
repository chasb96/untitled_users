use prost::Message;
use redis::AsyncCommands;

use crate::repository::{error::QueryError, redis::RedisCache, users::{User, UserRepository}};

use super::NewUser;

pub struct UserCachingRepository<T> {
    cache: RedisCache,
    repository: T,
}

impl<T> UserRepository for UserCachingRepository<T> 
where   
    T: UserRepository,
{
    async fn create<'a>(&self, user: NewUser<'a>) -> Result<(), QueryError> {
        self.repository
            .create(user)
            .await
    }

    async fn update(&self, user: &User) -> Result<(), QueryError> {
        let cache_key = format!("user:{}", user.user_id);

        let mut conn = self.cache
            .connection_pool
            .get()
            .await?;

        let _: () = conn.del(&cache_key).await?;

        self.repository
            .update(user)
            .await
    }

    async fn list(&self, user_ids: &Vec<String>) -> Result<Vec<User>, QueryError> {
        self.repository
            .list(user_ids)
            .await
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<User>, QueryError> {
        let cache_key = format!("user:{}", id);

        let mut conn = self.cache
            .connection_pool
            .get()
            .await?;

        if let Some(bytes) = conn.get(&cache_key).await? {
            let bytes: Vec<u8> = bytes;

            return Ok(Some(User::decode(bytes.as_slice())?))
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
            let bytes: Vec<u8> = bytes;

            return Ok(Some(User::decode(bytes.as_slice())?))
        }

        if let Some(user) = self.repository.get_by_username(username).await? {
            let _: () = conn.set(cache_key, user.encode_to_vec()).await?;

            return Ok(Some(user))
        }

        Ok(None)
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