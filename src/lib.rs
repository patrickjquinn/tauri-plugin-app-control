use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod error;
mod models;
mod commands;

pub use error::{Error, Result}; // Ensure this is correct based on your error module

#[cfg(desktop)]
use desktop::AppControl;
#[cfg(mobile)]
use mobile::AppControl; // AppControl now correctly wraps PluginHandle

/// Extensions to [`tauri::App`], [`tauri::AppHandle`], [`tauri::WebviewWindow`], [`tauri::Webview`] and [`tauri::Window`] to access the app control APIs.
pub trait AppControlExt<R: Runtime> {
    fn app_control(&self) -> &AppControl<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AppControlExt<R> for T {
    fn app_control(&self) -> &AppControl<R> {
        self.state::<AppControl<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("app-control")
        .invoke_handler(tauri::generate_handler![
            commands::minimize_app,
            commands::close_app,
            commands::exit_app,
            commands::is_app_in_foreground
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            {
                // mobile::init returns Result<AppControl<R>> which wraps PluginHandle
                let app_control_instance = mobile::init(app, api)?;
                app.manage(app_control_instance);
            }
            #[cfg(desktop)]
            {
                // Assuming desktop::init also returns Result<AppControl<R>> or similar
                // and AppControl for desktop is structured appropriately.
                let app_control_instance = desktop::init(app, api)?;
                app.manage(app_control_instance);
            }
            Ok(())
        })
        .build()
}
