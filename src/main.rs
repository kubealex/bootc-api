mod bootc;
mod models;
mod routes;

use axum::Router;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::*;
use crate::routes::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_status,
        get_booted,
        get_staged,
        get_rollback,
        get_update_available,
        health,
    ),
    components(schemas(
        BootcHost,
        Metadata,
        Spec,
        ImageRef,
        Status,
        BootEntry,
        ImageStatus,
        OstreeStatus,
        UpdateAvailable,
        HealthResponse,
    )),
    info(
        title = "Bootc Status API",
        version = "0.1.0",
        description = "REST API for querying bootc system status, updates, and rollback information"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/v1/status", axum::routing::get(get_status))
        .route("/api/v1/status/booted", axum::routing::get(get_booted))
        .route("/api/v1/status/staged", axum::routing::get(get_staged))
        .route("/api/v1/status/rollback", axum::routing::get(get_rollback))
        .route(
            "/api/v1/status/update-available",
            axum::routing::get(get_update_available),
        )
        .route("/health", axum::routing::get(health))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = TcpListener::bind("0.0.0.0:8005").await.unwrap();
    println!("bootc-api listening on port 8005");
    axum::serve(listener, app).await.unwrap();
}
