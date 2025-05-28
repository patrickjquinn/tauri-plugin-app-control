use tauri::{plugin::PluginApi, AppHandle, Runtime, Manager};

use crate::models::*;

pub struct AppControl<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AppControl<R> {
    pub fn minimize_app(&self) -> crate::Result<MinimizeResult> {
        // Only minimize the main window
        let mut minimized_count = 0;
        if let Some(window) = self.0.get_webview_window("main") {
            if window.minimize().is_ok() {
                minimized_count += 1;
            }
        }
        Ok(MinimizeResult {
            success: minimized_count > 0,
            message: format!("Minimized {}/1 windows", minimized_count),
        })
    }

    pub fn close_app(&self) -> crate::Result<CloseResult> {
        // Only close the main window
        let mut closed_count = 0;
        if let Some(window) = self.0.get_webview_window("main") {
            if window.close().is_ok() {
                closed_count += 1;
            }
        }
        Ok(CloseResult {
            success: closed_count > 0,
            message: format!("Closed {}/1 windows", closed_count),
        })
    }

    pub fn exit_app(&self, _options: ExitOptions) -> crate::Result<ExitResult> {
        // Desktop implementation - exit the app
        self.0.exit(0);
        Ok(ExitResult {
            success: true,
            message: "App exit initiated".to_string(),
        })
    }

    pub fn is_app_in_foreground(&self) -> crate::Result<AppState> {
        // Only check the main window
        let mut in_foreground = false;
        let mut has_visible_window = false;
        if let Some(window) = self.0.get_webview_window("main") {
            if let Ok(is_visible) = window.is_visible() {
                if is_visible {
                    has_visible_window = true;
                    if let Ok(is_focused) = window.is_focused() {
                        if is_focused {
                            in_foreground = true;
                        }
                    }
                }
            }
        }
        if !in_foreground && has_visible_window {
            in_foreground = true;
        }
        Ok(AppState {
            in_foreground,
            is_finishing: false, // Not applicable on desktop
            is_destroyed: None,  // Not applicable on desktop
            package_name: Some(self.0.package_info().name.clone()),
        })
    }
}

pub(crate) fn init<R: Runtime, C: serde::de::DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<AppControl<R>> {
    Ok(AppControl(app.clone()))
}
