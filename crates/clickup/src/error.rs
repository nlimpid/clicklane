use reqwest::{header::InvalidHeaderValue, StatusCode};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ClickUpError>;

#[derive(Debug, Error)]
pub enum ClickUpError {
    #[error("ClickUp credentials are missing. Set CLICKUP_API_TOKEN or CLICKUP_ACCESS_TOKEN.")]
    MissingAuthToken,
    #[error("custom task ids require a workspace id")]
    MissingWorkspaceIdForCustomTask,
    #[error("invalid ClickUp API base URL: {0}")]
    InvalidBaseUrl(#[from] url::ParseError),
    #[error("invalid ClickUp authorization header: {0}")]
    InvalidHeader(#[from] InvalidHeaderValue),
    #[error("request to ClickUp failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("ClickUp returned {status}: {message}")]
    Http { status: StatusCode, message: String },
    #[error("failed to decode ClickUp response: {source}; body={body}")]
    Decode {
        #[source]
        source: serde_json::Error,
        body: String,
    },
}
