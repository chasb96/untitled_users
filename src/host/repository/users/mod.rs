mod postgres;
mod cache;

use cache::UserCachingRepository;
use prost::Message;

use super::{error::QueryError, postgres::PostgresDatabase};

#[derive(Clone)]
#[derive(Message)]
pub struct User {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub username: String,
    #[prost(message, repeated, tag = "3")]
    pub projects: Vec<UserProject>,
}

#[derive(Clone)]
#[derive(Message)]
pub struct UserProject {
    #[prost(string, tag = "1")]
    pub project_id: String,
    #[prost(string, tag = "2")]
    pub project_name: String,
}

pub trait UserRepository {
    async fn create(&self, username: &str) -> Result<i32, QueryError>;

    async fn list(&self, user_ids: Option<Vec<i32>>) -> Result<Vec<User>, QueryError>;

    async fn get_by_id(&self, id: i32) -> Result<Option<User>, QueryError>;

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError>;

    async fn add_project(&self, user_id: i32, project_id: &str, project_name: &str) -> Result<(), QueryError>;
}

#[allow(dead_code)]
pub enum UserRepositoryOption {
    Postgres(PostgresDatabase),
    CachedPostgres(UserCachingRepository<PostgresDatabase>)
}

impl UserRepository for UserRepositoryOption {
    async fn create(&self, username: &str) -> Result<i32, QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(username).await,
            Self::CachedPostgres(cached_pg) => cached_pg.create(username).await,
        }
    }
    
    async fn list(&self, user_ids: Option<Vec<i32>>) -> Result<Vec<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.list(user_ids).await,
            Self::CachedPostgres(cached_pg) => cached_pg.list(user_ids).await,
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_id(id).await,
            Self::CachedPostgres(cached_pg) => cached_pg.get_by_id(id).await,
        }
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_username(username).await,
            Self::CachedPostgres(cached_pg) => cached_pg.get_by_username(username).await,
        }
    }
    
    async fn add_project(&self, user_id: i32, project_id: &str, project_name: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.add_project(user_id, project_id, project_name).await,
            Self::CachedPostgres(cached_pg) => cached_pg.add_project(user_id, project_id, project_name).await,
        }
    }
}

impl Default for UserRepositoryOption {
    fn default() -> Self {
        Self::CachedPostgres(UserCachingRepository::default())
    }
}