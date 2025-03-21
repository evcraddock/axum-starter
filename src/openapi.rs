use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use axum::Router;

/// API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::health::handlers::get_health,
        crate::clients::handlers::get_secured_clients,
        crate::clients::handlers::get_clients
    ),
    components(
        schemas(
            crate::health::handlers::HealthResponse,
            crate::clients::handlers::Client,
            crate::clients::handlers::ClientResponse
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "clients", description = "Client management endpoints")
    ),
    info(
        title = "Axum Starter API",
        version = "0.1.0",
        description = "A starter template for Axum-based REST APIs"
    )
)]
pub struct ApiDoc;

/// Create routes for OpenAPI documentation
pub fn routes() -> Router {
    // Note that when mounted at /api, the full paths will be:
    // - Swagger UI: /api/docs (which redirects to /api/docs/)
    // - OpenAPI JSON: /api/openapi.json
    Router::new()
        // This creates all the necessary routes for Swagger UI
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
}