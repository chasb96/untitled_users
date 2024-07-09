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
    pub id: &'a str,
    pub username: &'a str,
}

#[derive(Clone, Deserialize, Serialize, Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub username: String,
    #[prost(message, repeated, tag = "3")]
    #[serde(default)]
    pub projects: Vec<UserProject>,
}

#[derive(Clone, Deserialize, Serialize, Message)]
pub struct UserProject {
    #[prost(string, tag = "1")]
    pub project_id: String,
    #[prost(string, tag = "2")]
    pub project_name: String,
}

pub trait UserRepository {
    async fn create<'a>(&self, user: NewUser<'a>) -> Result<(), QueryError>;

    async fn list(&self, user_ids: Option<Vec<i32>>) -> Result<Vec<User>, QueryError>;

    async fn get_by_id(&self, id: &str) -> Result<Option<User>, QueryError>;

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError>;

    async fn add_project(&self, user_id: &str, project_id: &str, project_name: &str) -> Result<(), QueryError>;
}

#[allow(dead_code)]
pub enum UserRepositoryOption {
    Postgres(PostgresDatabase),
    CachedPostgres(UserCachingRepository<PostgresDatabase>),
    Mongo(MongoDatabase),
}

impl UserRepository for UserRepositoryOption {
    async fn create<'a>(&self, user: NewUser<'a>) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(user).await,
            Self::CachedPostgres(cached_pg) => cached_pg.create(user).await,
            Self::Mongo(mongo) => mongo.create(user).await,
        }
    }
    
    async fn list(&self, user_ids: Option<Vec<i32>>) -> Result<Vec<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.list(user_ids).await,
            Self::CachedPostgres(cached_pg) => cached_pg.list(user_ids).await,
            Self::Mongo(mongo) => mongo.list(user_ids).await,
        }
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_id(id).await,
            Self::CachedPostgres(cached_pg) => cached_pg.get_by_id(id).await,
            Self::Mongo(mongo) => mongo.get_by_id(id).await,
        }
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_username(username).await,
            Self::CachedPostgres(cached_pg) => cached_pg.get_by_username(username).await,
            Self::Mongo(mongo) => mongo.get_by_username(username).await,
        }
    }
    
    async fn add_project(&self, user_id: &str, project_id: &str, project_name: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.add_project(user_id, project_id, project_name).await,
            Self::CachedPostgres(cached_pg) => cached_pg.add_project(user_id, project_id, project_name).await,
            Self::Mongo(mongo) => mongo.add_project(user_id, project_id, project_name).await,
        }
    }
}

impl Default for UserRepositoryOption {
    fn default() -> Self {
        Self::Mongo(Default::default())
    }
}