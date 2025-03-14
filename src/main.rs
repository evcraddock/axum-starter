mod health;
mod clients;
mod config;
mod errors;
mod auth;
mod openapi;

use std::{net::SocketAddr, panic::AssertUnwindSafe};
use axum::{
    Router, 
    response::IntoResponse,
    middleware::{self, Next},
    extract::Request,
    http::StatusCode,
};
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

use crate::errors::{AppError, handle_panic};

// Fallback handler for 404 errors
async fn handle_404() -> impl IntoResponse {
    AppError::not_found("Route not found")
}

// Middleware to catch panics and return 500 errors
async fn panic_handler(
    req: Request,
    next: Next,
) -> Result<axum::response::Response, (StatusCode, String)> {
    // Use a closure that we can catch panics from
    let response = std::panic::catch_unwind(AssertUnwindSafe(move || async move {
        next.run(req).await
    }));

    // Handle the result
    match response {
        Ok(future) => {
            // This runs the future if the closure didn't panic
            match tokio::task::spawn(future).await {
                Ok(response) => Ok(response),
                Err(_) => Ok(handle_panic(Box::new("Task failed"))),
            }
        }
        Err(panic) => {
            // This handles the case where the closure panicked
            Ok(handle_panic(panic))
        }
    }
}

// Build the application router
pub fn app() -> Router {
    Router::new()
        // Original routes
        .merge(health::routes::routes())
        .merge(clients::routes::routes())
        // API routes with proper nesting
        .nest("/api", api_routes())
        // Add 404 fallback
        .fallback(handle_404)
        // Add middleware with panic recovery
        .layer(middleware::from_fn(panic_handler))
}

// Helper function to create secured routes
fn secured_routes() -> Router {
    Router::new()
        .nest("/clients", clients::routes::api_routes())
        // Additional secured routes can be added here
        // For example:
        // .nest("/admin", admin::routes::api_routes())
        // .nest("/users", users::routes::api_routes())
        .layer(middleware::from_fn(auth::auth_middleware))
}

// Define API routes
fn api_routes() -> Router {
    Router::new()
        // Public routes don't need authentication
        .nest("/health", health::routes::api_routes())
        // Secured routes that require authentication
        .merge(secured_routes())
        // OpenAPI documentation
        .merge(openapi::routes())
}

#[tokio::main]
async fn main() {
    // Initialize JSON tracing
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("axum_starter=debug,tower_http=debug"));
    
    // Configure JSON formatter
    let json_layer = fmt::layer()
        .json()
        .with_timer(fmt::time::UtcTime::rfc_3339())
        .with_current_span(true)
        .with_span_list(true);
    
    // Set up tracing
    tracing_subscriber::registry()
        .with(env_filter)
        .with(json_layer)
        .init();
        
    // Load configuration
    let config = config::load_config().unwrap_or_else(|err| {
        tracing::error!("Failed to load configuration: {}", err);
        std::process::exit(1);
    });
    
    // Log successful configuration load
    tracing::info!(
        run_mode = %config.run_mode,
        some_other_setting = %config.some_other_setting,
        "Application configuration loaded successfully"
    );

    // Build our application
    let app = app();

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!(address = %addr, "Server listening");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    // Log startup complete
    tracing::info!("Startup complete - server ready to accept connections");
    
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use tower::util::ServiceExt;
    use serde_json::Value;
    use axum::http::Request;

    #[tokio::test]
    async fn test_original_health_endpoint() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_original_clients_endpoint() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/clients").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_health_endpoint() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/api/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_clients_endpoint() {
        let app = app();

        // Need to include auth token for secured routes
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/clients")
                    .header("Authorization", auth::DEV_TOKEN)
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
    
    // Test for not found (404) response
    #[tokio::test]
    async fn test_not_found() {
        let app = app();

        // Request to a non-existent endpoint
        let response = app
            .oneshot(Request::builder().uri("/nonexistent").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Check status code
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Extract and check the response body
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();

        // Verify the error structure
        assert_eq!(body["error"]["status"], 404);
        assert_eq!(body["error"]["message"], "Route not found");
    }
    
    // Test secured route with no authentication
    #[tokio::test]
    async fn test_secured_route_no_auth() {
        let app = app();

        // Request to secured endpoint without token
        let response = app
            .oneshot(Request::builder().uri("/api/clients").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Should get 401 Unauthorized
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    // Test secured route with invalid token
    #[tokio::test]
    async fn test_secured_route_invalid_token() {
        let app = app();

        // Request to secured endpoint with invalid token
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/clients")
                    .header("Authorization", "Bearer wrong_token")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        // Should get 401 Unauthorized
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    // Test secured route with valid token
    #[tokio::test]
    async fn test_secured_route_valid_token() {
        let app = app();

        // Request to secured endpoint with valid token
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/clients")
                    .header("Authorization", crate::auth::DEV_TOKEN)
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        // Should get 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Extract and verify the response body
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();

        // Check it contains the expected message
        assert_eq!(body["message"], "Clients endpoint");
    }

    // This is a test of the request 404 handler, not the panic handler
    #[tokio::test]
    async fn test_404_handler() {
        // Create a test app with only the 404 handler
        let app = Router::new().fallback(handle_404);

        // Send a request to a non-existent route
        let response = app
            .oneshot(Request::builder().uri("/nonexistent").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Check status code
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Extract and check the response body
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();

        // Verify the error structure
        assert_eq!(body["error"]["status"], 404);
        assert_eq!(body["error"]["message"], "Route not found");
    }
    
    // Test for OpenAPI docs endpoint
    #[tokio::test]
    async fn test_openapi_docs() {
        let app = app();

        // Request to the OpenAPI UI endpoint
        let response = app
            .oneshot(Request::builder().uri("/api/docs").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Should get a redirect (303) or OK (200) - both are valid
        assert!(
            response.status() == StatusCode::SEE_OTHER || 
            response.status() == StatusCode::OK
        );
    }
    
    // Test for OpenAPI JSON endpoint
    #[tokio::test]
    async fn test_openapi_json() {
        let app = app();

        // Request to the OpenAPI JSON endpoint
        let response = app
            .oneshot(Request::builder().uri("/api/openapi.json").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Should get 200 OK
        assert_eq!(response.status(), StatusCode::OK);
    }
}