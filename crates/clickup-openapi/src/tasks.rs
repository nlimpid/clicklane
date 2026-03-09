use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{client::ClickUpClient, error::Result};

const GET_TASK_PATH: &str = "/api/v2/task/{task_id}";
const LIST_TEAM_TASKS_PATH: &str = "/api/v2/team/{team_id}/task";
const MOVE_TASK_V3_PATH: &str =
    "/api/v3/workspaces/{workspace_id}/tasks/{task_id}/home_list/{list_id}";

/// Identifies a ClickUp task either by its internal ID or by a custom task ID.
///
/// # Examples
///
/// ```
/// use clickup_openapi::TaskReference;
///
/// // By internal ClickUp ID
/// let r1 = TaskReference::id("86abc123");
///
/// // By custom task ID (e.g. from an integration or shorthand like "PROJ-42")
/// let r2 = TaskReference::custom_id(9_012_345_678, "PROJ-42");
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskReference {
    /// A ClickUp-internal task ID (e.g. `"86abc123"`).
    Id(String),
    /// A custom task ID scoped to a workspace.
    CustomId {
        /// The ClickUp workspace (team) ID required to resolve the custom task ID.
        workspace_id: u64,
        /// The custom task ID (e.g. `"PROJ-42"`).
        custom_task_id: String,
    },
}

impl TaskReference {
    /// Creates a reference by internal ClickUp task ID.
    pub fn id(task_id: impl Into<String>) -> Self {
        Self::Id(task_id.into())
    }

    /// Creates a reference by custom task ID within a workspace.
    pub fn custom_id(workspace_id: u64, custom_task_id: impl Into<String>) -> Self {
        Self::CustomId {
            workspace_id,
            custom_task_id: custom_task_id.into(),
        }
    }

    /// Returns the workspace ID if this is a [`TaskReference::CustomId`].
    pub fn workspace_id(&self) -> Option<u64> {
        match self {
            Self::Id(_) => None,
            Self::CustomId { workspace_id, .. } => Some(*workspace_id),
        }
    }

    fn path_value(&self) -> &str {
        match self {
            Self::Id(task_id) => task_id,
            Self::CustomId { custom_task_id, .. } => custom_task_id,
        }
    }

    fn append_query(&self, query: &mut Vec<(String, String)>) {
        if let Self::CustomId { workspace_id, .. } = self {
            query.push(("custom_task_ids".to_owned(), "true".to_owned()));
            query.push(("team_id".to_owned(), workspace_id.to_string()));
        }
    }
}

/// Options for [`ClickUpClient::get_task`].
#[derive(Clone, Debug, Default)]
pub struct GetTaskOptions {
    /// Include the task's subtasks in the response.
    pub include_subtasks: bool,
    /// Request the `markdown_description` field in addition to plain-text descriptions.
    pub include_markdown_description: bool,
}

/// Request parameters for [`ClickUpClient::list_team_tasks`].
#[derive(Clone, Debug, Default)]
pub struct ListTeamTasksRequest {
    /// The ClickUp workspace (team) ID.
    pub workspace_id: u64,
    /// Filter to subtasks of this parent task ID.
    pub parent: Option<String>,
    /// Include tasks in closed statuses.
    pub include_closed: bool,
    /// Include subtasks in the listing.
    pub include_subtasks: bool,
    /// Request markdown descriptions.
    pub include_markdown_description: bool,
    /// Zero-based page number for paginated results.
    pub page: Option<u32>,
}

impl ListTeamTasksRequest {
    /// Creates a request pre-configured for fetching all subtasks of a parent task.
    pub fn for_subtasks(workspace_id: u64, parent: impl Into<String>) -> Self {
        Self {
            workspace_id,
            parent: Some(parent.into()),
            include_closed: true,
            include_subtasks: true,
            include_markdown_description: false,
            page: None,
        }
    }

    fn to_query(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();

        if let Some(parent) = &self.parent {
            query.push(("parent".to_owned(), parent.clone()));
        }
        if self.include_closed {
            query.push(("include_closed".to_owned(), "true".to_owned()));
        }
        if self.include_subtasks {
            query.push(("subtasks".to_owned(), "true".to_owned()));
        }
        if self.include_markdown_description {
            query.push(("include_markdown_description".to_owned(), "true".to_owned()));
        }
        if let Some(page) = self.page {
            query.push(("page".to_owned(), page.to_string()));
        }

        query
    }
}

/// Options for [`ClickUpClient::track_task`].
#[derive(Clone, Debug, Default)]
pub struct TrackTaskOptions {
    /// Request markdown descriptions for the task and its subtasks.
    pub include_markdown_description: bool,
}

