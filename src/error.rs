#[cfg(feature = "gitlab12")]
pub type ApiGitlabError = gitlab::GitlabError;
#[cfg(feature = "gitlab12")]
use gitlab::api::ApiError as GitlabApiError;

// This is hack since gitlab::.*::RestError Error wasn't public
pub type ApiRestError = <gitlab::Gitlab as gitlab::api::Client>::Error;

use serde_json::Error as JsonError;
use std::io::Error as StdIoError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("gitlab: {}", .0)]
    LegacyGitlabError(#[from] ApiGitlabError),
    #[error("io: {}", .0)]
    StdIoError(#[from] StdIoError),
    #[error("json: {}", .0)]
    Json(#[from] JsonError),
    #[error(transparent)]
    ApiError(#[from] GitlabApiError<ApiRestError>),
    #[error(transparent)]
    GitlabCtl(#[from] GitlabCtlError),
}

#[derive(Debug, Error)]
pub enum GitlabCtlError {
    #[error("User {user} was not found")]
    UserNotFound { user: String },
}
