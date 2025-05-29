use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "app.tauri.appcontrol";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_app_control);

pub struct AppControl<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AppControl<R> {
  pub fn minimize_app(&self) -> crate::Result<MinimizeResult> {
    self.0
      .run_mobile_plugin("minimizeApp", ())
      .map_err(Into::into)
  }

  pub fn close_app(&self) -> crate::Result<CloseResult> {
    self.0
      .run_mobile_plugin("closeApp", ())
      .map_err(Into::into)
  }

  pub fn exit_app(&self, options: ExitOptions) -> crate::Result<ExitResult> {
    self.0
      .run_mobile_plugin("exitApp", options)
      .map_err(Into::into)
  }

  pub fn is_app_in_foreground(&self) -> crate::Result<AppState> {
    self.0
      .run_mobile_plugin("isAppInForeground", ())
      .map_err(Into::into)
  }
}

pub(crate) fn init<R: Runtime>(
  _app: &AppHandle<R>,
  api: PluginApi<R, ()>,
) -> crate::Result<AppControl<R>> {
  #[cfg(target_os = "android")]
  let handle = api
    .register_android_plugin(PLUGIN_IDENTIFIER, "AppControlPlugin")
    .map_err(|e| crate::error::Error::Unknown(e.to_string()))?;
  
  #[cfg(target_os = "ios")]
  let handle = api
    .register_ios_plugin(init_plugin_app_control)
    .map_err(|e| crate::error::Error::Unknown(e.to_string()))?;
  
  Ok(AppControl(handle))
}
