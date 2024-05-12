use futures::TryStreamExt;
use sqlx::Row;

use crate::host::repository::postgres::PostgresDatabase;

use super::error::QueryError;
use super::{User, UserRepository};


impl UserRepository for PostgresDatabase {
    async fn get_by_id(&self, id: i32) -> Result<Option<User>, QueryError> {
        const QUERY: &'static str = r#"
            SELECT username, project_id
            FROM users u 
            LEFT JOIN user_projects up ON u.id = up.user_id
            WHERE u.id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let mut records = sqlx::query(QUERY)
            .bind(id)
            .fetch(conn.as_mut());

        let mut user = match records.try_next().await? {
            Some(record) => User {
                id,
                username: record.get("username"),
                projects: match record.get("project_id") {
                    Some(project_id) => vec![project_id],
                    None => Vec::new(),
                },
            },
            None => return Ok(None),
        };

        while let Some(record) = records.try_next().await? {
            user.projects.push(record.get("project_id"))
        }
        
        Ok(Some(user))
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        const QUERY: &'static str = r#"
            SELECT id
            FROM users
            WHERE username = $1;

            SELECT project_id
            FROM user_projects up
            LEFT JOIN users u ON up.user_id = u.id
            WHERE u.username = $1;
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let mut records = sqlx::query(QUERY)
            .bind(username)
            .fetch(conn.as_mut());

        let mut user = match records.try_next().await? {
            Some(record) => User {
                id: record.get("id"),
                username: username.to_owned(),
                projects: Vec::new(),
            },
            None => return Ok(None),
        };

        while let Some(record) = records.try_next().await? {
            user.projects.push(record.get("project_id"))
        }
        
        Ok(Some(user))
    }
    
    async fn add_project(&self, user_id: i32, project_id: &str) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO user_projects (user_id, project_id)
            VALUES ($1, $2)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(user_id)
            .bind(project_id)
            .execute(conn.as_mut())
            .await?;

        Ok(())
    }
}