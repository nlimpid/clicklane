use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process,
};

use clap::{Args, Parser, Subcommand, ValueEnum};
use clickup_openapi::{ClickUpClient, TaskReference, TrackTaskOptions, TrackedTask};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use serde::Deserialize;

const RELEASE_OWNER: &str = "nlimpid";
const RELEASE_REPO: &str = "clicklane";
const RELEASE_API_BASE: &str = "https://api.github.com";

#[derive(Debug, Parser)]
#[command(
    name = "clickup-cli",
    about = "Track ClickUp work from the terminal and manage installed macOS releases."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Task(TaskCommand),
    Install(InstallCommand),
    Update(UpdateCommand),
}

#[derive(Debug, Args)]
struct TaskCommand {
    #[arg(long)]
    workspace_id: u64,
    #[arg(long)]
    task_id: String,
    #[arg(
        long,
        help = "Treat task_id as a custom task id, for example TASK-123."
    )]
    custom_id: bool,
    #[arg(long, help = "Ask ClickUp for markdown descriptions when available.")]
    markdown: bool,
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

#[derive(Debug, Args)]
struct InstallCommand {
    #[arg(long, help = "Install into this directory instead of ~/.local/bin.")]
    dir: Option<PathBuf>,
    #[arg(long, help = "Replace an existing binary if one is already installed.")]
    force: bool,
}

#[derive(Debug, Args)]
struct UpdateCommand {
    #[arg(long, help = "Update this path instead of the current executable.")]
    target_path: Option<PathBuf>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubReleaseAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubReleaseAsset {
    name: String,
    browser_download_url: String,
}

struct ReleaseBinary {
    tag_name: String,
    bytes: Vec<u8>,
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("{error}");
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Task(command) => {
            let client = ClickUpClient::from_env()?;
            let reference =
                resolve_task_reference(command.workspace_id, command.task_id, command.custom_id);

            let tracked = client
                .track_task(
                    command.workspace_id,
                    &reference,
                    &TrackTaskOptions {
                        include_markdown_description: command.markdown,
                    },
                )
                .await?;

            match command.format {
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&tracked)?);
                }
                OutputFormat::Text => print_text(&tracked),
            }
        }
        Command::Install(command) => install_latest_release(command).await?,
        Command::Update(command) => update_current_install(command).await?,
    }

    Ok(())
}

async fn install_latest_release(command: InstallCommand) -> Result<(), Box<dyn std::error::Error>> {
    let destination_dir = match command.dir {
        Some(path) => path,
        None => default_install_dir()?,
    };

    let destination = destination_dir.join(binary_name());
    if destination.exists() && !command.force {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!(
                "{} already exists. Re-run with --force to replace it.",
                destination.display()
            ),
        )
        .into());
    }

    fs::create_dir_all(&destination_dir)?;

    let release = download_latest_release_binary().await?;
    write_binary(&destination, &release.bytes)?;

    println!(
        "Installed {} from {}/{} {}",
        destination.display(),
        RELEASE_OWNER,
        RELEASE_REPO,
        release.tag_name
    );

    Ok(())
}

async fn update_current_install(command: UpdateCommand) -> Result<(), Box<dyn std::error::Error>> {
    let destination = match command.target_path {
        Some(path) => path,
        None => env::current_exe()?,
    };

    let release = download_latest_release_binary().await?;
    write_binary(&destination, &release.bytes)?;

    println!(
        "Updated {} from {}/{} {}",
        destination.display(),
        RELEASE_OWNER,
        RELEASE_REPO,
        release.tag_name
    );

    Ok(())
}

