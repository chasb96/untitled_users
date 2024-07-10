use std::{error::Error, fmt::{self, Display}};

use crate::repository::error::QueryError;
use search_client::Error as SearchClientError;

#[derive(Debug)]
pub enum HandleError {
    QueryError(QueryError),
    SearchClientError(SearchClientError),
}

impl Error for HandleError { }

impl Display for HandleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error handling message: ")?;

        match self {
            Self::QueryError(e) => write!(f, "QueryError({})", e),
            Self::SearchClientError(e) => write!(f, "SearchClientError({})", e),
        }
    }
}

impl From<QueryError> for HandleError {
    fn from(value: QueryError) -> Self {
        HandleError::QueryError(value)
    }
}

impl From<SearchClientError> for HandleError {
    fn from(value: SearchClientError) -> Self {
        HandleError::SearchClientError(value)
    }
}