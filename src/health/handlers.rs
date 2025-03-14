use axum::Json;
use serde_json::json;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ "status": "UP" }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let result = health_check().await;
        let json = result.0;
        
        assert_eq!(json["status"], "UP");
    }
}