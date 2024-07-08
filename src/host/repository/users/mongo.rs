use mongodb::bson::doc;
use futures::TryStreamExt;

use crate::host::repository::{error::QueryError, mongo::MongoDatabase};

use super::{NewUser, User, UserRepository};

impl UserRepository for MongoDatabase {
    async fn create<'a>(&self, user: NewUser<'a>) -> Result<(), QueryError> {
        let conn  =  self.connection_pool
            .get()
            .await?;

        conn.collection::<_>("users")
            .insert_one(doc! {
                "id": user.id,
                "username": user.username
            })
            .await?;

        Ok(())
    }

    async fn list(&self, user_ids: Option<Vec<i32>>) -> Result<Vec<User>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        let mut cursor = conn.collection("users")
            .find(doc! {
                "id": { "$in": user_ids }
            })
            .await?;

        let mut users = Vec::new();

        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }

        Ok(users)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<User>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        conn.collection("users")
            .find_one(doc! {
                "id": id
            })
            .await
            .map_err(Into::into)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        conn.collection("users")
            .find_one(doc! {
                "username": username
            })
            .await
            .map_err(Into::into)
    }

    async fn add_project(&self, user_id: &str, project_id: &str, project_name: &str) -> Result<(), QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        conn.collection::<User>("users")
            .update_one(
                doc! { "id": user_id },
                doc! { "$push": { 
                        "projects": {
                            "project_id": project_id,
                            "project_name": project_name
                        }
                    }
                }
            )
            .await?;

        Ok(())
    }
}