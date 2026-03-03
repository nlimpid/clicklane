# Clicklane

Clicklane is a Tauri 2 desktop client for focused ClickUp work. The repository is split into a small Rust workspace so the shared API client, CLI, and macOS app release pipeline stay aligned:

- `crates/clickup`: published Rust SDK crate for ClickUp auth, task tracking, and move-task workflows.
- `crates/clickup-cli`: CLI for task inspection plus GitHub-release based install and update flows.
- `src-tauri`: the desktop app bundle that ships as `Clicklane` for macOS.

The frontend remains the same React application with `Task focus`, `For you`, and settings flows, but the release metadata now targets the public `nlimpid/clicklane` repository.

## Stack

- Bun + Vite + React + TypeScript
- Tailwind CSS v4
- Base UI primitives (`@base-ui/react`)
- COSS-inspired design tokens (no shadcn dependency)
- Zustand state management
- Vercel AI SDK (`ai` + `@ai-sdk/openai`)
- Vitest
- Oxc toolchain: `oxfmt`, `oxlint`, `oxlint-tsgolint`
- Tauri 2.0

## Local setup

1. Install dependencies:

```bash
bun install
```

2. Set env variables:

```bash
cp .env.example .env
```

3. Start web mode:

```bash
bun run dev
```

4. Start desktop mode:

```bash
bun run tauri:dev
```

5. Run the Rust CLI against ClickUp:

```bash
source .env
cargo run -p clickup-cli -- task --workspace-id "$VITE_CLICKUP_TEST_WORKSPACE_ID" --task-id "$VITE_CLICKUP_TEST_TASK_ID" --custom-id
```

## Install and Update `clickup-cli`

Once `clickup-cli` is published to crates.io, install it from source with:

```bash
cargo install clickup-cli
```

For the GitHub Releases distribution that this repository now builds for macOS:

```bash
cargo run -p clickup-cli -- install --dir ~/.local/bin
clickup-cli update
```

The `install` and `update` commands download the latest macOS binary from [nlimpid/clicklane](https://github.com/nlimpid/clicklane/releases).

## Env vars

- `VITE_CLICKUP_OPENAI_TOKEN`: OpenAI token used by Vercel AI SDK in the current prototype.
- `VITE_CLICKUP_TEST_WORKSPACE_ID`: Local-only workspace ID used to prefill the desktop app and CLI examples.
- `VITE_CLICKUP_TEST_TASK_ID`: Local-only task ID used to prefill the task lookup form and CLI examples.
- `CLICKUP_API_TOKEN`: Personal ClickUp API key (`Authorization: pk_...`).
- `CLICKUP_ACCESS_TOKEN`: OAuth access token (`Authorization: Bearer ...`).
- `CLICKUP_API_BASE_URL`: Optional override for the ClickUp API host.

## Current scope

- Basic app shell
- `For you` workspace UI
- Mock task stream + tab filters
- AI summary action with fallback when token is missing or request fails
- Shared Rust ClickUp SDK workspace for future backend integration
- GitHub Actions for CI, macOS app releases, and crates.io publishing
