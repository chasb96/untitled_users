mod get_user;
mod list_users;
mod create_user;
mod map_users;
mod update_user;

pub use get_user::get_by_id;
pub use get_user::get_by_username;
pub use list_users::list_users;
pub use create_user::create_user;
pub use map_users::map_users;
pub use update_user::update_user;

use axum::http::StatusCode;

type ApiResult<T> = Result<T, StatusCode>;