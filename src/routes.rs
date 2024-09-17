use axum::{routing::{get, post}, Router};
use super::web::{create_user, get_by_id, get_by_username, list_users};

pub trait UsersRouter {
    fn register_user_routes(self) -> Self;
}

impl UsersRouter for Router {
    fn register_user_routes(self) -> Self {
        self.route("/users", get(list_users))
            .route("/users", post(create_user))
            .route("/users/:id", get(get_by_id))
            .route("/users/@:username", get(get_by_username))
    }
}