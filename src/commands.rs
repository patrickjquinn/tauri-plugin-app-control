use tauri::{AppHandle, Runtime, command};

use crate::models::*;
use crate::Result;
use crate::AppControlExt;

#[command]
pub(crate) async fn minimize_app<R: Runtime>(
    app: AppHandle<R>,
) -> Result<MinimizeResult> {
    app.app_control().minimize_app()
}

#[command]
pub(crate) async fn close_app<R: Runtime>(
    app: AppHandle<R>,
) -> Result<CloseResult> {
    app.app_control().close_app()
}

#[command]
pub(crate) async fn exit_app<R: Runtime>(
    app: AppHandle<R>,
    options: Option<ExitOptions>,
) -> Result<ExitResult> {
    app.app_control().exit_app(options.unwrap_or_default())
}

#[command]
pub(crate) async fn is_app_in_foreground<R: Runtime>(
    app: AppHandle<R>,
) -> Result<AppState> {
    app.app_control().is_app_in_foreground()
}
