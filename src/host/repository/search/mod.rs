mod postgres;

use super::{error::QueryError, postgres::PostgresDatabase};

pub struct SearchRecord {
    pub user_id: i32,
    pub username: String,
    pub score: f32,
}

pub trait SearchRepository {
    async fn create(&self, user_id: i32, username: &str) -> Result<(), QueryError>;

    async fn query(&self, terms: Vec<&str>) -> Result<Vec<SearchRecord>, QueryError>;
}

pub enum SearchRepositoryOption {
    Postgres(PostgresDatabase),
}

impl SearchRepository for SearchRepositoryOption {
    async fn create(&self, user_id: i32, username: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(user_id, username).await
        }
    }

    async fn query(&self, terms: Vec<&str>) -> Result<Vec<SearchRecord>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.query(terms).await
        }
    }
}

impl Default for SearchRepositoryOption {
    fn default() -> Self {
        Self::Postgres(Default::default())
    }
}