use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts};
use axum::http::StatusCode;
use axum::http::request::Parts;

use crate::host::metrics::MessageQueueProducer;

pub struct MetricsQueueExtractor(MessageQueueProducer);

impl Deref for MetricsQueueExtractor {
    type Target = MessageQueueProducer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MetricsQueueExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for MetricsQueueExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(MetricsQueueExtractor::default())
    }
}