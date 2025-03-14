# Axum Starter API - To Do

## 1. Project Initialization & Basic Structure

- [x] **1.1**: Run `cargo new axum-starter` to create the project.
- [x] **1.2**: Update `Cargo.toml` with dependencies (`axum`, `tokio`, `tracing`, `config`, `serde`, `serde_json`).
- [x] **1.3**: Create the folder structure:
  - `src/health/` with `mod.rs`, `handlers.rs`, `routes.rs`
  - `src/clients/` with `mod.rs`, `handlers.rs`, `routes.rs`
- [x] **1.4**: Ensure the project compiles via `cargo build`.
- [x] **1.5**: Confirm basic runtime with `cargo run`.

---

## 2. Configuration Management

- [x] **2.1**: Add a `settings.toml` in the project root (with `run_mode = "local"` and a placeholder setting).
- [x] **2.2**: Implement a `config.rs` module to load these settings and merge environment variables (`APP_*` overrides).
- [x] **2.3**: Define an `AppConfig` struct for storing loaded settings.
- [x] **2.4**: Make the service exit if config is missing or invalid.

---

## 3. Logging Setup

- [x] **3.1**: Initialize `tracing` in `main.rs` for JSON log output.
- [x] **3.2**: Configure log levels for different environments (development/production).
- [x] **3.3**: Add a sample `info!("Startup complete")` to confirm JSON logging works.

---

## 4. Error Handling

- [x] **4.1**: Create an `AppError` struct in `src/errors.rs`.
- [x] **4.2**: Implement `IntoResponse` for `AppError` to return structured JSON errors.
- [x] **4.3**: Add a global panic/fallback handler that returns a 500 JSON response for unexpected errors.
- [x] **4.4**: Write a test that induces a panic and checks for a 500 response.

---

## 5. Health Route

- [x] **5.1**: In `health::routes.rs`, implement `pub fn routes() -> Router`.
- [x] **5.2**: In `health::handlers.rs`, implement `get_health` to return `{"status":"ok"}`.
- [x] **5.3**: Update `main.rs` to mount `/api/health` on the router.
- [x] **5.4**: Write a unit test to confirm a 200 response with `{"status":"ok"}`.

---

## 6. Clients Route (Secured)

- [ ] **6.1**: Add a placeholder `auth_middleware` that checks for a Bearer token in the `Authorization` header.
- [x] **6.2**: In `clients::routes.rs`, mount `/api/clients` behind the auth middleware.
- [x] **6.3**: In `clients::handlers.rs`, create `get_clients` returning `{"message":"Clients endpoint"}`.
- [x] **6.4**: Test 401 response if missing/invalid token, and 200 if valid token.

---

## 7. OpenAPI Documentation

- [ ] **7.1**: Add libraries (`utoipa`, `utoipa-swagger-ui`) for OpenAPI generation.
- [ ] **7.2**: Annotate endpoints/structs for doc generation or define schemas.
- [ ] **7.3**: Serve OpenAPI JSON at `/api/openapi.json` and Swagger UI at `/api/docs`.
- [ ] **7.4**: Test that `/api/docs` is accessible.

---

## 8. Command-Line Health Check

- [ ] **8.1**: Parse CLI args (e.g., using `clap` or `std::env::args`) to detect `-- health`.
- [ ] **8.2**: If subcommand is `health`, make an HTTP request to `/api/health`.
- [ ] **8.3**: Print "Service is running" on 200, else "Service is unavailable".
- [ ] **8.4**: Test or manually verify the output.

---

## 9. Dockerization

- [ ] **9.1**: Create a multi-stage `Dockerfile`.
  - Stage 1: Use Rust official image, build the project.
  - Stage 2: Copy binary into minimal base image.
- [ ] **9.2**: Expose necessary port (e.g. 8080) and set `CMD ["./axum-starter"]`.
- [ ] **9.3**: Verify `docker build .` and `docker run` works; test `/api/health`.

---

## 10. Final Cleanup & README

- [ ] **10.1**: Write a README.md explaining:
  - Project purpose
  - Configuration usage
  - Logging
  - Tests
  - Docker instructions
  - CLI health check
  - `/api/docs` for OpenAPI
- [ ] **10.2**: Confirm all tests pass, no unused code or modules.
- [ ] **10.3**: Optionally create a release or tag in Git if desired.

---
