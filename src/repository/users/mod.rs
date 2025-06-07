mod postgres;
mod cache;
mod mongo;

use cache::UserCachingRepository;
use prost::Message;
use serde::{Deserialize, Serialize};

use super::{error::QueryError, mongo::MongoDatabase, postgres::PostgresDatabase};

pub const USERS_ID_LENGTH: usize = 16;

#[derive(Serialize)]
pub struct NewUser<'a> {
    pub user_id: &'a str,
    pub username: &'a str,
    pub profile_picture: Option<&'a str>,
}

#[derive(Clone, Deserialize, Serialize, Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub username: String,
    #[prost(optional, string, tag = "3")]
    pub profile_picture: Option<String>,
}

pub trait UserRepository {
    async fn create<'a>(&self, user: NewUser<'a>) -> Result<(), QueryError>;

    async fn update(&self, user: &User) -> Result<(), QueryError>;

    async fn list(&self, user_ids: &Vec<String>) -> Result<Vec<User>, QueryError>;

    async fn get_by_id(&self, id: &str) -> Result<Option<User>, QueryError>;

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError>;
}

#[allow(dead_code)]
pub enum UserRepositoryOption {
    Postgres(PostgresDatabase),
    CachedPostgres(UserCachingRepository<PostgresDatabase>),
    Mongo(MongoDatabase),
    CachedMongo(UserCachingRepository<MongoDatabase>),
}

impl UserRepository for UserRepositoryOption {
    async fn create<'a>(&self, user: NewUser<'a>) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(user).await,
            Self::CachedPostgres(cached_pg) => cached_pg.create(user).await,
            Self::Mongo(mongo) => mongo.create(user).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.create(user).await,
        }
    }

    async fn update(&self, user: &User) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.update(user).await,
            Self::CachedPostgres(cached_pg) => cached_pg.update(user).await,
            Self::Mongo(mongo) => mongo.update(user).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.update(user).await,
        }
    }
    
    async fn list(&self, user_ids: &Vec<String>) -> Result<Vec<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.list(user_ids).await,
            Self::CachedPostgres(cached_pg) => cached_pg.list(user_ids).await,
            Self::Mongo(mongo) => mongo.list(user_ids).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.list(user_ids).await,
        }
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_id(id).await,
            Self::CachedPostgres(cached_pg) => cached_pg.get_by_id(id).await,
            Self::Mongo(mongo) => mongo.get_by_id(id).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.get_by_id(id).await,
        }
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_username(username).await,
            Self::CachedPostgres(cached_pg) => cached_pg.get_by_username(username).await,
            Self::Mongo(mongo) => mongo.get_by_username(username).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.get_by_username(username).await,
        }
    }
}

impl Default for UserRepositoryOption {
    fn default() -> Self {
        Self::Mongo(Default::default())
    }
}