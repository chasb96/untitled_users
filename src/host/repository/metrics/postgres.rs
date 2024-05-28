use crate::host::repository::{error::QueryError, postgres::PostgresDatabase};

use super::MetricsRepository;

impl MetricsRepository for PostgresDatabase {
    async fn increment_view_count(&self, user_id: i32) -> Result<(), QueryError> {
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
}