# clickup-openapi

A handwritten Rust SDK for the [ClickUp API](https://developer.clickup.com/docs), with support for personal API keys, OAuth tokens, task tracking, and list moves.

## Usage

Add the dependency:

```toml
[dependencies]
clickup-openapi = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### Quick start

```rust
use clickup_openapi::{ClickUpClient, TaskReference, TrackTaskOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reads CLICKUP_API_TOKEN or CLICKUP_ACCESS_TOKEN from the environment
    let client = ClickUpClient::from_env()?;

    let task = client
        .get_task(
            &TaskReference::id("86abc123"),
            &Default::default(),
        )
        .await?;

    println!("{}", task.display_name());
    Ok(())
}
```

### Custom task IDs

```rust
let reference = TaskReference::custom_id(workspace_id, "PROJ-42");
let tracked = client.track_task(workspace_id, &reference, &TrackTaskOptions {
    include_markdown_description: true,
}).await?;
```

### Builder-style client

```rust
use clickup_openapi::{ClickUpClient, ClickUpAuth};

let client = ClickUpClient::builder()
    .auth(ClickUpAuth::personal("pk_..."))
    .build()?;
```

## Authentication

| Env var | Auth type |
|---|---|
| `CLICKUP_API_TOKEN` | Personal API key (`pk_...`) |
| `CLICKUP_ACCESS_TOKEN` | OAuth Bearer token |
| `CLICKUP_OAUTH_ACCESS_TOKEN` | OAuth Bearer token |
| `CLICKUP_API_BASE_URL` | Optional base URL override |

## License

MIT
