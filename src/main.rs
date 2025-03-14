mod health;
mod clients;
mod config;

use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

// Build the application router
pub fn app() -> Router {
    Router::new()
        // Original routes
        .merge(health::routes::routes())
        .merge(clients::routes::routes())
        // API routes with proper nesting
        .nest("/api", api_routes())
}

// Define API routes
fn api_routes() -> Router {
    Router::new()
        .nest("/health", health::routes::api_routes())
        .nest("/clients", clients::routes::api_routes())
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
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::util::ServiceExt;

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

        let response = app
            .oneshot(Request::builder().uri("/api/clients").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
