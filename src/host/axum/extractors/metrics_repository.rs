use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};
use crate::host::repository::metrics::MetricsRepositoryOption;

pub struct MetricsRepositoryExtractor(MetricsRepositoryOption);

impl Deref for MetricsRepositoryExtractor {
    type Target = MetricsRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MetricsRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for MetricsRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(MetricsRepositoryExtractor::default())
    }
}