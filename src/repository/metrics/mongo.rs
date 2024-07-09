use mongodb::{bson::{self, doc}, options::UpdateOptions};
use futures::TryStreamExt;

use crate::repository::{error::QueryError, mongo::MongoDatabase};

use super::{MetricsRepository, User};

impl MetricsRepository for MongoDatabase {
    async fn increment_view_count(&self, user_id: &str) -> Result<(), QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        conn.collection::<User>("user_metrics")
            .update_one(
                doc! { "user_id": user_id }, 
                doc! {
                    "$inc": { "view_count": 1 }
                }
            )
            .with_options(
                UpdateOptions::builder()
                    .upsert(true)
                    .build()
            )
            .await?;

        Ok(())
    }

    async fn popular(&self, limit: i32) -> Result<Vec<User>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        let mut cursor = conn.collection::<User>("users")
            .aggregate(vec! [
                doc! {
                    "$lookup": {
                        "from": "user_metrics",
                        "localField": "id",
                        "foreignField": "user_id",
                        "as": "metrics"
                    }
                },
                doc! {
                    "$sort": {
                        "metrics.view_count": -1
                    }
                },
                doc! {
                    "$limit": limit
                },
                doc! {
                    "$project": {
                        "id": 1,
                        "username": 1,
                        "score": { "$ifNull": ["$metrics.view_count", 0] }
                    }
                }
            ])
            .await
            .map_err(QueryError::from)?;

        let mut users = Vec::new();

        while let Some(document) = cursor.try_next().await? {
            let user = bson::from_document(document)?;

            users.push(user);
        }

        Ok(users)
    }
}