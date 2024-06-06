use ::axum::{routing::get, Router};
use log_layer::LogLayer;
use routes::UsersRouter;

mod axum;
mod repository;
mod web;
mod configuration;
mod health;
mod routes;
mod message_queue;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .register_user_routes()
        .layer(LogLayer::new())
}