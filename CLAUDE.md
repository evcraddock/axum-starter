# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands
- Build: `cargo build`
- Run: `cargo run`
- Test all: `cargo test`
- Test single: `cargo test <test_name>` (e.g., `cargo test test_health_check`)
- Test module: `cargo test --package axum-starter <module>::` (e.g., `cargo test --package axum-starter health::handlers::tests::`)
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Style Guidelines
- **Imports**: Group standard library, external crates, and internal modules separately
- **Error Handling**: Use `AppError` from `errors.rs` for all API responses
- **Testing**: Each module should have unit tests in a `tests` submodule
- **Types**: Use Rust's strong typing; create domain-specific types with `struct`
- **Naming**: Use snake_case for functions, variables; CamelCase for types
- **API Documentation**: Use doc comments (`///`) and utoipa annotations for OpenAPI docs
- **Modules**: Organize code by domain with handlers, routes, and mod files
- **Authentication**: Secured endpoints require valid Bearer token through auth middleware
- **Logging**: Use tracing for structured JSON logs via `tracing::info!`, etc.
- **Config**: Application config loaded from settings.toml and/or environment variables