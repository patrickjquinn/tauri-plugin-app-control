# Tauri 2 Android App Control Plugin

A complete Tauri 2 plugin for controlling Android app lifecycle (minimize, close, exit) with full implementation details. This plugin provides cross-platform APIs for managing app state on both mobile and desktop platforms.

## Features

- **Minimize App** - Send app to background (Android) or minimize windows (Desktop)
- **Close App** - Close current activity (Android) or all windows (Desktop)
- **Exit App** - Completely exit with configurable options
- **Check App State** - Determine if app is in foreground
- **Lifecycle Events** - Listen to app state changes

## Plugin Structure

```
tauri-plugin-app-control/
├── src/
│   ├── lib.rs
│   ├── desktop.rs
│   ├── mobile.rs
│   ├── models.rs
│   └── commands.rs
├── android/
│   └── src/main/java/AppControlPlugin.kt
├── build.rs
├── Cargo.toml
├── tauri.conf.json
├── permissions/
│   └── default.toml
└── webview-src/
    └── index.ts
```



## 1. Android Implementation

### `android/src/main/java/AppControlPlugin.kt`

```kotlin
// Copyright 2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

package app.tauri.appcontrol

import android.app.Activity
import android.app.ActivityManager
import android.content.Context
import android.content.Intent
import android.os.Build
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class ExitOptions {
    var removeFromRecents: Boolean = true
    var killProcess: Boolean = false
}

@TauriPlugin
class AppControlPlugin(private val activity: Activity): Plugin(activity) {
    
    companion object {
        // Error codes
        private const val MINIMIZE_FAILED = "minimizeFailed"
        private const val CLOSE_FAILED = "closeFailed"
        private const val EXIT_FAILED = "exitFailed"
        private const val INVALID_CONTEXT = "invalidContext"
        private const val UNKNOWN_ERROR = "unknownError"
        
        // Event names
        private const val EVENT_PLUGIN_LOADED = "plugin-loaded"
        private const val EVENT_APP_RESUMED = "app-resumed"
        private const val EVENT_APP_MINIMIZED = "app-minimized"
        private const val EVENT_APP_CLOSING = "app-closing"
        private const val EVENT_APP_EXITING = "app-exiting"
    }
    
    override fun load(webView: WebView) {
        super.load(webView)
        
        // Emit plugin loaded event
        val event = JSObject()
        event.put("message", "App Control Plugin loaded")
        event.put("platform", "android")
        event.put("apiLevel", Build.VERSION.SDK_INT)
        trigger(EVENT_PLUGIN_LOADED, event)
    }
    
    override fun onNewIntent(intent: Intent) {
        // Handle new intent (e.g., when app is brought back to foreground)
        val event = JSObject()
        event.put("action", intent.action ?: "unknown")
        event.put("data", intent.dataString)
        event.put("categories", intent.categories?.joinToString(",") ?: "")
        trigger(EVENT_APP_RESUMED, event)
    }
    
    /**
     * Minimize the app by moving the task to the background.
     * This is equivalent to pressing the home button.
     */
    @Command
    fun minimizeApp(invoke: Invoke) {
        try {
            // Move the task to background (minimize the app)
            val result = activity.moveTaskToBack(true)
            
            val ret = JSObject()
            ret.put("success", result)
            ret.put("message", if (result) "App minimized successfully" else "Failed to minimize app")
            
            if (result) {
                // Emit event only on success
                trigger(EVENT_APP_MINIMIZED, ret)
                invoke.resolve(ret)
            } else {
                invoke.reject("Failed to minimize app", MINIMIZE_FAILED)
            }
        } catch (e: Exception) {
            invoke.reject(
                "Failed to minimize app: ${e.message}",
                UNKNOWN_ERROR
            )
        }
    }
    
    /**
     * Close the current activity.
     * Note: This may not exit the app if there are other activities in the stack.
     */
    @Command
    fun closeApp(invoke: Invoke) {
        try {
            // Emit event before closing
            val event = JSObject()
            event.put("message", "App is closing")
            event.put("timestamp", System.currentTimeMillis())
            trigger(EVENT_APP_CLOSING, event)
            
            // Close the current activity
            activity.finish()
            
            val ret = JSObject()
            ret.put("success", true)
            ret.put("message", "App closed successfully")
            invoke.resolve(ret)
        } catch (e: Exception) {
            invoke.reject(
                "Failed to close app: ${e.message}",
                CLOSE_FAILED
            )
        }
    }
    
    /**
     * Exit the app completely with configurable options.
     * 
     * @param options Configuration for how to exit the app
     */
    @Command
    fun exitApp(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(ExitOptions::class.java)
            
            // Emit event before exiting
            val event = JSObject()
            event.put("removeFromRecents", args.removeFromRecents)
            event.put("killProcess", args.killProcess)
            event.put("timestamp", System.currentTimeMillis())
            trigger(EVENT_APP_EXITING, event)
            
            if (args.removeFromRecents) {
                // Completely remove the app from recents
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
                    activity.finishAndRemoveTask()
                } else {
                    // Fallback for older versions
                    activity.finish()
                }
            } else {
                // Just finish the activity
                activity.finish()
            }
            
            if (args.killProcess) {
                // Schedule process termination after a short delay to allow response
                android.os.Handler(android.os.Looper.getMainLooper()).postDelayed({
                    android.os.Process.killProcess(android.os.Process.myPid())
                }, 100)
            }
            
            val ret = JSObject()
            ret.put("success", true)
            ret.put("message", "App exit initiated")
            invoke.resolve(ret)
        } catch (e: Exception) {
            invoke.reject(
                "Failed to exit app: ${e.message}",
                EXIT_FAILED
            )
        }
    }
    
    /**
     * Check if the app is currently in the foreground.
     * Also provides additional app state information.
     */
    @Command
    fun isAppInForeground(invoke: Invoke) {
        try {
            val activityManager = activity.getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager
            var isInForeground = false
            
            // Check if our app is in foreground
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                // For Android Q and above
                isInForeground = activityManager.appTasks.any { 
                    it.taskInfo.topActivity?.packageName == activity.packageName 
                }
            } else {
                // For older versions
                @Suppress("DEPRECATION")
                val runningAppProcesses = activityManager.runningAppProcesses
                if (runningAppProcesses != null) {
                    for (processInfo in runningAppProcesses) {
                        if (processInfo.processName == activity.packageName &&
                            processInfo.importance == ActivityManager.RunningAppProcessInfo.IMPORTANCE_FOREGROUND) {
                            isInForeground = true
                            break
                        }
                    }
                }
            }
            
            val ret = JSObject()
            ret.put("inForeground", isInForeground)
            ret.put("isFinishing", activity.isFinishing)
            ret.put("isDestroyed", if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR1) activity.isDestroyed else false)
            ret.put("packageName", activity.packageName)
            invoke.resolve(ret)
        } catch (e: Exception) {
            invoke.reject(
                "Failed to check app state: ${e.message}",
                UNKNOWN_ERROR
            )
        }
    }
}
```

