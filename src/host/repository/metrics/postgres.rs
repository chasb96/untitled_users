use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::host::repository::{error::QueryError, metrics::User, postgres::PostgresDatabase};

use super::MetricsRepository;

impl MetricsRepository for PostgresDatabase {
    async fn increment_view_count(&self, user_id: &str) -> Result<(), QueryError> {
        const INCREMENT_QUERY: &'static str = r#"
            INSERT INTO user_metrics (user_id, view_count)
            VALUES ($1, 1)
            ON CONFLICT (user_id)
            DO UPDATE SET view_count = user_metrics.view_count + 1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INCREMENT_QUERY)
            .bind(user_id)
            .execute(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(())
    }
    
    async fn popular(&self, limit: i32) -> Result<Vec<User>, QueryError> {
        const POPULAR_QUERY: &'static str = r#"
            SELECT u.id, u.username, um.view_count
            FROM users u
                LEFT JOIN user_metrics um
                    ON u.id = um.user_id
            ORDER BY um.view_count DESC NULLS LAST
            LIMIT $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(POPULAR_QUERY)
            .bind(limit)
            .map(|row: PgRow| User {
                id: row.get("id"),
                username: row.get("username"),
                score: row.get::<Option<i32>, _>("view_count").unwrap_or(0),
            })
            .fetch_all(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }
}