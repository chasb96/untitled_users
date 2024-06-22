mod postgres;
mod cache;

use cache::MetricsCachingRepository;
use prost::Message;

use super::{error::QueryError, postgres::PostgresDatabase};

#[derive(Clone, Message)]
pub struct User {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub username: String,
    #[prost(int32, tag = "3")]
    pub score: i32,
}

pub trait MetricsRepository {
    async fn increment_view_count(&self, user_id: i32) -> Result<(), QueryError>;

    async fn popular(&self, limit: i32) -> Result<Vec<User>, QueryError>;
}

#[allow(dead_code)]
pub enum MetricsRepositoryOption {
    Postgres(PostgresDatabase),
    CachedPostgres(MetricsCachingRepository<PostgresDatabase>),
}

impl MetricsRepository for MetricsRepositoryOption {
    async fn increment_view_count(&self, user_id: i32) -> Result<(), QueryError> {
        match self {
            MetricsRepositoryOption::Postgres(postgres) => postgres.increment_view_count(user_id).await,
            MetricsRepositoryOption::CachedPostgres(cached_postgres) => cached_postgres.increment_view_count(user_id).await,
        }
    }
    
    async fn popular(&self, limit: i32) -> Result<Vec<User>, QueryError> {
        match self {
            MetricsRepositoryOption::Postgres(postgres) => postgres.popular(limit).await,
            MetricsRepositoryOption::CachedPostgres(cached_postgres) => cached_postgres.popular(limit).await,
        }
    }
}

impl Default for MetricsRepositoryOption {
    fn default() -> Self {
        Self::CachedPostgres(MetricsCachingRepository::default())
    }
}