/// A ClickUp task as returned by the v2 API.
///
/// Unknown fields are captured in [`Task::extra`] so callers are not broken by
/// undocumented or new API fields.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    /// The ClickUp-internal task ID.
    pub id: String,
    /// The custom task ID if one is configured for the workspace.
    #[serde(default)]
    pub custom_id: Option<String>,
    /// The task name.
    #[serde(default)]
    pub name: Option<String>,
    /// Plain-text content of the task description.
    #[serde(default)]
    pub text_content: Option<String>,
    /// Plain-text description (may differ from `text_content` in some contexts).
    #[serde(default)]
    pub description: Option<String>,
    /// Markdown description, populated when `include_markdown_description=true`.
    #[serde(default)]
    pub markdown_description: Option<String>,
    /// Current task status.
    #[serde(default)]
    pub status: Option<TaskStatus>,
    /// Parent task ID if this is a subtask.
    #[serde(default)]
    pub parent: Option<String>,
    /// Task priority.
    #[serde(default)]
    pub priority: Option<TaskPriority>,
    /// Due date as a Unix timestamp string (milliseconds).
    #[serde(default)]
    pub due_date: Option<String>,
    /// Start date as a Unix timestamp string (milliseconds).
    #[serde(default)]
    pub start_date: Option<String>,
    /// Creation date as a Unix timestamp string (milliseconds).
    #[serde(default)]
    pub date_created: Option<String>,
    /// Last-updated date as a Unix timestamp string (milliseconds).
    #[serde(default)]
    pub date_updated: Option<String>,
    /// Date the task was closed, if applicable.
    #[serde(default)]
    pub date_closed: Option<String>,
    /// Date the task was marked done, if applicable.
    #[serde(default)]
    pub date_done: Option<String>,
    /// The ClickUp web URL for this task.
    #[serde(default)]
    pub url: Option<String>,
    /// The user who created the task.
    #[serde(default)]
    pub creator: Option<TaskUserSummary>,
    /// The list this task belongs to.
    #[serde(default)]
    pub list: Option<TaskLocationRef>,
    /// The folder this task belongs to.
    #[serde(default)]
    pub folder: Option<TaskLocationRef>,
    /// The space this task belongs to.
    #[serde(default)]
    pub space: Option<TaskLocationRef>,
    /// Additional fields not explicitly modelled above.
    #[serde(default, flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl Task {
    /// Returns the task name if available, falling back to the raw task ID.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.id)
    }

    /// Extracts subtasks that were inlined into the `extra` map by the API.
    ///
    /// Returns an empty vec if no inline subtasks are present.
    pub fn inline_subtasks(&self) -> Vec<Task> {
        match self.extra.get("subtasks") {
            Some(value) => serde_json::from_value(value.clone()).unwrap_or_default(),
            None => Vec::new(),
        }
    }
}

/// A paginated page of tasks from [`ClickUpClient::list_team_tasks`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskPage {
    /// The tasks in this page.
    #[serde(default)]
    pub tasks: Vec<Task>,
    /// `true` if there are no more pages after this one.
    #[serde(default)]
    pub last_page: bool,
}

/// A task together with its resolved subtask list, as returned by
/// [`ClickUpClient::track_task`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackedTask {
    /// The primary task.
    pub task: Task,
    /// The subtasks of the primary task.
    #[serde(default)]
    pub subtasks: Vec<Task>,
}

/// The status of a ClickUp task (e.g. `"in progress"`, `"done"`).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskStatus {
    /// Internal status ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Human-readable status label.
    #[serde(default)]
    pub status: Option<String>,
    /// Hex colour associated with the status.
    #[serde(default)]
    pub color: Option<String>,
    /// Ordering index within the status list.
    #[serde(default)]
    pub orderindex: Option<Value>,
    /// Status category type (e.g. `"open"`, `"closed"`).
    #[serde(default)]
    pub r#type: Option<String>,
}

/// The priority level of a ClickUp task.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskPriority {
    /// Hex colour for the priority level.
    #[serde(default)]
    pub color: Option<String>,
    /// Internal priority ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Ordering index within the priority list.
    #[serde(default)]
    pub orderindex: Option<String>,
    /// Human-readable priority label (e.g. `"urgent"`, `"high"`).
    #[serde(default)]
    pub priority: Option<String>,
}

/// A minimal summary of a ClickUp user as embedded in task responses.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskUserSummary {
    /// The user's ClickUp ID. Can be `-1` for system/automation users.
    #[serde(default)]
    pub id: Option<i64>,
    /// The user's display name.
    #[serde(default)]
    pub username: Option<String>,
    /// The user's profile colour.
    #[serde(default)]
    pub color: Option<String>,
    /// URL of the user's profile picture.
    #[serde(default, rename = "profilePicture")]
    pub profile_picture: Option<String>,
}

/// A lightweight reference to a ClickUp list, folder, or space embedded in a task.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskLocationRef {
    /// The ID of the referenced container.
    pub id: String,
    /// The display name of the referenced container.
    #[serde(default)]
    pub name: Option<String>,
}

/// Request body for [`ClickUpClient::move_task_v3`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MoveTaskRequest {
    /// Whether to move custom field values along with the task.
    #[serde(default)]
    pub move_custom_fields: Option<bool>,
    /// Specific custom field IDs to move (only used when `move_custom_fields` is `false`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_fields_to_move: Vec<String>,
    /// Status mappings to apply when moving between lists with different statuses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_mappings: Vec<StatusMapping>,
}

