use clickup::{ClickUpAuth, ClickUpClient, ClickUpConfig, Task, TaskReference, TrackTaskOptions};
use serde::Serialize;
use serde_json::Value;

struct AppState {
    clickup_ready: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskFocusField {
    name: String,
    value: String,
    required: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskFocusSubtask {
    id: String,
    title: String,
    status: String,
    is_closed: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskFocusResponse {
    id: String,
    custom_id: Option<String>,
    title: String,
    status: Option<String>,
    priority: Option<String>,
    description: Option<String>,
    path_label: Option<String>,
    due_date: Option<String>,
    start_date: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    assignees: Vec<String>,
    key_fields: Vec<TaskFocusField>,
    subtasks: Vec<TaskFocusSubtask>,
    completed_subtasks: usize,
    total_subtasks: usize,
}

#[tauri::command]
fn clickup_sdk_status(state: tauri::State<'_, AppState>) -> bool {
    state.clickup_ready
}

#[tauri::command]
async fn get_task_focus(
    workspace_id: u64,
    task_id: String,
    auth_token: String,
) -> Result<TaskFocusResponse, String> {
    let auth = ClickUpAuth::from_authorization_value(auth_token);
    let config = ClickUpConfig::new(auth).map_err(|error| error.to_string())?;
    let client = ClickUpClient::new(config).map_err(|error| error.to_string())?;
    let reference = resolve_task_reference(workspace_id, &task_id);

    let tracked = client
        .track_task(
            workspace_id,
            &reference,
            &TrackTaskOptions {
                include_markdown_description: true,
            },
        )
        .await
        .map_err(|error| error.to_string())?;

    let task = tracked.task;
    let subtasks: Vec<TaskFocusSubtask> = tracked
        .subtasks
        .into_iter()
        .map(|subtask| {
            let title = subtask.display_name().to_owned();

            TaskFocusSubtask {
                id: subtask.id,
                title,
                status: subtask
                    .status
                    .as_ref()
                    .and_then(|status| status.status.as_deref())
                    .unwrap_or("Unknown")
                    .to_owned(),
                is_closed: subtask
                    .status
                    .as_ref()
                    .and_then(|status| status.r#type.as_deref())
                    == Some("closed"),
            }
        })
        .collect();

    let completed_subtasks = subtasks.iter().filter(|subtask| subtask.is_closed).count();

    Ok(TaskFocusResponse {
        id: task.id.clone(),
        custom_id: task.custom_id.clone(),
        title: task.display_name().to_owned(),
        status: task
            .status
            .as_ref()
            .and_then(|status| status.status.as_deref())
            .map(str::to_owned),
        priority: task
            .priority
            .as_ref()
            .and_then(|priority| priority.priority.as_deref())
            .map(str::to_owned),
        description: task_summary(&task),
        path_label: task_path(&task),
        due_date: task.due_date.clone(),
        start_date: task.start_date.clone(),
        created_at: task.date_created.clone(),
        updated_at: task.date_updated.clone(),
        assignees: assignee_names(&task),
        key_fields: key_fields(&task),
        total_subtasks: subtasks.len(),
        completed_subtasks,
        subtasks,
    })
}

fn resolve_task_reference(workspace_id: u64, task_id: &str) -> TaskReference {
    if looks_like_custom_task_id(task_id) {
        TaskReference::custom_id(workspace_id, task_id)
    } else {
        TaskReference::id(task_id)
    }
}

fn looks_like_custom_task_id(task_id: &str) -> bool {
    let Some((prefix, suffix)) = task_id.split_once('-') else {
        return false;
    };

    !prefix.is_empty()
        && !suffix.is_empty()
        && prefix
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
        && suffix.chars().all(|character| character.is_ascii_digit())
}

fn task_summary(task: &Task) -> Option<String> {
    task.markdown_description
        .as_deref()
        .or(task.description.as_deref())
        .or(task.text_content.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

fn task_path(task: &Task) -> Option<String> {
    let mut segments = Vec::new();

    if let Some(space_name) = task.space.as_ref().and_then(|space| space.name.as_deref()) {
        segments.push(space_name.to_owned());
    }
    if let Some(folder_name) = task
        .folder
        .as_ref()
        .and_then(|folder| folder.name.as_deref())
    {
        segments.push(folder_name.to_owned());
    }
    if let Some(list_name) = task.list.as_ref().and_then(|list| list.name.as_deref()) {
        segments.push(list_name.to_owned());
    }

    if segments.is_empty() {
        None
    } else {
        Some(segments.join(" / "))
    }
}

fn assignee_names(task: &Task) -> Vec<String> {
    match task.extra.get("assignees") {
        Some(Value::Array(assignees)) => assignees
            .iter()
            .filter_map(|assignee| {
                assignee
                    .get("username")
                    .and_then(Value::as_str)
                    .or_else(|| assignee.get("email").and_then(Value::as_str))
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned)
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn key_fields(task: &Task) -> Vec<TaskFocusField> {
    match task.extra.get("custom_fields") {
        Some(Value::Array(fields)) => fields.iter().filter_map(custom_field).take(8).collect(),
        _ => Vec::new(),
    }
}

fn custom_field(field: &Value) -> Option<TaskFocusField> {
    let name = field.get("name")?.as_str()?.trim();
    if name.is_empty() {
        return None;
    }

    let value = custom_field_value(field)?;

    Some(TaskFocusField {
        name: name.to_owned(),
        value,
        required: field
            .get("required")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    })
}

fn custom_field_value(field: &Value) -> Option<String> {
    let value = field.get("value")?;
    resolve_custom_field_option(value, field.get("type_config"))
        .or_else(|| display_value(value))
        .map(|formatted| formatted.trim().to_owned())
        .filter(|formatted| !formatted.is_empty())
}

fn resolve_custom_field_option(value: &Value, type_config: Option<&Value>) -> Option<String> {
    let options = type_config?.get("options")?.as_array()?;

    match value {
        Value::Array(values) => {
            let labels: Vec<String> = values
                .iter()
                .filter_map(|entry| option_label(entry, options))
                .collect();

            if labels.is_empty() {
                None
            } else {
                Some(labels.join(", "))
            }
        }
        _ => option_label(value, options),
    }
}

fn option_label(value: &Value, options: &[Value]) -> Option<String> {
    let needle = match value {
        Value::String(inner) => inner.clone(),
        Value::Number(inner) => inner.to_string(),
        _ => return None,
    };

    options.iter().find_map(|option| {
        let matches = option
            .get("id")
            .and_then(Value::as_str)
            .map(|candidate| candidate == needle)
            .unwrap_or(false)
            || option
                .get("orderindex")
                .and_then(Value::as_str)
                .map(|candidate| candidate == needle)
                .unwrap_or(false)
            || option
                .get("name")
                .and_then(Value::as_str)
                .map(|candidate| candidate == needle)
                .unwrap_or(false)
            || option
                .get("label")
                .and_then(Value::as_str)
                .map(|candidate| candidate == needle)
                .unwrap_or(false);

        if !matches {
            return None;
        }

        option
            .get("name")
            .and_then(Value::as_str)
            .or_else(|| option.get("label").and_then(Value::as_str))
            .map(str::trim)
            .filter(|candidate| !candidate.is_empty())
            .map(str::to_owned)
    })
}

fn display_value(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::Bool(inner) => Some(if *inner { "Yes" } else { "No" }.to_owned()),
        Value::Number(inner) => Some(inner.to_string()),
        Value::String(inner) => {
            let trimmed = inner.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_owned())
            }
        }
        Value::Array(items) => {
            let values: Vec<String> = items.iter().filter_map(display_value).collect();
            if values.is_empty() {
                None
            } else {
                Some(values.join(", "))
            }
        }
        Value::Object(object) => object
            .get("label")
            .and_then(display_value)
            .or_else(|| object.get("name").and_then(display_value))
            .or_else(|| object.get("username").and_then(display_value))
            .or_else(|| object.get("email").and_then(display_value))
            .or_else(|| {
                let current = object.get("current").and_then(display_value)?;
                let total = object.get("total").and_then(display_value)?;
                Some(format!("{current} / {total}"))
            }),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let clickup_ready = ClickUpConfig::from_env().is_ok();

    tauri::Builder::default()
        .manage(AppState { clickup_ready })
        .invoke_handler(tauri::generate_handler![clickup_sdk_status, get_task_focus])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{custom_field, looks_like_custom_task_id};

    #[test]
    fn detects_custom_task_ids() {
        assert!(looks_like_custom_task_id("TASK-123"));
        assert!(!looks_like_custom_task_id("8675309"));
    }

    #[test]
    fn custom_fields_resolve_option_labels() {
        let field = json!({
            "name": "需求优先级",
            "required": true,
            "value": "1",
            "type_config": {
                "options": [
                    { "orderindex": "0", "name": "P0" },
                    { "orderindex": "1", "name": "P1" }
                ]
            }
        });

        let parsed = custom_field(&field).expect("custom field should parse");

        assert_eq!(parsed.name, "需求优先级");
        assert_eq!(parsed.value, "P1");
        assert!(parsed.required);
    }
}
