# Axum Starter API

A starter template for building production-ready REST APIs with Axum, Tokio, and Rust.

## Features

- **Modern Rust Web Framework**: Built on Axum, a lightweight and fast web framework
- **Async Runtime**: Powered by Tokio for high-performance async I/O
- **Structured JSON Logging**: Configurable logging with tracing
- **Configuration Management**: Uses TOML files and environment variables
- **Error Handling**: Custom AppError with consistent JSON responses
- **Authentication**: Simple token-based auth middleware
- **Health Checks**: Both API and CLI-based health checks
- **OpenAPI Documentation**: Auto-generated from code annotations
- **Docker Support**: Multi-stage builds for minimal container size
- **Module Organization**: Clean separation of routes, handlers, and business logic
- **Testing**: Comprehensive test suite for all components

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable recommended)
- Docker (optional, for containerization)

### Building and Running

```bash
# Build the project
cargo build

# Run the server
cargo run

# Run the server in release mode
cargo run --release
```

The server will start on http://localhost:3000.

### Running Health Check CLI

```bash
# Start the server in one terminal
cargo run

# In another terminal, run the health check
cargo run -- health
```

The health check will output "Service is running" if the API is available, or "Service is unavailable" with an error message if not.

## API Endpoints

- **Health Check**: `GET /health` or `GET /api/health`
- **Clients List**: `GET /clients`
- **Secured Clients Endpoint**: `GET /api/clients` (requires authentication)
- **API Documentation**: `GET /api/docs`
- **OpenAPI JSON**: `GET /api/openapi.json`

## Authentication

Protected routes require a Bearer token in the Authorization header:

```
Authorization: Bearer dev_token
```

In a real application, you would replace this with proper JWT or OAuth authentication.

## Configuration

Configuration is loaded from:

1. `settings.toml` file in the project root
2. Environment variables (override TOML settings)

Example `settings.toml`:
```toml
run_mode = "local"
some_other_setting = "placeholder"
```

Environment variables can override these settings by using the `APP_` prefix:
```bash
export APP_RUN_MODE=production
export APP_SOME_OTHER_SETTING=value
```

## Logging

The application uses structured JSON logging with the following features:

- Log levels configurable via environment variables
- Request logging for debugging
- Structured JSON format for better parsing
- UTC timestamps in RFC3339 format

Set the log level with the `RUST_LOG` environment variable:
```bash
export RUST_LOG=axum_starter=debug,tower_http=debug
```

## Testing

Run the test suite with:

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test --package axum-starter health::handlers::tests

# Run a specific test
cargo test test_health_check
```

## Docker

### Building

```bash
docker build -t axum-starter .
```

### Running

```bash
# Run container with port mapping
docker run -p 3000:3000 axum-starter
```

### Testing

```bash
# Test the API from outside the container
curl http://localhost:3000/api/health
```

## Project Structure

```
axum-starter/
├── Cargo.toml             # Project dependencies
├── Dockerfile             # Docker build instructions
├── settings.toml          # Application configuration
├── src/
│   ├── main.rs            # Application entry point
│   ├── auth.rs            # Authentication middleware
│   ├── config.rs          # Configuration loading
│   ├── errors.rs          # Error handling
│   ├── openapi.rs         # OpenAPI documentation
│   ├── health/            # Health check endpoints
│   │   ├── mod.rs
│   │   ├── handlers.rs    # Request handlers
│   │   └── routes.rs      # Route definitions
│   └── clients/           # Client management endpoints
│       ├── mod.rs
│       ├── handlers.rs    # Request handlers
│       └── routes.rs      # Route definitions
```

## Development Practices

- **Error Handling**: Always return structured JSON errors using the AppError type
- **Testing**: Write unit tests for all handlers and middleware
- **Documentation**: Document all public APIs with rustdoc and OpenAPI annotations
- **Configuration**: Add new settings to both AppConfig struct and settings.toml

## License

This project is available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.