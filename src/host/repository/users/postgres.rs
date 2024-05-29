use futures::TryStreamExt;
use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::host::repository::{error::QueryError, postgres::PostgresDatabase};
use crate::host::repository::users::UserProject;

use super::{Period, Ranking, User, UserRepository};


impl UserRepository for PostgresDatabase {
    async fn list(&self, ranking: impl Into<Ranking>, period: impl Into<Period>, limit: i32) -> Result<Vec<User>, QueryError> {
        let ranking: Ranking =  ranking.into();
        let period: Period = period.into();
        
        let query = format!(r#"
            SELECT u.id, username
            FROM users u
                LEFT JOIN user_metrics um
                    ON u.id = um.user_id
            WHERE {}
            ORDER BY {}
            LIMIT $1
        "#, period.as_where_clause(), ranking.as_ordering_clause());

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(&query)
            .bind(limit)
            .map(|row: PgRow| User {
                id: row.get("id"),
                username: row.get("username"),
                projects: Vec::new(),
            })
            .fetch_all(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<User>, QueryError> {
        const QUERY: &'static str = r#"
            SELECT username, project_id, project_name
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
                projects: match (record.get("project_id"), record.get("project_name")) {
                    (Some(project_id), Some(project_name)) => vec![UserProject {
                        project_id,
                        project_name,
                    }],
                    _ => Vec::new(),
                },
            },
            None => return Ok(None),
        };

        while let Some(record) = records.try_next().await? {
            user.projects.push(UserProject {
                project_id: record.get("project_id"),
                project_name: record.get("project_name"),
            })
        }
        
        Ok(Some(user))
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        const QUERY: &'static str = r#"
            SELECT u.id, project_id, project_name
            FROM users u 
            LEFT JOIN user_projects up ON u.id = up.user_id
            WHERE u.username = $1
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
                username: username.to_string(),
                projects: match (record.get("project_id"), record.get("project_name")) {
                    (Some(project_id), Some(project_name)) => vec![UserProject {
                        project_id,
                        project_name,
                    }],
                    _ => Vec::new(),
                },
            },
            None => return Ok(None),
        };

        while let Some(record) = records.try_next().await? {
            user.projects.push(UserProject {
                project_id: record.get("project_id"),
                project_name: record.get("project_name"),
            })
        }
        
        Ok(Some(user))
    }
    
    async fn add_project(&self, user_id: i32, project_id: &str, project_name: &str) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO user_projects (user_id, project_id, project_name)
            VALUES ($1, $2, $3)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(user_id)
            .bind(project_id)
            .bind(project_name)
            .execute(conn.as_mut())
            .await?;

        Ok(())
    }
}