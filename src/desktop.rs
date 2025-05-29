use tauri::{AppHandle, Runtime};
use crate::{models::*, ExitOptions, Result, Error};

// Stub implementation for Desktop

pub struct AppControl<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AppControl<R> {
    pub fn minimize_app(&self) -> Result<MinimizeResult> {
        Err(Error::UnsupportedPlatform("minimize_app is only available on mobile".to_string()))
    }

    pub fn close_app(&self) -> Result<CloseResult> {
        Err(Error::UnsupportedPlatform("close_app is only available on mobile".to_string()))
    }

    pub fn exit_app(&self, _options: ExitOptions) -> Result<ExitResult> {
        Err(Error::UnsupportedPlatform("exit_app is only available on mobile".to_string()))
    }

    pub fn is_app_in_foreground(&self) -> Result<AppState> {
        Err(Error::UnsupportedPlatform("is_app_in_foreground is only available on mobile".to_string()))
    }
}

pub fn init<R: Runtime>(app: &AppHandle<R>, _api: tauri::plugin::PluginApi<R, ()>) -> Result<AppControl<R>> {
    Ok(AppControl(app.clone()))
}
