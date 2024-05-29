use axum::Json;
use axum::{extract::Query, response::IntoResponse};
use or_status_code::OrInternalServerError;
use serde::{Deserialize, Serialize};

use crate::host::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::host::repository::users;
use crate::host::repository::users::UserRepository;

use super::ApiResult;

#[derive(Deserialize)]
pub struct ListUsersQuery {
    #[serde(rename = "r")]
    #[serde(default = "default_ranking")]
    pub ranking: Ranking,
    #[serde(rename = "p")]
    #[serde(default = "default_period")]
    pub period: Period,
    #[serde(rename = "l")]
    #[serde(default = "default_limit")]
    pub limit: u8,
}

#[derive(Deserialize)]
pub enum Ranking {
    #[serde(rename = "p")]
    Popularity,
}

impl Into<users::Ranking> for Ranking {
    fn into(self) -> users::Ranking {
        match self {
            Ranking::Popularity => users::Ranking::ViewCount,
        }
    }
}

#[derive(Deserialize)]
pub enum Period {
    #[serde(rename = "a")]
    All,
}

impl Into<users::Period> for Period {
    fn into(self) -> users::Period {
        match self {
            Period::All => users::Period::All,
        }
    }
}

fn default_ranking() -> Ranking {
    Ranking::Popularity
}

fn default_period() -> Period {
    Period::All
}

fn default_limit() -> u8 {
    10
}

#[derive(Serialize)]
pub struct ListUsersResponse {
    #[serde(rename = "u")]
    users: Vec<UserResponse>,
}

#[derive(Serialize)]
pub struct UserResponse {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "u")]
    username: String,
}

pub async fn list_users(
    user_repository: UserRepositoryExtractor,
    Query(query): Query<ListUsersQuery>
) -> ApiResult<impl IntoResponse> {
    let users = user_repository
        .list(query.ranking, query.period, query.limit as i32)
        .await
        .or_internal_server_error()?;

    let response_body = ListUsersResponse {
        users: users
            .into_iter()
            .map(|user| UserResponse {
                id: user.id,
                username: user.username,
            })
            .collect(),
    };

    Ok(Json(response_body))
}