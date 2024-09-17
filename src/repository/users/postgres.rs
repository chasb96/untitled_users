use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::repository::{error::QueryError, postgres::PostgresDatabase};

use super::{NewUser, User, UserRepository};

impl UserRepository for PostgresDatabase {
    async fn create<'a>(&self, user: NewUser<'a>) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO users (id, username, password_hash)
            VALUES ($1, $2, '')
            RETURNING id
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let id = sqlx::query(INSERT_QUERY)
            .bind(user.user_id)
            .bind(user.username)
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await?;

        Ok(id)
    }

    async fn list(&self, user_ids: &Vec<String>) -> Result<Vec<User>, QueryError> {
        const LIST_QUERY: &'static str = r#"
            SELECT u.id, username
            FROM users u
            u.id = ANY($1)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(LIST_QUERY)
            .bind(user_ids)
            .map(|row: PgRow| User {
                user_id: row.get("id"),
                username: row.get("username"),
            })
            .fetch_all(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<User>, QueryError> {
        const QUERY: &'static str = r#"
            SELECT u.id, u.username
            FROM users u 
            WHERE u.id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(QUERY)
            .bind(id)
            .map(|row: PgRow| User {
                user_id: row.get("id"),
                username: row.get("username"),
            })
            .fetch_optional(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        const QUERY: &'static str = r#"
            SELECT u.id, u.username
            FROM users u
            WHERE u.username = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

            sqlx::query(QUERY)
            .bind(username)
            .map(|row: PgRow| User {
                user_id: row.get("id"),
                username: row.get("username"),
            })
            .fetch_optional(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }
}