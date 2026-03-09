//! # clickup-openapi
//!
//! A handwritten async Rust SDK for the [ClickUp API](https://developer.clickup.com/docs).
//!
//! ## Quick start
//!
//! ```no_run
//! use clickup_openapi::{ClickUpClient, TaskReference, TrackTaskOptions};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), clickup_openapi::ClickUpError> {
//!     // Reads CLICKUP_API_TOKEN or CLICKUP_ACCESS_TOKEN from the environment
//!     let client = ClickUpClient::from_env()?;
//!
//!     let tracked = client
//!         .track_task(
//!             9_012_345_678,
//!             &TaskReference::id("86abc123"),
//!             &TrackTaskOptions::default(),
//!         )
//!         .await?;
//!
//!     println!("{} — {} subtasks", tracked.task.display_name(), tracked.subtasks.len());
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! | Env var | Type |
//! |---|---|
//! | `CLICKUP_API_TOKEN` | Personal API key (`pk_...`) |
//! | `CLICKUP_ACCESS_TOKEN` | OAuth Bearer token |
//! | `CLICKUP_OAUTH_ACCESS_TOKEN` | OAuth Bearer token |
//! | `CLICKUP_API_BASE_URL` | Optional base URL override |
//!
//! You can also construct auth explicitly:
//!
//! ```no_run
//! use clickup_openapi::{ClickUpAuth, ClickUpClient};
//!
//! let client = ClickUpClient::builder()
//!     .auth(ClickUpAuth::personal("pk_live_..."))
//!     .build()?;
//! # Ok::<_, clickup_openapi::ClickUpError>(())
//! ```

mod auth;
mod client;
mod error;
mod tasks;

pub use auth::{ClickUpAuth, ClickUpConfig};
pub use client::{ClickUpClient, ClickUpClientBuilder};
pub use error::{ClickUpError, Result};
pub use tasks::{
    GetTaskOptions, ListTeamTasksRequest, MoveTaskRequest, MoveTaskResponse, MoveTaskResult,
    StatusMapping, Task, TaskLocationRef, TaskPage, TaskPriority, TaskReference, TaskStatus,
    TaskUserSummary, TrackTaskOptions, TrackedTask,
};

/// A convenience re-export of the most commonly used items.
///
/// ```no_run
/// use clickup_openapi::prelude::*;
///
/// let client = ClickUpClient::from_env()?;
/// # Ok::<_, clickup_openapi::ClickUpError>(())
/// ```
pub mod prelude {
    pub use crate::{
        ClickUpAuth, ClickUpClient, ClickUpClientBuilder, ClickUpError, GetTaskOptions,
        ListTeamTasksRequest, Task, TaskReference, TrackTaskOptions, TrackedTask,
    };
}

/// URLs for ClickUp's developer resources.
pub mod docs {
    /// Authentication guide on the ClickUp developer portal.
    pub const AUTHENTICATION_GUIDE_URL: &str = "https://developer.clickup.com/docs/authentication";
    /// ClickUp public API v2 OpenAPI specification.
    pub const OPENAPI_V2_SPEC_URL: &str =
        "https://developer.clickup.com/openapi/clickup-api-v2-reference.json";
    /// ClickUp public API v3 OpenAPI specification.
    pub const OPENAPI_V3_SPEC_URL: &str =
        "https://developer.clickup.com/openapi/ClickUp_PUBLIC_API_V3.yaml";
}
