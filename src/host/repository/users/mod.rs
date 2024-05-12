pub mod error;
mod postgres;

use self::error::QueryError;

use super::postgres::PostgresDatabase;

pub struct User {
    pub id: i32,
    pub username: String,
    pub projects: Vec<String>,
}

pub trait UserRepository {
    async fn get_by_id(&self, id: i32) -> Result<Option<User>, QueryError>;

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError>;

    async fn add_project(&self, user_id: i32, project_id: &str) -> Result<(), QueryError>;
}

pub enum UserRepositoryOption {
    Postgres(PostgresDatabase),
}

impl UserRepository for UserRepositoryOption {
    async fn get_by_id(&self, id: i32) -> Result<Option<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_id(id).await
        }
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_username(username).await
        }
    }
    
    async fn add_project(&self, user_id: i32, project_id: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.add_project(user_id, project_id).await
        }
    }
}

impl Default for UserRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}