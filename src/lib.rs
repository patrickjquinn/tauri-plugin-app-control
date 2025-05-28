use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::AppControl;
#[cfg(mobile)]
use mobile::AppControl;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the app-control APIs.
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
        commands::is_app_in_foreground,
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let app_control = mobile::init(app, api)?;
      #[cfg(desktop)]
      let app_control = desktop::init(app, api)?;
      app.manage(app_control);
      Ok(())
    })
    .build()
}
