#![allow(dead_code)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub message: String,
}

impl AppError {
    // Constructor functions for various error types
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            message: message.into(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error ({}): {}", self.status.as_u16(), self.message)
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": {
                "status": self.status.as_u16(),
                "message": self.message
            }
        }));

        (self.status, body).into_response()
    }
}

// Helper function to convert any error into an AppError
pub fn internal_error<E>(err: E) -> AppError
where
    E: std::error::Error,
{
    tracing::error!("Internal error: {:?}", err);
    AppError::internal_error(format!("Internal server error: {}", err))
}

// Function to handle panics and convert them to a structured error response
pub fn handle_panic(err: Box<dyn std::any::Any + Send + 'static>) -> Response {
    let message = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic occurred".to_string()
    };

    tracing::error!("Panic occurred: {}", message);

    let body = Json(json!({
        "error": {
            "status": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "message": "Internal server error"
        }
    }));

    (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::Value;
    use tower::ServiceExt;

    // A handler that returns an AppError
    async fn error_handler() -> Result<&'static str, AppError> {
        Err(AppError::bad_request("Invalid request"))
    }

    #[tokio::test]
    async fn test_app_error_response() {
        // Create a test router with an endpoint that returns an AppError
        let app = Router::new().route("/error", get(error_handler));

        // Send a request to the endpoint
        let response = app
            .oneshot(Request::builder().uri("/error").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Check the status code
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // Extract the response body
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();

        // Check the error structure
        assert_eq!(body["error"]["status"], 400);
        assert_eq!(body["error"]["message"], "Invalid request");
    }
}