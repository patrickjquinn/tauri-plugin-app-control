use serde::{ser::Serializer, Serialize};

#[cfg(mobile)] // Conditionally compile this import
use tauri::plugin::mobile::PluginInvokeError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    PluginApi(#[from] tauri::Error),

    #[cfg(mobile)] // Conditionally compile this variant
    #[error(transparent)]
    PluginInvoke(#[from] PluginInvokeError),
    
    #[error("Failed to minimize app")]
    MinimizeFailed,
    
    #[error("Failed to close app")]
    CloseFailed,
    
    #[error("Failed to exit app")]
    ExitFailed,
    
    #[error("Invalid context")]
    InvalidContext,

    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Error {
    pub fn error_code(&self) -> &'static str {
        match self {
            Error::MinimizeFailed => "minimizeFailed",
            Error::CloseFailed => "closeFailed",
            Error::ExitFailed => "exitFailed",
            Error::InvalidContext => "invalidContext",
            Error::Io(_) => "ioError",
            Error::PluginApi(_) => "pluginError",
            #[cfg(mobile)] // Conditionally compile this match arm
            Error::PluginInvoke(_) => "pluginInvokeError",
            Error::UnsupportedPlatform(_) => "unsupportedPlatform",
            Error::Unknown(_) => "unknownError",
        }
    }
}
