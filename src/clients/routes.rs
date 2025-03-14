use axum::{
    routing::get,
    Router,
};

use super::handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/clients", get(handlers::get_clients))
}

pub fn api_routes() -> Router {
    // This router doesn't include authentication yet - 
    // Authentication will be added in main.rs
    Router::new()
        .route("/", get(handlers::get_secured_clients))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::util::ServiceExt;
    use super::handlers::Client;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn test_clients_route() {
        let app = routes();

        let response = app
            .oneshot(Request::builder().uri("/clients").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let clients: Vec<Client> = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(clients.len(), 1);
        assert_eq!(clients[0].id, "1");
        assert_eq!(clients[0].name, "Example Client");
    }
    
    #[tokio::test]
    async fn test_api_clients_route() {
        let app = api_routes();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_data: handlers::ClientResponse = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(response_data.message, "Clients endpoint");
    }
}