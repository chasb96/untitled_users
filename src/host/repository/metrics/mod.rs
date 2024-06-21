mod postgres;

use super::{error::QueryError, postgres::PostgresDatabase};

pub struct User {
    pub id: i32,
    pub username: String,
    pub score: i32,
}

pub trait MetricsRepository {
    async fn increment_view_count(&self, user_id: i32) -> Result<(), QueryError>;

    async fn popular(&self, limit: i32) -> Result<Vec<User>, QueryError>;
}

pub enum MetricsRepositoryOption {
    Postgres(PostgresDatabase),
}

impl MetricsRepository for MetricsRepositoryOption {
    async fn increment_view_count(&self, user_id: i32) -> Result<(), QueryError> {
        match self {
            MetricsRepositoryOption::Postgres(postgres) => postgres.increment_view_count(user_id).await,
        }
    }
    
    async fn popular(&self, limit: i32) -> Result<Vec<User>, QueryError> {
        match self {
            MetricsRepositoryOption::Postgres(postgres) => postgres.popular(limit).await,
        }
    }
}

impl Default for MetricsRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}