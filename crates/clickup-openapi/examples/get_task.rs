//! Fetch a task from ClickUp and print its details.
//!
//! Set credentials in the environment, then run:
//!
//! ```bash
//! CLICKUP_API_TOKEN=pk_... cargo run --example get_task -- 86abc123
//! # or with a custom task ID:
//! CLICKUP_API_TOKEN=pk_... cargo run --example get_task -- PROJ-42 --workspace-id 9012345678
//! ```

use std::env;

use clickup_openapi::{ClickUpClient, TaskReference, TrackTaskOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let task_id = args
        .next()
        .unwrap_or_else(|| panic!("Usage: get_task <task-id> [--workspace-id <id>]"));

    let workspace_id: Option<u64> = {
        let mut id = None;
        while let Some(arg) = args.next() {
            if arg == "--workspace-id" {
                id = args.next().and_then(|v| v.parse().ok());
            }
        }
        id
    };

    let reference = match workspace_id {
        Some(ws) => TaskReference::custom_id(ws, &task_id),
        None => TaskReference::id(&task_id),
    };

    let workspace_id = workspace_id.unwrap_or(0);

    let client = ClickUpClient::from_env()?;
    let tracked = client
        .track_task(
            workspace_id,
            &reference,
            &TrackTaskOptions {
                include_markdown_description: true,
            },
        )
        .await?;

    let task = &tracked.task;
    println!("Name:    {}", task.display_name());
    println!("ID:      {}", task.id);

    if let Some(custom_id) = &task.custom_id {
        println!("Custom:  {custom_id}");
    }
    if let Some(status) = task.status.as_ref().and_then(|s| s.status.as_deref()) {
        println!("Status:  {status}");
    }
    if let Some(url) = &task.url {
        println!("URL:     {url}");
    }

    println!("Subtasks ({}):", tracked.subtasks.len());
    for subtask in &tracked.subtasks {
        let status = subtask
            .status
            .as_ref()
            .and_then(|s| s.status.as_deref())
            .unwrap_or("?");
        println!("  - {} [{}]", subtask.display_name(), status);
    }

    Ok(())
}
