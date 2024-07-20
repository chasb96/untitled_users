use mongodb::bson::doc;
use futures::TryStreamExt;

use crate::repository::{error::QueryError, mongo::MongoDatabase};

use super::{NewUser, User, UserRepository};

impl UserRepository for MongoDatabase {
    async fn create<'a>(&self, user: NewUser<'a>) -> Result<(), QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection("users")
            .insert_one(doc! {
                "user_id": user.id,
                "username": user.username,
            })
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn list(&self, user_ids: &Vec<String>) -> Result<Vec<User>, QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection("users")
            .find(doc! { "user_id": { "$in": user_ids } })
            .await?
            .try_collect()
            .await
            .map_err(QueryError::from)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<User>, QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection("users")
            .find_one(doc! { "user_id": id })
            .await
            .map_err(Into::into)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection("users")
            .find_one(doc! { "username": username })
            .await
            .map_err(QueryError::from)
    }

    async fn add_project(&self, user_id: &str, project_id: &str, project_name: &str) -> Result<(), QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<User>("users")
            .update_one(
                doc! { "user_id": user_id },
                doc! { "$push": { 
                        "projects": {
                            "project_id": project_id,
                            "project_name": project_name
                        }
                    }
                }
            )
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }
}