## 2. Rust Implementation

### `src/lib.rs`

```rust
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

pub trait AppControlExt<R: Runtime> {
    fn app_control(&self) -> &AppControl<R>;
}

impl<R: Runtime, T: Manager<R>> AppControlExt<R> for T {
    fn app_control(&self) -> &AppControl<R> {
        self.state::<AppControl<R>>().inner()
    }
}

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
```

### `src/mobile.rs`

```rust
use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.appcontrol";

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
    api: PluginApi<R>,
) -> crate::Result<AppControl<R>> {
    #[cfg(target_os = "android")]
    api.register_android_plugin(PLUGIN_IDENTIFIER, "AppControlPlugin")?;
    
    #[cfg(target_os = "ios")]
    api.register_ios_plugin(init_plugin_app_control)?;
    
    Ok(AppControl(api))
}
```

### `src/desktop.rs`

```rust
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime, Window};

use crate::models::*;

pub struct AppControl<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AppControl<R> {
    pub fn minimize_app(&self) -> crate::Result<MinimizeResult> {
        // Desktop implementation - minimize all windows
        let mut minimized_count = 0;
        let total_windows = self.0.windows().len();
        
        for window in self.0.windows().values() {
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
        let total_windows = self.0.windows().len();
        
        for window in self.0.windows().values() {
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
        
        for window in self.0.windows().values() {
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

pub(crate) fn init<R: Runtime>(
    app: &AppHandle<R>,
    _api: PluginApi<R>,
) -> crate::Result<AppControl<R>> {
    Ok(AppControl(app.clone()))
}
```
```

### `src/models.rs`

```rust
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
```

### `src/commands.rs`

```rust
use tauri::{AppHandle, Runtime, command};
use crate::models::*;
use crate::AppControlExt;
use crate::Result;

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
```

### `src/error.rs`

```rust
use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    PluginApi(#[from] tauri::plugin::Error),
    
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
```

## 3. Build Configuration

### `build.rs`

```rust
const COMMANDS: &[&str] = &[
    "minimize_app",
    "close_app", 
    "exit_app",
    "is_app_in_foreground"
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
```

## 4. Permissions Configuration

### `permissions/default.toml`

```toml
# Default permissions for the app-control plugin

[[permission]]
identifier = "allow-minimize-app"
description = "Allows minimizing the app to background"

[[permission]]
identifier = "allow-close-app"
description = "Allows closing the app"

[[permission]]
identifier = "allow-exit-app"
description = "Allows exiting the app completely"

[[permission]]
identifier = "allow-is-app-in-foreground"
description = "Allows checking if app is in foreground"

[[set]]
identifier = "default"
description = "Default permissions for app control"
permissions = [
    "allow-minimize-app",
    "allow-close-app",
    "allow-exit-app",
    "allow-is-app-in-foreground"
]
```

## 5. TypeScript/JavaScript Bindings

### `webview-src/index.ts`

```typescript
import { invoke, addPluginListener, PluginListener } from '@tauri-apps/api/core'

export interface ExitOptions {
  removeFromRecents?: boolean;
  killProcess?: boolean;
}

export interface MinimizeResult {
  success: boolean;
  message: string;
}

export interface CloseResult {
  success: boolean;
  message: string;
}

export interface ExitResult {
  success: boolean;
  message: string;
}

export interface AppState {
  inForeground: boolean;
  isFinishing: boolean;
}

export async function minimizeApp(): Promise<MinimizeResult> {
  return await invoke('plugin:app-control|minimize_app')
}

export async function closeApp(): Promise<CloseResult> {
  return await invoke('plugin:app-control|close_app')
}

export async function exitApp(options?: ExitOptions): Promise<ExitResult> {
  return await invoke('plugin:app-control|exit_app', { options })
}

export async function isAppInForeground(): Promise<AppState> {
  return await invoke('plugin:app-control|is_app_in_foreground')
}

// Event listeners
export async function onPluginLoaded(
  handler: (event: { message: string }) => void
): Promise<PluginListener> {
  return await addPluginListener('app-control', 'plugin-loaded', handler)
}

export async function onAppMinimized(
  handler: (event: MinimizeResult) => void
): Promise<PluginListener> {
  return await addPluginListener('app-control', 'app-minimized', handler)
}

export async function onAppClosing(
  handler: (event: { message: string }) => void
): Promise<PluginListener> {
  return await addPluginListener('app-control', 'app-closing', handler)
}

export async function onAppExiting(
  handler: (event: { removeFromRecents: boolean; killProcess: boolean }) => void
): Promise<PluginListener> {
  return await addPluginListener('app-control', 'app-exiting', handler)
}

export async function onAppResumed(
  handler: (event: { action: string }) => void
): Promise<PluginListener> {
  return await addPluginListener('app-control', 'app-resumed', handler)
}
```

## 6. Package Configuration

### `Cargo.toml`

```toml
[package]
name = "tauri-plugin-app-control"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "Tauri plugin for controlling app lifecycle on Android"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/tauri-plugin-app-control"
keywords = ["tauri", "plugin", "android", "lifecycle", "minimize"]
categories = ["gui"]

[dependencies]
tauri = { version = "2.0.0", features = [] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"

[build-dependencies]
tauri-plugin = { version = "2.0.0", features = ["build"] }
```

### `package.json` (for NPM package)

```json
{
  "name": "tauri-plugin-app-control-api",
  "version": "0.1.0",
  "description": "Tauri plugin for controlling app lifecycle",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "prepublishOnly": "npm run build"
  },
  "keywords": ["tauri", "plugin", "android", "app-control"],
  "files": ["dist", "README.md"],
  "devDependencies": {
    "@tauri-apps/api": "^2.0.0",
    "typescript": "^5.0.0"
  }
}
```

### `webview-src/tsconfig.json`

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "node",
    "declaration": true,
    "declarationMap": true,
    "outDir": "../dist",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["*.ts"],
  "exclude": ["node_modules", "dist"]
}
```

## 7. Using the Plugin in Your Tauri App

### Add to `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-app-control = { path = "../tauri-plugin-app-control" }
# or from git
# tauri-plugin-app-control = { git = "https://github.com/yourusername/tauri-plugin-app-control" }
```

### Register in `src-tauri/src/main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_app_control::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Add capability in `src-tauri/capabilities/default.json`:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default permissions",
  "windows": ["main"],
  "permissions": [
    "app-control:default"
  ]
}
```

### Use in Frontend:

```javascript
import { 
  minimizeApp, 
  closeApp, 
  exitApp, 
  isAppInForeground,
  onAppMinimized,
  onAppResumed 
} from 'tauri-plugin-app-control-api';

// Set up event listeners
const unlistenMinimized = await onAppMinimized((event) => {
  console.log('App minimized:', event);
});

const unlistenResumed = await onAppResumed((event) => {
  console.log('App resumed:', event.action);
});

// Minimize app (send to background)
await minimizeApp();

// Close current activity
await closeApp();

// Exit app completely with options
await exitApp({ 
  removeFromRecents: true,  // Remove from recent apps
  killProcess: false        // Don't force kill process
});

// Check app state
const state = await isAppInForeground();
console.log('App in foreground:', state.inForeground);
```

## 8. Handle Android Back Button (Optional)

To override the back button behavior, add to your `MainActivity.kt`:

```kotlin
override fun onBackPressed() {
    // Instead of default behavior, minimize the app
    moveTaskToBack(true)
}
```

## Building and Testing

1. Build the plugin:
   ```bash
   cd tauri-plugin-app-control
   cargo build
   cd webview-src && npm install && npm run build
   ```

2. Test in your Tauri app:
   ```bash
   cargo tauri android dev
   ```

3. Build for production:
   ```bash
   cargo tauri android build
   ```

## Publishing

To publish as a reusable plugin:

1. Update version in `Cargo.toml` and `package.json`
2. Build the TypeScript bindings: `cd webview-src && npm run build`
3. Publish to crates.io: `cargo publish`
4. Publish to npm: `cd webview-src && npm publish`

## API Reference

### Commands

- `minimizeApp()` - Minimizes the app to background
- `closeApp()` - Closes the current activity/window
- `exitApp(options?)` - Exits the app with optional configuration
- `isAppInForeground()` - Checks if app is currently visible

### Events

- `plugin-loaded` - Emitted when plugin is initialized
- `app-minimized` - Emitted after app is minimized
- `app-closing` - Emitted before app closes
- `app-exiting` - Emitted before app exits
- `app-resumed` - Emitted when app is brought back (Android only)

## Platform Differences

### Android
- `minimizeApp()` uses `moveTaskToBack()` 
- `closeApp()` calls `finish()` on the activity
- `exitApp()` can remove from recents and kill process
- Full lifecycle event support

### Desktop
- `minimizeApp()` minimizes all windows
- `closeApp()` closes all windows
- `exitApp()` calls system exit
- Limited lifecycle events

## License

MIT OR Apache-2.0