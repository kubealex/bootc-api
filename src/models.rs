use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BootcHost {
    pub api_version: String,
    pub kind: String,
    pub metadata: Metadata,
    pub spec: Spec,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Metadata {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub boot_order: String,
    pub image: ImageRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImageRef {
    pub image: String,
    pub transport: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub booted: Option<BootEntry>,
    pub rollback: Option<BootEntry>,
    pub rollback_queued: bool,
    pub staged: Option<BootEntry>,
    #[serde(rename = "type")]
    pub status_type: String,
    pub usr_overlay: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BootEntry {
    pub cached_update: Option<ImageStatus>,
    pub composefs: Option<serde_json::Value>,
    pub download_only: bool,
    pub image: ImageStatus,
    pub incompatible: bool,
    pub ostree: OstreeStatus,
    pub pinned: bool,
    pub soft_reboot_capable: bool,
    pub store: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImageStatus {
    pub architecture: String,
    pub image: ImageRef,
    pub image_digest: String,
    pub timestamp: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OstreeStatus {
    pub checksum: String,
    pub deploy_serial: u32,
    pub stateroot: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateAvailable {
    pub update_available: bool,
    pub current_image: Option<ImageRef>,
    pub current_version: Option<String>,
    pub update_image: Option<ImageRef>,
    pub update_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
}
