use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Client {
    pub id: String,
    pub name: String,
}

pub async fn get_clients() -> Json<Vec<Client>> {
    Json(vec![
        Client {
            id: "1".to_string(),
            name: "Example Client".to_string(),
        }
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_clients() {
        let result = get_clients().await;
        let clients = result.0;
        
        assert_eq!(clients.len(), 1);
        assert_eq!(clients[0], Client {
            id: "1".to_string(),
            name: "Example Client".to_string(),
        });
    }
}