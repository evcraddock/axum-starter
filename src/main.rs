mod health;
mod clients;
mod config;

use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber::FmtSubscriber;

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
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");
        
    // Load configuration
    let config = config::load_config().unwrap_or_else(|err| {
        eprintln!("Failed to load configuration: {}", err);
        std::process::exit(1);
    });
    
    tracing::info!("Application configuration loaded: run_mode={}", config.run_mode);

    // Build our application
    let app = app();

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
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
