//! Integration tests against the real ClickUp API.
//!
//! These tests are **ignored by default** so that `cargo test` works without
//! credentials.  Run them explicitly with:
//!
//! ```bash
//! CLICKUP_API_TOKEN=pk_... \
//! CLICKUP_WORKSPACE_ID=90171009640 \
//! cargo test -p clickup-openapi --test integration -- --ignored
//! ```
//!
//! Required environment variables:
//! - `CLICKUP_API_TOKEN` or `CLICKUP_ACCESS_TOKEN` — your ClickUp API token
//! - `CLICKUP_WORKSPACE_ID` — the numeric workspace / team ID to test against

use clickup_openapi::{ClickUpClient, ListTeamTasksRequest};

fn workspace_id() -> u64 {
    std::env::var("CLICKUP_WORKSPACE_ID")
        .expect("CLICKUP_WORKSPACE_ID must be set to run integration tests")
        .parse()
        .expect("CLICKUP_WORKSPACE_ID must be a valid u64")
}

#[tokio::test]
#[ignore = "requires CLICKUP_API_TOKEN and CLICKUP_WORKSPACE_ID"]
async fn list_team_tasks_returns_results() {
    let client = ClickUpClient::from_env().expect("failed to build ClickUp client from env");
    let ws = workspace_id();

    let request = ListTeamTasksRequest {
        workspace_id: ws,
        include_closed: false,
        ..Default::default()
    };

    let page = client
        .list_team_tasks(&request)
        .await
        .expect("list_team_tasks failed");

    // The workspace exists and the API responded — that's enough to confirm auth works.
    println!("tasks on first page: {}", page.tasks.len());
}

#[tokio::test]
#[ignore = "requires CLICKUP_API_TOKEN and CLICKUP_WORKSPACE_ID"]
async fn list_team_tasks_pagination_fields_are_present() {
    let client = ClickUpClient::from_env().expect("failed to build ClickUp client from env");
    let ws = workspace_id();

    let request = ListTeamTasksRequest {
        workspace_id: ws,
        page: Some(0),
        include_closed: true,
        include_subtasks: false,
        ..Default::default()
    };

    let page = client
        .list_team_tasks(&request)
        .await
        .expect("list_team_tasks failed");

    // last_page should be set (true or false) — just assert the response parsed cleanly.
    println!(
        "page 0: {} tasks, last_page={}",
        page.tasks.len(),
        page.last_page
    );
}
