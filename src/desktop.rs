use tauri::{plugin::PluginApi, AppHandle, Runtime, Manager};

use crate::models::*;

pub struct AppControl<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AppControl<R> {
    pub fn minimize_app(&self) -> crate::Result<MinimizeResult> {
        // Desktop implementation - minimize all windows
        let mut minimized_count = 0;
        let windows = self.0.windows();
        let total_windows = windows.len();
        
        for window in windows.values() {
            if window.minimize().is_ok() {
                minimized_count += 1;
            }
        }
        
        Ok(MinimizeResult {
            success: minimized_count > 0,
            message: format!("Minimized {}/{} windows", minimized_count, total_windows),
        })
    }

    pub fn close_app(&self) -> crate::Result<CloseResult> {
        // Desktop implementation - close all windows
        let mut closed_count = 0;
        let windows = self.0.windows();
        let total_windows = windows.len();
        
        for window in windows.values() {
            if window.close().is_ok() {
                closed_count += 1;
            }
        }
        
        Ok(CloseResult {
            success: closed_count > 0,
            message: format!("Closed {}/{} windows", closed_count, total_windows),
        })
    }

    pub fn exit_app(&self, _options: ExitOptions) -> crate::Result<ExitResult> {
        // Desktop implementation - exit the app
        // Note: options are ignored on desktop as they're Android-specific
        self.0.exit(0);
        
        Ok(ExitResult {
            success: true,
            message: "App exit initiated".to_string(),
        })
    }

    pub fn is_app_in_foreground(&self) -> crate::Result<AppState> {
        // Check if any window is visible and focused
        let mut in_foreground = false;
        let mut has_visible_window = false;
        let windows = self.0.windows();
        
        for window in windows.values() {
            if let Ok(is_visible) = window.is_visible() {
                if is_visible {
                    has_visible_window = true;
                    if let Ok(is_focused) = window.is_focused() {
                        if is_focused {
                            in_foreground = true;
                            break;
                        }
                    }
                }
            }
        }
        
        // If no window is focused but we have visible windows, we're still "active"
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
