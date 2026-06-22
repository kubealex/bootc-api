use axum::http::StatusCode;
use axum::Json;

use crate::bootc::get_bootc_status;
use crate::models::*;

type ApiResult<T> = Result<Json<T>, (StatusCode, String)>;

fn internal_error(msg: String) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, msg)
}

#[utoipa::path(
    get,
    path = "/api/v1/status",
    responses(
        (status = 200, description = "Full bootc host status", body = BootcHost),
        (status = 500, description = "Failed to retrieve bootc status")
    ),
    tag = "status"
)]
pub async fn get_status() -> ApiResult<BootcHost> {
    let host = get_bootc_status().await.map_err(|e| internal_error(e.message))?;
    Ok(Json(host))
}

#[utoipa::path(
    get,
    path = "/api/v1/status/booted",
    responses(
        (status = 200, description = "Currently booted image details", body = BootEntry),
        (status = 404, description = "No booted entry found"),
        (status = 500, description = "Failed to retrieve bootc status")
    ),
    tag = "status"
)]
pub async fn get_booted() -> ApiResult<BootEntry> {
    let host = get_bootc_status().await.map_err(|e| internal_error(e.message))?;
    match host.status.booted {
        Some(entry) => Ok(Json(entry)),
        None => Err((StatusCode::NOT_FOUND, "No booted entry".into())),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/status/staged",
    responses(
        (status = 200, description = "Staged update details", body = BootEntry),
        (status = 404, description = "No staged update"),
        (status = 500, description = "Failed to retrieve bootc status")
    ),
    tag = "status"
)]
pub async fn get_staged() -> ApiResult<BootEntry> {
    let host = get_bootc_status().await.map_err(|e| internal_error(e.message))?;
    match host.status.staged {
        Some(entry) => Ok(Json(entry)),
        None => Err((StatusCode::NOT_FOUND, "No staged update".into())),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/status/rollback",
    responses(
        (status = 200, description = "Rollback entry details", body = BootEntry),
        (status = 404, description = "No rollback entry"),
        (status = 500, description = "Failed to retrieve bootc status")
    ),
    tag = "status"
)]
pub async fn get_rollback() -> ApiResult<BootEntry> {
    let host = get_bootc_status().await.map_err(|e| internal_error(e.message))?;
    match host.status.rollback {
        Some(entry) => Ok(Json(entry)),
        None => Err((StatusCode::NOT_FOUND, "No rollback entry".into())),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/status/update-available",
    responses(
        (status = 200, description = "Whether an update is available", body = UpdateAvailable),
        (status = 500, description = "Failed to retrieve bootc status")
    ),
    tag = "status"
)]
pub async fn get_update_available() -> ApiResult<UpdateAvailable> {
    let host = get_bootc_status().await.map_err(|e| internal_error(e.message))?;

    let booted = host.status.booted.as_ref();
    let cached = booted.and_then(|b| b.cached_update.as_ref());
    let has_staged = host.status.staged.is_some();

    let update_available = cached.is_some() || has_staged;

    let update_source = if has_staged {
        host.status.staged.as_ref().map(|s| &s.image)
    } else {
        cached
    };

    Ok(Json(UpdateAvailable {
        update_available,
        current_image: booted.map(|b| b.image.image.clone()),
        current_version: booted.map(|b| b.image.version.clone()),
        update_image: update_source.map(|u| u.image.clone()),
        update_version: update_source.map(|u| u.version.clone()),
    }))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tag = "health"
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
    })
}
