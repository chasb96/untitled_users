use axum::layers::LogLayer;
use ::axum::{routing::get, Router};
use routes::UsersRouter;

mod util;
mod axum;
mod repository;
mod web;
mod configuration;
mod health;
mod routes;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .register_user_routes()
        .layer(LogLayer::new())
}