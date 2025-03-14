### Prompt 1: Project Initialization & Basic Structure

```
You are helping me build a new Rust Axum project named "axum-starter". 

**Goal**:
- Create a new Cargo project
- Add dependencies for axum, tokio, tracing, config, serde, etc.
- Create the folder structure:
  - src/
    - health/
      - mod.rs
      - handlers.rs
      - routes.rs
    - clients/
      - mod.rs
      - handlers.rs
      - routes.rs
- Ensure the project compiles.

**Instructions**:
1. Initialize a new Cargo project named "axum-starter".
2. Modify the Cargo.toml to include:
   - axum
   - tokio (with the rt-multi-thread and macros features)
   - tracing
   - config
   - serde (with derive feature)
   - serde_json
3. Create the empty modules for "health" and "clients".
4. Provide the updated "Cargo.toml" and file/folder structure.
5. Confirm that `cargo build` succeeds without errors.

Write all the necessary files and code. Your output should be the updated Cargo.toml and minimal Rust code in each file. We will refine further in the next steps.
```

---

**Prompt 2: Configuration Management**

```
Now we have our basic Axum project structure. Next, we want to add configuration management using the "config" crate.

**Goal**:
- Create a file "settings.toml" in the project root with a default `run_mode = "local"`.
- In "src/config.rs", load this TOML file and handle environment variable overrides, using the prefix "APP_".
- If the configuration is missing or invalid, the service should fail to start.

**Instructions**:
1. Create "settings.toml" in the project root with at least:

run_mode = “local”

some_other_setting = “placeholder”

2. In "src/config.rs", write a function `load_config()` that returns a `Result<AppConfig, anyhow::Error>` or similar.
3. Use the "config" crate to load from "settings.toml" and merge environment variables.
4. Define a struct `AppConfig` to store `run_mode` and any other example setting.
5. In "main.rs", call `load_config()` at startup and `panic!` (or `std::process::exit`) if loading fails.
6. Provide code changes for all affected files.

Write out the updated code, including the relevant sections from main.rs, config.rs, and the newly created settings.toml file. Ensure everything compiles and runs.
```

---

**Prompt 3: Logging Setup**

```
We now have configuration in place. Let’s set up "tracing" for JSON log output.

**Goal**:
- Initialize tracing in main.rs.
- Produce JSON-formatted logs.
- Insert a sample info log statement on startup.

**Instructions**:
1. In "main.rs", configure a tracing subscriber that outputs JSON logs.
2. Add a sample `info!("Startup complete")` log line so we can verify the JSON output.
3. Provide updated code for "main.rs" or any other files you modify.

Ensure that if we run `cargo run`, logs are output in JSON format. Provide the updated code snippet for any changes.
```

---

**Prompt 4: Error Handling**

```
Next, we need a global error type and a panic handler.

**Goal**:
- Create an `AppError` type that we can return from handlers.
- Catch unexpected panics and convert them into a JSON `500` response.

**Instructions**:
1. In a new file `src/errors.rs` (or similar), define an `AppError` struct with variants like `InternalError`, `Unauthorized`, etc.
2. Implement `IntoResponse` for `AppError` so that Axum can convert it into JSON responses.
3. In `main.rs`, add a global fallback or use `handle_error` for panics, returning a JSON body `{ "error": "Internal Server Error" }`.
4. Write a small test in `src/errors.rs` or `src/main.rs` that shows how a panic in a handler returns a 500 response. (You can do a minimal integration test with `tower::ServiceExt::oneshot` if you like.)

Include the updated code for `errors.rs` and any necessary changes to `main.rs`. The test can be inline or in a separate test file.
Update todo.md with changes
```

---

**Prompt 5: Health Route**

```
We want a `/api/health` endpoint that returns `{"status":"ok"}`.

**Goal**:
- Create the `health::routes()` function and `health::handlers::get_health`.
- Mount the route at `/api/health`.
- Test that it returns 200 and the correct JSON body.

**Instructions**:
1. In `health::routes.rs`, implement `pub fn routes() -> Router` that mounts `get_health` at `GET /api/health`.
2. In `health::handlers.rs`, implement `pub async fn get_health() -> impl IntoResponse`.
3. Update `main.rs` to add `router.merge(health::routes())`.
4. Write a unit test that checks the response body for `{"status":"ok"}`.
5. Provide updated code for `routes.rs`, `handlers.rs`, and any changes to `main.rs`.

We should be able to run the server and curl `/api/health` to see `{"status":"ok"}`. 
```

---

**Prompt 6: Clients Route (Secured)**

