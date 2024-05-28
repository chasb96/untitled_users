use axum::{routing::{get, post}, Router};
use super::web::{add_project, get_by_id, get_by_username};

pub trait UsersRouter {
    fn register_user_routes(self) -> Self;
}

impl UsersRouter for Router {
    fn register_user_routes(self) -> Self {
        self.route("/users/:id", get(get_by_id))
            .route("/users/@:username", get(get_by_username))
            .route("/users/:id/projects", post(add_project))
    }
}