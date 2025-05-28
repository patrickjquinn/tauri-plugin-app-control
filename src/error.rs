use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    PluginApi(#[from] tauri::Error),
    
    #[error("Failed to minimize app")]
    MinimizeFailed,
    
    #[error("Failed to close app")]
    CloseFailed,
    
    #[error("Failed to exit app")]
    ExitFailed,
    
    #[error("Invalid context")]
    InvalidContext,
    
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
            Error::Unknown(_) => "unknownError",
        }
    }
}