/// Maps a source status to a destination status when moving a task.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusMapping {
    /// Status ID in the source list.
    pub source_status_id: String,
    /// Corresponding status ID in the destination list.
    pub destination_status_id: String,
}

/// The top-level wrapper returned by the move-task v3 endpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveTaskResponse {
    /// The outcome of the move operation.
    pub data: MoveTaskResult,
}

/// The result payload inside a [`MoveTaskResponse`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveTaskResult {
    /// The ID of the task that was moved.
    pub task_id: String,
    /// The ID of the list the task now belongs to.
    pub new_list_id: String,
}

impl ClickUpClient {
    /// Fetches a single task by ID or custom task ID.
    ///
    /// # Errors
    ///
    /// Returns an error on network failures or if ClickUp returns a non-2xx status.
    pub async fn get_task(
        &self,
        reference: &TaskReference,
        options: &GetTaskOptions,
    ) -> Result<Task> {
        let path = GET_TASK_PATH.replace("{task_id}", reference.path_value());
        let mut query = Vec::new();
        reference.append_query(&mut query);

        if options.include_subtasks {
            query.push(("include_subtasks".to_owned(), "true".to_owned()));
        }
        if options.include_markdown_description {
            query.push(("include_markdown_description".to_owned(), "true".to_owned()));
        }

        self.get_json(&path, &query).await
    }

    /// Lists tasks in a workspace (team), with optional filters.
    ///
    /// Results are paginated; check [`TaskPage::last_page`] and increment
    /// [`ListTeamTasksRequest::page`] to retrieve subsequent pages.
    ///
    /// # Errors
    ///
    /// Returns an error on network failures or if ClickUp returns a non-2xx status.
    pub async fn list_team_tasks(&self, request: &ListTeamTasksRequest) -> Result<TaskPage> {
        let path = LIST_TEAM_TASKS_PATH.replace("{team_id}", &request.workspace_id.to_string());
        let query = request.to_query();
        self.get_json(&path, &query).await
    }

    /// Fetches a task and resolves its complete subtask list.
    ///
    /// When the task already contains inline subtasks in the API response those
    /// are used directly; otherwise a second request via [`ClickUpClient::list_team_tasks`]
    /// is made to fetch them.
    ///
    /// # Errors
    ///
    /// Returns an error on network failures or if ClickUp returns a non-2xx status.
    pub async fn track_task(
        &self,
        workspace_id: u64,
        reference: &TaskReference,
        options: &TrackTaskOptions,
    ) -> Result<TrackedTask> {
        let task = self
            .get_task(
                reference,
                &GetTaskOptions {
                    include_subtasks: true,
                    include_markdown_description: options.include_markdown_description,
                },
            )
            .await?;

        let inline_subtasks = task.inline_subtasks();
        let subtasks = if inline_subtasks.is_empty() {
            let mut request = ListTeamTasksRequest::for_subtasks(workspace_id, task.id.clone());
            request.include_markdown_description = options.include_markdown_description;
            self.list_team_tasks(&request).await?.tasks
        } else {
            inline_subtasks
        };

        Ok(TrackedTask { task, subtasks })
    }

    /// Moves a task to a different list using the ClickUp v3 API.
    ///
    /// # Errors
    ///
    /// Returns an error on network failures or if ClickUp returns a non-2xx status.
    pub async fn move_task_v3(
        &self,
        workspace_id: u64,
        task_id: &str,
        list_id: &str,
        request: &MoveTaskRequest,
    ) -> Result<MoveTaskResponse> {
        let path = MOVE_TASK_V3_PATH
            .replace("{workspace_id}", &workspace_id.to_string())
            .replace("{task_id}", task_id)
            .replace("{list_id}", list_id);

        self.post_json(&path, &[], request).await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{TaskPage, TaskReference};

    const TEST_WORKSPACE_ID: u64 = 42;
    const TEST_CUSTOM_TASK_ID: &str = "TASK-123";

    #[test]
    fn custom_task_references_add_expected_query_params() {
        let reference = TaskReference::custom_id(TEST_WORKSPACE_ID, TEST_CUSTOM_TASK_ID);
        let mut query = Vec::new();
        reference.append_query(&mut query);

        assert!(query.contains(&("custom_task_ids".to_owned(), "true".to_owned())));
        assert!(query.contains(&("team_id".to_owned(), TEST_WORKSPACE_ID.to_string())));
        assert!(!query.iter().any(|(name, _)| name == "workspace_id"));
    }

    #[test]
    fn task_page_deserializes_creator_with_negative_id() {
        let page: TaskPage = serde_json::from_value(json!({
            "tasks": [
                {
                    "id": "86ewg8qe1",
                    "creator": {
                        "id": -1,
                        "username": "System"
                    }
                }
            ],
            "last_page": true
        }))
        .unwrap();

        assert_eq!(page.tasks.len(), 1);
        assert_eq!(
            page.tasks[0]
                .creator
                .as_ref()
                .and_then(|creator| creator.id),
            Some(-1)
        );
    }
}
