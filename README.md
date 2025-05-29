# Tauri Plugin: App Control

[![crates.io](https://img.shields.io/crates/v/tauri-plugin-app-control.svg)](https://crates.io/crates/tauri-plugin-app-control)
[![documentation](https://img.shields.io/docsrs/tauri-plugin-app-control)](https://docs.rs/tauri-plugin-app-control)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/License-Apache--2.0%20OR%20MIT-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A Tauri 2 plugin focused on providing comprehensive **Android application lifecycle control**. It allows you to programmatically minimize, close, and exit your Tauri application on Android, check its foreground/background state, and listen to native lifecycle events. Desktop functionality is included with stubbed implementations that return an `UnsupportedPlatform` error, clearly indicating that these features are mobile-centric.

## Features (Primarily Android)

-   **Minimize App**: Sends the Android app to the background (equivalent to pressing the Home button).
-   **Close App**: Closes the current Android Activity.
-   **Exit App**: Completely exits the Android application with options to:
    -   Remove from recents screen (Android Lollipop+).
    -   Forcefully kill the app process (use with caution).
-   **Check App State**: Determines if the Android app is in the foreground, if the activity is finishing or destroyed, and provides the package name.
-   **Lifecycle Events (Android)**:
    -   `plugin-loaded`: Emitted when the native Android plugin part is loaded.
    -   `app-resumed`: Emitted when the app is brought back to the foreground or a new intent is received.
    -   `app-minimized`: Emitted after the app is successfully minimized.
    -   `app-closing`: Emitted just before the current activity closes.
    -   `app-exiting`: Emitted just before the app exits via the `exitApp` command.

**Desktop Behavior:**
All functions will return an `Error::UnsupportedPlatform` when called on a desktop environment, as this plugin is specifically designed for Android control.

## Prerequisites

-   Your Tauri project must be set up for Android development. Follow the official [Tauri Android guide](https://tauri.app/v2/guides/building/android).
-   This plugin is designed for Tauri 2.x.

## Setup

There are two main parts to installing this plugin: the Rust (Core) part and the JavaScript (API Bindings) part.

### 1. Rust Crate Installation

Add the plugin to your Tauri app's `src-tauri/Cargo.toml`.

**A. Using `cargo add` (Recommended for local development):**
If you have the plugin locally:
```bash
cargo add tauri-plugin-app-control --path /path/to/your/tauri-plugin-app-control
```
*(Replace `/path/to/your/tauri-plugin-app-control` with the actual local path to this plugin's directory.)*

If published to crates.io (once it is):\
```bash
cargo add tauri-plugin-app-control
```

**B. Manual `Cargo.toml` Edit:**
Add the following to your `src-tauri/Cargo.toml` under `[dependencies]`:
```toml
tauri-plugin-app-control = { path = "/path/to/your/tauri-plugin-app-control" } 
# Or if published to crates.io (replace with actual version):
# tauri-plugin-app-control = "0.1.0"
```

### 2. Register the Plugin (Rust)

In your `src-tauri/src/main.rs`, register the plugin with Tauri's builder:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_app_control::init()) // Add this line
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. JavaScript/TypeScript API Installation

The JavaScript bindings provide a typed API to interact with the plugin from your frontend code.

**A. Install the NPM package:**

The NPM package for this plugin is `tauri-plugin-app-control-api`.

If you are developing the plugin locally and want to test it in another project, you should first link it. Navigate to the root of *this plugin's directory* (`tauri-plugin-app-control`) and run:
```bash
# Using bun
bun link

# Using npm
npm link

# Using yarn v1 (classic)
yarn link
```
This makes the package `tauri-plugin-app-control-api` available globally for linking.

Then, in your **Tauri application's root directory** (where your frontend `package.json` is), link the package:
```bash
# Using bun
bun link tauri-plugin-app-control-api

# Using npm
npm link tauri-plugin-app-control-api

# Using yarn v1 (classic)
yarn link tauri-plugin-app-control-api
```

If this plugin were published to NPM, you would install it directly in your Tauri application:
```bash
# Using bun
bun add tauri-plugin-app-control-api

# Using npm
npm install tauri-plugin-app-control-api

# Using pnpm
pnpm add tauri-plugin-app-control-api

# Using yarn
yarn add tauri-plugin-app-control-api
```

### 4. Permissions (Tauri v2+)

For Tauri v2 and later, you must explicitly grant permissions to your plugin's commands. The `app-control` plugin comes with a default permission set that allows all its commands.

In your app's `src-tauri/capabilities/default.json` (or your specific capability file, e.g., `mobile.json`), add the plugin's default permission set by referencing it as `"app-control:default"`:

```json
{
  "$schema": "../gen/schemas/mobile-schema.json", // Or desktop-schema.json as appropriate
  "identifier": "default", // Or your specific capability identifier
  "description": "Default capabilities for the application.",
  "windows": [
    "main" // Ensure your main window identifier is listed
  ],
  "permissions": [
    "core:default", // Or other core permissions you use
    "app-control:default" // Add this line
  ]
}
```

This grants permissions for the commands included in the plugin's `default` set. Internally, the `default` set defined by this plugin bundles the following autogenerated permissions:
*   `allow-minimize-app`
*   `allow-close-app`
*   `allow-exit-app`
*   `allow-is-app-in-foreground`

If you prefer to grant permissions individually instead of using the `default` set, you would reference them in your capabilities file like so:
`"app-control:allow-minimize-app"`, `"app-control:allow-close-app"`, etc.

```json
{
  "$schema": "../gen/schemas/desktop-schema.json", // Or mobile-schema.json for mobile-focused apps
  "identifier": "default",
  "description": "Default capabilities for the application.",
  "windows": [
    "main" // Ensure your main window identifier is listed
  ],
  "permissions": [
    "core:default", // Or other core permissions you use
    "app-control:default" // Add this line
  ]
}
```

This grants permissions for `minimize_app`, `close_app`, `exit_app`, and `is_app_in_foreground` commands when invoked from JavaScript, as these are included in the plugin's `default` set (which internally references `allow-minimize_app`, `allow-close_app`, etc.).

## Usage (JavaScript/TypeScript API)

Import the desired functions and types from the `tauri-plugin-app-control-api` package in your frontend code.

```typescript
import {
  minimizeApp,
  closeApp,
  exitApp,
  isAppInForeground,
  type ExitOptions, // TypeScript type
  type MinimizeResult,
  type CloseResult,
  type ExitResult,
  type AppState,
  // Event listeners
  onPluginLoaded,
  onAppResumed,
  onAppMinimized,
  onAppClosing,
  onAppExiting,
  type PluginLoadedEvent,
  type AppResumedEvent,
  type AppMinimizedEvent,
  type AppClosingEvent,
  type AppExitingEvent,
  type PluginListener // Type for the unlisten function
} from 'tauri-plugin-app-control-api';

// Example: Minimize the app (Android)
async function handleMinimize() {
  try {
    const result: MinimizeResult = await minimizeApp();
    console.log('Minimize success:', result.success, 'Message:', result.message);
  } catch (error) {
    console.error('Failed to minimize:', error);
  }
}

// Example: Exit the app with options (Android)
async function handleExit() {
  const options: ExitOptions = { 
    removeFromRecents: true, 
    killProcess: false 
  };
  try {
    const result: ExitResult = await exitApp(options);
    console.log('Exit success:', result.success, 'Message:', result.message);
  } catch (error) {
    console.error('Failed to exit:', error);
  }
}

// Example: Check app state (Android)
async function checkState() {
  try {
    const state: AppState = await isAppInForeground();
    console.log('App in foreground:', state.inForeground);
    console.log('App is finishing:', state.isFinishing);
    console.log('App is destroyed:', state.isDestroyed);
    console.log('Package Name:', state.packageName);
  } catch (error) {
    console.error('Failed to check state:', error);
  }
}

// Example: Listen to events (Android)
async function setupEventListeners() {
  const unlistenLoaded: PluginListener = await onPluginLoaded((event: PluginLoadedEvent) => {
    console.log('App Control Plugin Loaded:', event.message, 'Platform:', event.platform, 'API Level:', event.apiLevel);
  });

  const unlistenMinimized: PluginListener = await onAppMinimized((event: AppMinimizedEvent) => {
    console.log('App Minimized:', event.success, event.message);
  });

  const unlistenResumed: PluginListener = await onAppResumed((event: AppResumedEvent) => {
    console.log('App Resumed:', event.action, event.data, event.categories);
  });

  // Remember to clean up listeners when your component unmounts or they are no longer needed:
  // unlistenLoaded.unlisten(); // or just call unlistenLoaded()
  // unlistenMinimized.unlisten();
  // unlistenResumed.unlisten();
}

setupEventListeners();
```

## API Details

### Commands (via JS API)

-   `async minimizeApp(): Promise<MinimizeResult>`
    -   Minimizes the Android application.
    -   `MinimizeResult`: `{ success: boolean; message?: string; }`

-   `async closeApp(): Promise<CloseResult>`
    -   Closes the current Android activity.
    -   `CloseResult`: `{ success: boolean; message?: string; }`

-   `async exitApp(options?: ExitOptions): Promise<ExitResult>`
    -   Exits the Android application completely.
    -   `ExitOptions` (Android only, all optional, defaults are `removeFromRecents: true`, `killProcess: false`):
        -   `removeFromRecents?: boolean`: Remove app from recents list (Lollipop+).
        -   `killProcess?: boolean`: Forcefully kill the app process.
    -   `ExitResult`: `{ success: boolean; message?: string; }`

-   `async isAppInForeground(): Promise<AppState>`
    -   Checks the current state of the Android application.
    -   `AppState`:
        -   `inForeground: boolean`
        -   `isFinishing: boolean`
        -   `isDestroyed: boolean` (Android API 17+)
        -   `packageName: string`

### Events (Android - via JS API)

Each event listener function returns a `Promise<PluginListener>`, where `PluginListener` is an object with an `unlisten: () => void` method (or can be called directly as a function) to remove the event listener.

-   `onPluginLoaded(handler: (event: PluginLoadedEvent) => void): Promise<PluginListener>`
    -   `PluginLoadedEvent`: `{ message: string; platform: string; apiLevel?: number; }` (`apiLevel` is Android specific)

-   `onAppMinimized(handler: (event: AppMinimizedEvent) => void): Promise<PluginListener>`
    -   `AppMinimizedEvent`: `{ success: boolean; message?: string; }` (extends `MinimizeResult`)

-   `onAppClosing(handler: (event: AppClosingEvent) => void): Promise<PluginListener>`
    -   `AppClosingEvent`: `{ message: string; timestamp: number; }`

-   `onAppExiting(handler: (event: AppExitingEvent) => void): Promise<PluginListener>`
    -   `AppExitingEvent`: `{ removeFromRecents: boolean; killProcess: boolean; timestamp: number; }`

-   `onAppResumed(handler: (event: AppResumedEvent) => void): Promise<PluginListener>`
    -   `AppResumedEvent`: `{ action: string; data?: string; categories?: string; }`

### Rust API (for use within `src-tauri`)

You can also use the plugin's functions directly from Rust code via the `AppControlExt` trait.

```rust
use tauri_plugin_app_control::{AppControlExt, ExitOptions, AppState, MinimizeResult};
// In a function where you have access to AppHandle, Window, etc.
fn example_rust_usage<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) {
    // Minimize
    match app_handle.app_control().minimize_app() {
        Ok(MinimizeResult { success, message }) => println!("Minimize success: {}, message: {}", success, message.unwrap_or_default()),
        Err(e) => eprintln!("Minimize error: {:?}", e),
    }

    // Check foreground state
    match app_handle.app_control().is_app_in_foreground() {
        Ok(AppState { in_foreground, .. }) => println!("App in foreground from Rust: {}", in_foreground),
        Err(e) => eprintln!("Error getting app state from Rust: {:?}", e),
    }

    // Exit app
    let options = ExitOptions { remove_from_recents: true, kill_process: false };
    if let Err(e) = app_handle.app_control().exit_app(options) {
        eprintln!("Exit error from Rust: {:?}", e);
    }
}
```
Note: When calling `exit_app` directly from Rust as shown above, the `ExitOptions` struct in Rust has non-optional fields (`removeFromRecents: bool`, `killProcess: bool`). This differs slightly from the JS API's optional fields due to how `#[derive(Deserialize)]` works with `Option` for command arguments versus direct struct instantiation.

## Android Specifics

-   Utilizes native Android `Activity` lifecycle methods and `ActivityManager`.
-   `minimizeApp()` uses `activity.moveTaskToBack(true)`.
-   `closeApp()` calls `activity.finish()`.
-   `exitApp()` uses `activity.finishAndRemoveTask()` and `android.os.Process.killProcess()` based on options.

## Desktop Behavior

As mentioned, on desktop platforms (Windows, macOS, Linux), all plugin functions (both from Rust and JS) will result in an `Error::UnsupportedPlatform` being returned. This is by design, as the core focus is Android control.

## Building the Plugin (Development)

1.  Navigate to the root directory of this plugin: `cd tauri-plugin-app-control`
2.  **Rust & Android Native Code:**
    ```bash
    cargo build # For host
    cargo build --target aarch64-linux-android # For Android (or other targets)
    ```
    (The `build.rs` script handles linking the Android Kotlin project during the Rust build process for mobile targets.)
3.  **JavaScript/TypeScript Bindings:**
    (Compiles `guest-js/*.ts` to `dist/` using Rollup)
    ```bash
    bun install # Or npm install, yarn install
    bun run build # Or npm run build, yarn build
    ```

## Contributing

Contributions that align with the Android-centric focus of this plugin are welcome. Please open an issue or submit a pull request.

## License

This plugin is licensed under either of

-   Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.