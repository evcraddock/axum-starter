use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

// Expected token for development
pub const DEV_TOKEN: &str = "Bearer dev_token";

// Auth middleware that checks for a valid Bearer token
pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    // Check if header exists and matches our expected token
    match auth_header {
        Some(token) if token == DEV_TOKEN => {
            // Token is valid, proceed to handler
            Ok(next.run(request).await)
        }
        _ => {
            // Return 401 Unauthorized if token is missing or invalid
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
        routing::get,
        middleware,
    };
    use tower::ServiceExt;

    // Test handler that returns a simple response
    async fn test_handler() -> &'static str {
        "Protected content"
    }

    #[tokio::test]
    async fn test_auth_middleware_no_token() {
        // Create a test app with auth middleware
        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(middleware::from_fn(auth_middleware));

        // Send a request without an auth token
        let response = app
            .oneshot(Request::builder().uri("/protected").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Should get 401 Unauthorized
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_auth_middleware_invalid_token() {
        // Create a test app with auth middleware
        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(middleware::from_fn(auth_middleware));

        // Send a request with wrong token
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header(header::AUTHORIZATION, "Bearer wrong_token")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        // Should get 401 Unauthorized
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_auth_middleware_valid_token() {
        // Create a test app with auth middleware
        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(middleware::from_fn(auth_middleware));

        // Send a request with valid token
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header(header::AUTHORIZATION, DEV_TOKEN)
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        // Should get 200 OK
        assert_eq!(response.status(), StatusCode::OK);
    }
}