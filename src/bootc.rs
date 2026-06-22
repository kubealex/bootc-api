use crate::models::BootcHost;
use tokio::process::Command;

#[derive(Debug)]
pub struct BootcError {
    pub message: String,
}

impl std::fmt::Display for BootcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub async fn get_bootc_status() -> Result<BootcHost, BootcError> {
    let output = Command::new("bootc")
        .args(["status", "--json"])
        .output()
        .await
        .map_err(|e| BootcError {
            message: format!("Failed to execute bootc: {e}"),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(BootcError {
            message: format!("bootc exited with {}: {stderr}", output.status),
        });
    }

    serde_json::from_slice(&output.stdout).map_err(|e| BootcError {
        message: format!("Failed to parse bootc output: {e}"),
    })
}
