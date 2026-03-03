# Contributing

## Setup

1. Install Bun and the Rust toolchain.
2. Run `bun install`.
3. Run `cargo test --workspace`.
4. Run `bun run tauri:build -- --bundles app` before opening a release PR.

## Pull Requests

- Keep the Rust workspace, CLI, and Tauri metadata version-aligned.
- Update README examples when changing the CLI surface.
- Do not commit secrets, production tokens, or local `.env` files.
- Prefer small, reviewable changes with tests for Rust logic.
