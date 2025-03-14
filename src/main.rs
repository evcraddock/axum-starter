mod health;
mod clients;

use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber::FmtSubscriber;

// Build the application router
pub fn app() -> Router {
    Router::new()
        .merge(health::routes::routes())
        .merge(clients::routes::routes())
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

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
    async fn test_health_endpoint() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_clients_endpoint() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/clients").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
