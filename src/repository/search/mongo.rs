use mongodb::bson::doc;

use crate::repository::{error::QueryError, mongo::MongoDatabase};

use super::{SearchRecord, SearchRepository};

impl SearchRepository for MongoDatabase {
    async fn create(&self, user_id: &str, username: &str) -> Result<(), QueryError> {
        Ok(())
    }

    async fn query(&self, terms: Vec<&str>) -> Result<Vec<super::SearchRecord>, QueryError> {
        Ok(Vec::new())
    }
}