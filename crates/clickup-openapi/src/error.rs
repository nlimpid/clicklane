use reqwest::{header::InvalidHeaderValue, StatusCode};
use thiserror::Error;

/// Convenience alias for `Result<T, ClickUpError>`.
pub type Result<T> = std::result::Result<T, ClickUpError>;

/// Errors returned by the ClickUp API client.
#[derive(Debug, Error)]
pub enum ClickUpError {
    /// No auth token was found in the environment.
    #[error("ClickUp credentials are missing. Set CLICKUP_API_TOKEN or CLICKUP_ACCESS_TOKEN.")]
    MissingAuthToken,

    /// Custom task IDs require a workspace (team) ID to resolve.
    #[error("custom task ids require a workspace id")]
    MissingWorkspaceIdForCustomTask,

    /// The provided API base URL is not a valid URL.
    #[error("invalid ClickUp API base URL: {0}")]
    InvalidBaseUrl(#[from] url::ParseError),

    /// The token produced an invalid HTTP header value.
    #[error("invalid ClickUp authorization header: {0}")]
    InvalidHeader(#[from] InvalidHeaderValue),

    /// A network-level error from `reqwest`.
    #[error("request to ClickUp failed: {0}")]
    Request(#[from] reqwest::Error),

    /// The ClickUp API returned a non-2xx status code.
    #[error("ClickUp returned {status}: {message}")]
    Http {
        /// The HTTP status code.
        status: StatusCode,
        /// The error message extracted from the response body.
        message: String,
    },

    /// The response body could not be deserialized.
    #[error("failed to decode ClickUp response: {source}; body={body}")]
    Decode {
        /// The underlying deserialization error.
        #[source]
        source: serde_json::Error,
        /// The raw response body, included for debugging.
        body: String,
    },
}
