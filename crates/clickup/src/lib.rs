mod auth;
mod client;
mod error;
mod tasks;

pub use auth::{ClickUpAuth, ClickUpConfig};
pub use client::ClickUpClient;
pub use error::{ClickUpError, Result};
pub use tasks::{
    GetTaskOptions, ListTeamTasksRequest, MoveTaskRequest, MoveTaskResponse, MoveTaskResult,
    StatusMapping, Task, TaskLocationRef, TaskPage, TaskPriority, TaskReference, TaskStatus,
    TaskUserSummary, TrackTaskOptions, TrackedTask,
};

pub mod docs {
    pub const AUTHENTICATION_GUIDE_URL: &str = "https://developer.clickup.com/docs/authentication";
    pub const OPENAPI_V2_SPEC_URL: &str =
        "https://developer.clickup.com/openapi/clickup-api-v2-reference.json";
    pub const OPENAPI_V3_SPEC_URL: &str =
        "https://developer.clickup.com/openapi/ClickUp_PUBLIC_API_V3.yaml";
}