fn resolve_task_reference(workspace_id: u64, task_id: String, custom_id: bool) -> TaskReference {
    if custom_id || looks_like_custom_task_id(&task_id) {
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
        && prefix.chars().all(|char| char.is_ascii_alphanumeric())
        && suffix.chars().all(|char| char.is_ascii_digit())
}

fn print_text(tracked: &TrackedTask) {
    let task = &tracked.task;
    println!("Task: {}", task.display_name());
    println!("ID: {}", task.id);

    if let Some(custom_id) = &task.custom_id {
        println!("Custom ID: {custom_id}");
    }
    if let Some(status) = task
        .status
        .as_ref()
        .and_then(|status| status.status.as_deref())
    {
        println!("Status: {status}");
    }
    if let Some(priority) = task
        .priority
        .as_ref()
        .and_then(|priority| priority.priority.as_deref())
    {
        println!("Priority: {priority}");
    }
    if let Some(due_date) = &task.due_date {
        println!("Due: {due_date}");
    }
    if let Some(url) = &task.url {
        println!("URL: {url}");
    }

    let description = task
        .markdown_description
        .as_deref()
        .or(task.description.as_deref())
        .or(task.text_content.as_deref());

    if let Some(description) = description.map(str::trim).filter(|value| !value.is_empty()) {
        println!();
        println!("Description:");
        println!("{}", summarize(description));
    }

    println!();
    println!("Subtasks: {}", tracked.subtasks.len());
    for subtask in &tracked.subtasks {
        let status = subtask
            .status
            .as_ref()
            .and_then(|status| status.status.as_deref())
            .unwrap_or("unknown");
        println!("- {} [{}]", subtask.display_name(), status);
    }
}

fn summarize(value: &str) -> String {
    const MAX_LEN: usize = 280;
    if value.chars().count() <= MAX_LEN {
        return value.to_owned();
    }

    let summary: String = value.chars().take(MAX_LEN).collect();
    format!("{summary}...")
}

async fn download_latest_release_binary() -> Result<ReleaseBinary, Box<dyn std::error::Error>> {
    let asset_name = release_asset_name()?;
    let client = github_http_client()?;
    let release_url =
        format!("{RELEASE_API_BASE}/repos/{RELEASE_OWNER}/{RELEASE_REPO}/releases/latest");

    let release_response = client.get(release_url).send().await?;
    if !release_response.status().is_success() {
        let status = release_response.status();
        let body = release_response.text().await?;
        return Err(io::Error::other(format!(
            "failed to fetch latest release metadata ({status}): {}",
            body.trim()
        ))
        .into());
    }

    let release: GithubRelease = release_response.json().await?;
    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == asset_name)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "release asset `{asset_name}` was not found in {RELEASE_OWNER}/{RELEASE_REPO} {}",
                    release.tag_name
                ),
            )
        })?;

    let binary_response = client.get(&asset.browser_download_url).send().await?;
    if !binary_response.status().is_success() {
        let status = binary_response.status();
        let body = binary_response.text().await?;
        return Err(io::Error::other(format!(
            "failed to download release asset `{asset_name}` ({status}): {}",
            body.trim()
        ))
        .into());
    }

    Ok(ReleaseBinary {
        tag_name: release.tag_name,
        bytes: binary_response.bytes().await?.to_vec(),
    })
}

fn github_http_client() -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(&format!("clickup-cli/{}", env!("CARGO_PKG_VERSION")))?,
    );

    Ok(reqwest::Client::builder()
        .default_headers(headers)
        .build()?)
}

fn release_asset_name() -> io::Result<String> {
    release_asset_name_for(env::consts::OS, env::consts::ARCH)
}

fn release_asset_name_for(os: &str, arch: &str) -> io::Result<String> {
    match (os, arch) {
        ("macos", "aarch64") => Ok("clickup-cli-aarch64-apple-darwin".to_owned()),
        ("macos", "x86_64") => Ok("clickup-cli-x86_64-apple-darwin".to_owned()),
        _ => Err(io::Error::other(
            "GitHub release installs are configured for macOS only. Use `cargo install clickup-cli` on other platforms.",
        )),
    }
}

fn binary_name() -> &'static str {
    if cfg!(windows) {
        "clickup-cli.exe"
    } else {
        "clickup-cli"
    }
}

fn default_install_dir() -> io::Result<PathBuf> {
    if let Some(path) = env::var_os("CLICKUP_CLI_INSTALL_DIR") {
        return Ok(PathBuf::from(path));
    }

    let home = env::var_os("HOME").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "HOME is not set. Pass --dir to choose an install location.",
        )
    })?;

    Ok(PathBuf::from(home).join(".local/bin"))
}

fn write_binary(destination: &Path, bytes: &[u8]) -> io::Result<()> {
    let temporary = temporary_download_path(destination);
    fs::write(&temporary, bytes)?;
    set_executable_permissions(&temporary)?;

    if destination.exists() {
        fs::remove_file(destination)?;
    }

    fs::rename(temporary, destination)
}

fn temporary_download_path(destination: &Path) -> PathBuf {
    let file_name = destination
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("clickup-cli");

    destination.with_file_name(format!("{file_name}.download"))
}

#[cfg(unix)]
fn set_executable_permissions(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
}

#[cfg(not(unix))]
fn set_executable_permissions(_: &Path) -> io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use clickup_openapi::TaskReference;

    use super::{release_asset_name_for, resolve_task_reference};

    const TEST_WORKSPACE_ID: u64 = 42;
    const TEST_CUSTOM_TASK_ID: &str = "TASK-123";

    #[test]
    fn hyphenated_task_ids_are_treated_as_custom_ids() {
        let reference =
            resolve_task_reference(TEST_WORKSPACE_ID, TEST_CUSTOM_TASK_ID.to_owned(), false);

        assert_eq!(
            reference,
            TaskReference::custom_id(TEST_WORKSPACE_ID, TEST_CUSTOM_TASK_ID)
        );
    }

    #[test]
    fn numeric_task_ids_stay_as_regular_task_ids() {
        let reference = resolve_task_reference(TEST_WORKSPACE_ID, "8675309".to_owned(), false);

        assert_eq!(reference, TaskReference::id("8675309"));
    }

    #[test]
    fn mac_arm_asset_names_match_release_workflow() {
        let asset_name = release_asset_name_for("macos", "aarch64").unwrap();

        assert_eq!(asset_name, "clickup-cli-aarch64-apple-darwin");
    }

    #[test]
    fn non_macos_release_assets_are_rejected() {
        let error = release_asset_name_for("linux", "x86_64").unwrap_err();

        assert!(error.to_string().contains("macOS only"));
    }
}
