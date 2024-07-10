mod get_user;
mod add_project;
mod list_users;
mod create_user;
mod popular;

pub use get_user::get_by_id;
pub use get_user::get_by_username;
pub use add_project::add_project;
pub use list_users::list_users;
pub use create_user::create_user;
pub use popular::popular;

use axum::http::StatusCode;

type ApiResult<T> = Result<T, StatusCode>;