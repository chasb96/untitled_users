mod postgres;

use super::{error::QueryError, postgres::PostgresDatabase};

pub trait MetricsRepository {
    async fn increment_view_count(&self, user_id: i32) -> Result<(), QueryError>;
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
}

impl Default for MetricsRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}