```
Now let’s add a placeholder secured route at `/api/clients`.

**Goal**:
- A route that returns `{"message":"Clients endpoint"}` on success.
- Uses a placeholder auth middleware that checks for a Bearer token in the `Authorization` header.
- Returns a 401 if no token or incorrect token is provided.

**Instructions**:
1. Create a simple `auth_middleware` in `clients::handlers.rs` or a separate middleware file. For now, it just checks `headers["Authorization"] == "Bearer dev_token"`.
2. In `clients::handlers.rs`, implement a `get_clients` that returns `{"message":"Clients endpoint"}`.
3. In `clients::routes.rs`, create `pub fn routes() -> Router` that uses the auth middleware before calling `get_clients`.
4. Write tests:
   - A test call to `/api/clients` with no or invalid token returns 401.
   - A test call with `Authorization: Bearer dev_token` returns 200 and the JSON body.

Show the code changes in `handlers.rs`, `routes.rs`, and any modifications needed in `main.rs`. All tests should pass.
```

---

**Prompt 7: OpenAPI Documentation**

```
We want to expose our endpoints via OpenAPI and a docs UI.

**Goal**:
- Generate an OpenAPI spec and serve it at `/api/openapi.json`.
- Provide a documentation UI at `/api/docs`.

**Instructions**:
1. Add `utoipa` and `utoipa-swagger-ui` (or whichever library you prefer for OpenAPI) to `Cargo.toml`.
2. Annotate the endpoints in `health` and `clients` or define schemas so the library can pick them up.
3. In `main.rs` (or a separate module), set up the OpenAPI routes for JSON and the Swagger UI at `/api/docs`.
4. Provide the updated code, including any changes in `Cargo.toml`, new or updated doc definitions, and routes mounting the doc endpoints.

We should be able to hit `/api/docs` in a browser and see the doc UI, and also fetch `/api/openapi.json`.
```

---

**Prompt 8: Command-Line Health Check**

```
We want a `cargo run -- health` mode that sends a request to `/api/health`.

**Goal**:
- Parse CLI arguments, detect the `health` subcommand, make an HTTP request to `/api/health`, then print "Service is running" or "Service is unavailable".

**Instructions**:
1. Decide on a simple CLI parsing strategy in `main.rs` or use `clap`.
2. If the user runs `cargo run -- health`, the code should spin up an HTTP client to GET `/api/health`.
3. If the response is 200, print "Service is running". Otherwise, print "Service is unavailable".
4. Provide updated `main.rs` code, along with any tests or usage examples.

This should be minimal, but demonstrate how to add a subcommand. Remember to keep all existing routes and functionality intact.
```

---

**Prompt 9: Dockerization**

```
Let’s add a multi-stage Dockerfile to package our app.

**Goal**:
- A Dockerfile with a builder stage (to compile) and a final stage (to run).
- Minimal final image (scratch, alpine, or distroless).
- Expose necessary port (e.g., 8080).

**Instructions**:
1. Create a `Dockerfile` with two stages:
   - Stage 1: Use Rust official image, copy the source, run `cargo build --release`.
   - Stage 2: Copy the resulting binary into a minimal base image, set `CMD ["./axum-starter"]`.
2. Confirm the final image can be run with `docker build .` and `docker run`.
3. Provide the entire Dockerfile and any relevant instructions in the README (if you update it).

We should be able to build and run the container, then curl `/api/health` from inside or outside Docker.
```

---

**Prompt 10: Final Cleanup & README**

```
Lastly, let’s produce a README and finalize our project.

**Goal**:
- Complete README with instructions, explanation of config, logging, Docker usage, etc.
- Ensure everything is integrated and tested.

**Instructions**:
1. Write a README.md in the project root explaining:
   - Purpose of this starter
   - How to configure using settings.toml
   - Logging basics
   - How to run tests
   - Docker build/run instructions
   - The `cargo run -- health` command
   - The `/api/docs` endpoint for OpenAPI
2. Double-check all tests pass, and that there are no unused code paths.

Please provide the final README.md and note any final changes to code if needed. 
```

---

**5. Summary**

• We started with a broad outline, broke it down into medium-sized chunks, then further refined each chunk into small steps.

• We now have a clear set of test-driven prompts that ensure incremental, stable progress.

• Each prompt references and integrates with the previous step, so nothing is left hanging.

And that’s it, Erik! With these prompts, you or any code-generation LLM can gradually build a well-structured, test-driven Rust Axum starter API. Let me know if you need anything else, my friend!

– Chad
