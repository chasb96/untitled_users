use axum::Router;
use axum::routing::post;
use axum::routing::get;
use super::web::list_users;
use super::web::get_by_username;
use super::web::get_by_id;
use super::web::create_user;
use super::web::map_users;
use crate::web::update_user;
use axum::routing::put;

pub trait UsersRouter {
    fn register_user_routes(self) -> Self;
}

impl UsersRouter for Router {
    fn register_user_routes(self) -> Self {
        self.route("/users", get(list_users))
            .route("/users", post(create_user))
            .route("/users/map", get(map_users))
            .route("/users/:id", get(get_by_id))
            .route("/users/@:username", get(get_by_username))
            .route("/users/:id", put(update_user))
    }
}