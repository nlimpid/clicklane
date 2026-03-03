use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{client::ClickUpClient, error::Result};

const GET_TASK_PATH: &str = "/api/v2/task/{task_id}";
const LIST_TEAM_TASKS_PATH: &str = "/api/v2/team/{team_id}/task";
const MOVE_TASK_V3_PATH: &str =
    "/api/v3/workspaces/{workspace_id}/tasks/{task_id}/home_list/{list_id}";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskReference {
    Id(String),
    CustomId {
        workspace_id: u64,
        custom_task_id: String,
    },
}

impl TaskReference {
    pub fn id(task_id: impl Into<String>) -> Self {
        Self::Id(task_id.into())
    }

    pub fn custom_id(workspace_id: u64, custom_task_id: impl Into<String>) -> Self {
        Self::CustomId {
            workspace_id,
            custom_task_id: custom_task_id.into(),
        }
    }

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

#[derive(Clone, Debug, Default)]
pub struct GetTaskOptions {
    pub include_subtasks: bool,
    pub include_markdown_description: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ListTeamTasksRequest {
    pub workspace_id: u64,
    pub parent: Option<String>,
    pub include_closed: bool,
    pub include_subtasks: bool,
    pub include_markdown_description: bool,
    pub page: Option<u32>,
}

impl ListTeamTasksRequest {
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

#[derive(Clone, Debug, Default)]
pub struct TrackTaskOptions {
    pub include_markdown_description: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    #[serde(default)]
    pub custom_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub text_content: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub markdown_description: Option<String>,
    #[serde(default)]
    pub status: Option<TaskStatus>,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub priority: Option<TaskPriority>,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub date_created: Option<String>,
    #[serde(default)]
    pub date_updated: Option<String>,
    #[serde(default)]
    pub date_closed: Option<String>,
    #[serde(default)]
    pub date_done: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub creator: Option<TaskUserSummary>,
    #[serde(default)]
    pub list: Option<TaskLocationRef>,
    #[serde(default)]
    pub folder: Option<TaskLocationRef>,
    #[serde(default)]
    pub space: Option<TaskLocationRef>,
    #[serde(default, flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl Task {
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.id)
    }

    pub fn inline_subtasks(&self) -> Vec<Task> {
        match self.extra.get("subtasks") {
            Some(value) => serde_json::from_value(value.clone()).unwrap_or_default(),
            None => Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskPage {
    #[serde(default)]
    pub tasks: Vec<Task>,
    #[serde(default)]
    pub last_page: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackedTask {
    pub task: Task,
    #[serde(default)]
    pub subtasks: Vec<Task>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskStatus {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub orderindex: Option<Value>,
    #[serde(default)]
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskPriority {
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub orderindex: Option<String>,
    #[serde(default)]
    pub priority: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskUserSummary {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default, rename = "profilePicture")]
    pub profile_picture: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskLocationRef {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MoveTaskRequest {
    #[serde(default)]
    pub move_custom_fields: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_fields_to_move: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_mappings: Vec<StatusMapping>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusMapping {
    pub source_status_id: String,
    pub destination_status_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveTaskResponse {
    pub data: MoveTaskResult,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveTaskResult {
    pub task_id: String,
    pub new_list_id: String,
}

impl ClickUpClient {
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

    pub async fn list_team_tasks(&self, request: &ListTeamTasksRequest) -> Result<TaskPage> {
        let path = LIST_TEAM_TASKS_PATH.replace("{team_id}", &request.workspace_id.to_string());
        let query = request.to_query();
        self.get_json(&path, &query).await
    }

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
