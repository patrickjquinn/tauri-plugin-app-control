use serde::{Deserialize, Serialize};

/// Options for configuring app exit behavior
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExitOptions {
    /// Whether to remove the app from the recent apps list
    #[serde(default = "default_true")]
    pub remove_from_recents: bool,
    
    /// Whether to forcefully kill the process (use with caution)
    #[serde(default)]
    pub kill_process: bool,
}

fn default_true() -> bool {
    true
}

/// Result of minimizing the app
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimizeResult {
    /// Whether the minimize operation was successful
    pub success: bool,
    
    /// Human-readable message describing the result
    pub message: String,
}

/// Result of closing the app
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseResult {
    /// Whether the close operation was successful
    pub success: bool,
    
    /// Human-readable message describing the result
    pub message: String,
}

/// Result of exiting the app
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExitResult {
    /// Whether the exit operation was successful
    pub success: bool,
    
    /// Human-readable message describing the result
    pub message: String,
}

/// Current state of the application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    /// Whether the app is currently in the foreground
    pub in_foreground: bool,
    
    /// Whether the app is in the process of finishing
    pub is_finishing: bool,
    
    /// Whether the app has been destroyed (Android API 17+)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_destroyed: Option<bool>,
    
    /// The app's package name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
}
