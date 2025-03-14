use axum::{
    routing::get,
    Router,
};

use super::handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::json;
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
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(body, json!({"status": "UP"}));
    }
}