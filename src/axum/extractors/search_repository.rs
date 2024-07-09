use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};
use crate::repository::search::SearchRepositoryOption;

pub struct SearchRepositoryExtractor(SearchRepositoryOption);

impl Deref for SearchRepositoryExtractor {
    type Target = SearchRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for SearchRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for SearchRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(SearchRepositoryExtractor::default())
    }
}