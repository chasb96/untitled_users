use axum::{extract::Query, http::HeaderMap, response::IntoResponse};
use json_or_protobuf::JsonOrProtobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::axum::extractors::search_repository::SearchRepositoryExtractor;
use crate::repository::search::SearchRepository;

use super::ApiResult;

#[derive(Deserialize)]
pub struct SearchQuery {
    #[serde(rename = "q")]
    pub query: String,
}

#[derive(Serialize, Message)]
pub struct SearchResponse {
    #[serde(rename = "r")]
    #[prost(message, repeated, tag = "1")]
    pub records: Vec<SearchRecord>,
}

#[derive(Serialize, Message)]
pub struct SearchRecord {
    #[serde(rename = "uid")]
    #[prost(int32, tag = "1")]
    pub user_id: i32,
    #[serde(rename = "u")]
    #[prost(string, tag = "2")]
    pub username: String,
    #[serde(rename = "s")]
    #[prost(float, tag = "3")]
    pub score: f32,
}

pub async fn search_users(
    search_repository: SearchRepositoryExtractor,
    headers: HeaderMap,
    Query(search_query): Query<SearchQuery>
) -> ApiResult<impl IntoResponse> {
    let result = search_repository
        .query(search_query.query.split(' ').collect())
        .await
        .or_internal_server_error()?;

    let response = SearchResponse {
        records: result
            .into_iter()
            .map(|record| SearchRecord {
                user_id: record.user_id,
                username: record.username,
                score: record.score,
            })
            .collect(),
    };

    Ok(JsonOrProtobuf::from_accept_header(response, &headers))
}