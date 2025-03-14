use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, PartialEq, ToSchema)]
pub struct Client {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ToSchema)]
pub struct ClientResponse {
    pub message: String,
}

/// Get clients endpoint (secured)
/// 
/// This endpoint requires authentication with a valid Bearer token.
#[utoipa::path(
    get,
    path = "/api/clients",
    tag = "clients",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Successfully retrieved clients message", body = ClientResponse),
        (status = 401, description = "Unauthorized - Missing or invalid token")
    )
)]
pub async fn get_secured_clients() -> Json<ClientResponse> {
    Json(ClientResponse {
        message: "Clients endpoint".to_string(),
    })
}

/// Get client list
/// 
/// Returns a list of clients.
#[utoipa::path(
    get,
    path = "/clients",
    tag = "clients",
    responses(
        (status = 200, description = "Successfully retrieved client list", body = Vec<Client>),
    )
)]
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