use std::collections::HashMap;
use futures::TryStreamExt;
use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::host::repository::{error::QueryError, postgres::PostgresDatabase};

use super::{SearchRecord, SearchRepository};

impl SearchRepository for PostgresDatabase {
    async fn create(&self, user_id: i32, username: &str) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO users_search (user_id, username, code)
            VALUES ($1, $2, DMETAPHONE($2))
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(user_id)
            .bind(username)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn query(&self, terms: Vec<&str>) -> Result<Vec<SearchRecord>, QueryError> {
        const SEARCH_QUERY: &'static str = r#"
            SELECT s.user_id as uid, s.username as u, levenshtein(s.username, q.value) AS d
            FROM (SELECT p as value, DMETAPHONE(p) AS code FROM UNNEST($1) as query(p)) as q
            JOIN users_search s 
            ON s.code = q.code OR levenshtein_less_equal(s.username, q.value, 2) <= 2
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let mut results = sqlx::query(SEARCH_QUERY)
            .bind(terms)
            .map(|row: PgRow| (
                row.get("uid"),
                row.get("u"),
                row.get::<i32, &str>("d"),
            ))
            .map(|row| (
                row.0,
                row.1,
                1.0 / (1.0 + row.2 as f32),
            ))
            .fetch(conn.as_mut());

        let mut rows = HashMap::new();

        while let Some((user_id, username, score)) = results.try_next().await? {
            rows.entry(user_id)
                .or_insert_with(|| SearchRecord {
                    user_id,
                    username,
                    score: 0.0,
                })
                .score += score
        }

        let mut rows: Vec<SearchRecord> = rows.into_values().collect();

        rows.sort_by(|l, r| l.score.total_cmp(&r.score));
        rows.truncate(32);

        Ok(rows)
    }
}