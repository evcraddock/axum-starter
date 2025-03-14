use axum::{
    routing::get,
    Router,
};

use super::handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(handlers::get_health))
}

pub fn api_routes() -> Router {
    Router::new()
        .route("/", get(handlers::get_health))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    // Use the health response struct
    use tower::util::ServiceExt;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn test_health_route() {
        let app = routes();

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: handlers::HealthResponse = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(body.status, "ok");
    }
    
    #[tokio::test]
    async fn test_api_health_route() {
        let app = api_routes();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: handlers::HealthResponse = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(body.status, "ok");
    }
}