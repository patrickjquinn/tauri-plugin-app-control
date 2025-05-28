# Tauri Plugin App Control

[![crates.io](https://img.shields.io/crates/v/tauri-plugin-app-control.svg)](https://crates.io/crates/tauri-plugin-app-control) 
[![documentation](https://img.shields.io/docsrs/tauri-plugin-app-control)](https://docs.rs/tauri-plugin-app-control)

A Tauri plugin for comprehensive control over your application's lifecycle, with a primary focus on Android, and graceful fallbacks for Desktop.

This plugin allows you to programmatically minimize, close, and exit your Tauri application, check its foreground/background state, and listen to lifecycle events.

## Features

- **Minimize App**: 
  - Android: Sends the app to the background (similar to pressing the Home button).
  - Desktop: Minimizes the main application window.
- **Close App**:
  - Android: Closes the current Android Activity.
  - Desktop: Closes the main application window.
- **Exit App**: 
  - Android: Completely exits the application with options to remove from recents and kill the process.
  - Desktop: Exits the application process.
- **Check App State**:
  - Android: Determines if the app is in the foreground, finishing, or destroyed, and provides the package name.
  - Desktop: Determines if the main application window is focused and visible.
- **Lifecycle Events** (Android & Desktop where applicable):
  - `plugin-loaded`: Emitted when the plugin is successfully loaded.
  - `app-minimized`: Emitted after the app is minimized.
  - `app-closing`: Emitted just before the app/activity closes.
  - `app-exiting`: Emitted just before the app exits.
  - `app-resumed`: Emitted when the app is brought back to the foreground (primarily Android).

## Prerequisites

Ensure your Tauri project is set up for Android development if targeting Android. Follow the official [Tauri Android guide](https://tauri.app/v1/guides/building/android/).

## Setup

There are two ways to install this plugin:

1.  **Using `cargo add` (recommended for Rust dependencies):**
    Open your Tauri app's `src-tauri/Cargo.toml` and run:
    ```bash
    cargo add tauri-plugin-app-control --path /path/to/your/tauri-plugin-app-control
    ```
    (Replace `/path/to/your/tauri-plugin-app-control` with the actual local path to this plugin directory if you're not publishing it to crates.io yet). 

    If published to crates.io, you can use:
    ```bash
    cargo add tauri-plugin-app-control
    ```

2.  **Manual `Cargo.toml` edit:**
    Add the following to your `src-tauri/Cargo.toml` under `[dependencies]`:
    ```toml
    tauri-plugin-app-control = { path = "/path/to/your/tauri-plugin-app-control" } 
    # Or if published to crates.io:
    # tauri-plugin-app-control = "0.1.0" # Replace with the desired version
    ```

### Register the Plugin

In your `src-tauri/src/main.rs` (or `lib.rs` if you have a lib-based project), register the plugin with Tauri:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_app_control::init()) // Add this line
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Add Capabilities (for Tauri v2+)

If you are using Tauri v2 or later, you need to allowlist the plugin's commands in your app's capabilities file.

Create or modify `src-tauri/capabilities/default.json` (or your specific capability file if not using `default`):

```json
{
  "$schema": "../gen/schemas/desktop-schema.json", // Or mobile-schema.json if applicable
  "identifier": "default",
  "description": "Default permissions for the app",
  "windows": ["main"], // Ensure your main window label is listed
  "permissions": [
    "core:default", // Keep other default permissions
    "app-control:default" // Add this line to grant all plugin permissions
  ]
}
```

This grants the `default` permission set from the plugin, which includes:
*   `allow-minimize-app`
*   `allow-close-app`
*   `allow-exit-app`
*   `allow-is-app-in-foreground`

You can also grant permissions individually if preferred.

### Install JavaScript/TypeScript API

In your frontend project (e.g., the root of your Svelte/React/Vue app), install the JS bindings:

```bash
npm install /path/to/your/tauri-plugin-app-control/webview-src
# or
yarn add /path/to/your/tauri-plugin-app-control/webview-src
# or
pnpm add /path/to/your/tauri-plugin-app-control/webview-src
```

If you publish the `webview-src` directory as an NPM package (e.g., as `tauri-plugin-app-control-api`), you would install it by its package name:
```bash
npm install tauri-plugin-app-control-api
```

**Note**: The `package.json` within `webview-src` is configured to be named `tauri-plugin-app-control-api`.

## Usage

Import the functions from the JavaScript API into your frontend code:

```javascript
import {
  minimizeApp,
  closeApp,
  exitApp,
  isAppInForeground,
  onPluginLoaded,
  onAppMinimized,
  onAppClosing,
  onAppExiting,
  onAppResumed
} from 'tauri-plugin-app-control-api'; // Or the local path if not installed as a package

// Example: Minimize the app
async function handleMinimize() {
  try {
    const result = await minimizeApp();
    console.log(result.message);
  } catch (error) {
    console.error('Failed to minimize:', error);
  }
}

// Example: Exit the app with options (Android)
async function handleExit() {
  try {
    const result = await exitApp({ 
      removeFromRecents: true, 
      killProcess: false 
    });
    console.log(result.message);
  } catch (error) {
    console.error('Failed to exit:', error);
  }
}

// Example: Check app state
async function checkState() {
  try {
    const state = await isAppInForeground();
    console.log('App in foreground:', state.inForeground);
    console.log('App is finishing:', state.isFinishing); // Android specific
    if (state.packageName) {
      console.log('Package Name:', state.packageName);
    }
  } catch (error) {
    console.error('Failed to check state:', error);
  }
}

// Example: Listen to events
async function setupEventListeners() {
  const unlistenLoaded = await onPluginLoaded(event => {
    console.log('App Control Plugin Loaded:', event.message, event.platform, event.apiLevel);
  });

  const unlistenMinimized = await onAppMinimized(event => {
    console.log('App Minimized:', event.message);
  });

  const unlistenResumed = await onAppResumed(event => {
    console.log('App Resumed:', event.action, event.data, event.categories);
  });

  // To clean up listeners when your component unmounts or is no longer needed:
  // unlistenLoaded();
  // unlistenMinimized();
  // unlistenResumed();
}

setupEventListeners();
```

## API

### Commands

-   `async minimizeApp(): Promise<MinimizeResult>`
    -   Minimizes the application.
    -   `MinimizeResult`: `{ success: boolean; message: string; }`

-   `async closeApp(): Promise<CloseResult>`
    -   Closes the current activity (Android) or main window (Desktop).
    -   `CloseResult`: `{ success: boolean; message: string; }`

-   `async exitApp(options?: ExitOptions): Promise<ExitResult>`
    -   Exits the application completely.
    -   `ExitOptions` (Android only, defaults are `removeFromRecents: true`, `killProcess: false`):
        -   `removeFromRecents?: boolean`: Whether to remove the app from the recent apps list (Android Lollipop+).
        -   `killProcess?: boolean`: Whether to forcefully kill the app process. Use with caution.
    -   `ExitResult`: `{ success: boolean; message: string; }`

-   `async isAppInForeground(): Promise<AppState>`
    -   Checks the current state of the application.
    -   `AppState`:
        -   `inForeground: boolean`
        -   `isFinishing: boolean` (Android specific)
        -   `isDestroyed?: boolean` (Android API 17+)
        -   `packageName?: string`

### Events

-   `onPluginLoaded(handler: (event: { message: string; platform: string; apiLevel?: number }) => void): Promise<PluginListener>`
    -   Triggered when the plugin finishes loading.
    -   `apiLevel` is Android specific.

-   `onAppMinimized(handler: (event: MinimizeResult) => void): Promise<PluginListener>`
    -   Triggered after the app is successfully minimized.

-   `onAppClosing(handler: (event: { message: string; timestamp: number }) => void): Promise<PluginListener>`
    -   Triggered just before the app/activity attempts to close.

-   `onAppExiting(handler: (event: { removeFromRecents: boolean; killProcess: boolean; timestamp: number }) => void): Promise<PluginListener>`
    -   Triggered just before the app attempts to exit (after `exitApp` is called).

-   `onAppResumed(handler: (event: { action: string; data?: string; categories?: string }) => void): Promise<PluginListener>`
    -   Triggered when the app is brought back to the foreground (e.g., from recents or a new intent on Android).

`PluginListener` is an object with an `unlisten` function to remove the event listener.

## Platform Specifics

### Android

-   Leverages native Android Activity lifecycle methods.
-   `minimizeApp()` uses `moveTaskToBack(true)`.
-   `closeApp()` calls `finish()` on the current Activity.
-   `exitApp()` provides options for `finishAndRemoveTask()` and `Process.killProcess()`.
-   All lifecycle events are fully supported.

#### Optional: Handle Android Back Button

To override the default Android back button behavior (e.g., to minimize instead of going back or closing), you can modify your `MainActivity.kt` in your Tauri app (`src-tauri/gen/android/{your_app_id}/MainActivity.kt` or similar path):

```kotlin
// In your app's MainActivity.kt
override fun onBackPressed() {
    // Example: Minimize the app instead of default behavior
    // You could also call the plugin here if you prefer via an event or direct call
    // For instance, by sending an event to JS which then calls the plugin.
    // Direct call example (if plugin instance is accessible, might require setup):
    // val appControlPlugin = tauri::plugins::PluginManager.getPlugin("app-control") as? AppControlPlugin
    // appControlPlugin?.minimizeAppFromNativeSide() // You would need to add such a method

    // Simpler: just move task to back directly
    this.moveTaskToBack(true)
    // If you want to use the plugin's minimizeApp which also emits events:
    // You would typically trigger this from the JS side after catching a back button event there.
}
```

### Desktop (Windows, macOS, Linux)

-   `minimizeApp()`: Minimizes the main application window.
-   `closeApp()`: Closes the main application window.
-   `exitApp()`: Calls `std::process::exit(0)` or equivalent `app.exit(0)`.
-   `isAppInForeground()`: Checks if the main window is visible and focused.
-   `ExitOptions` are ignored on desktop.
-   Lifecycle events like `app-resumed` have limited applicability compared to Android but will be emitted where sensible (e.g., window focus changes might trigger similar logic if implemented).

## Building the Plugin (Development)

1.  Navigate to the plugin directory: `cd tauri-plugin-app-control`
2.  Build the Rust code: `cargo build`
3.  Build the TypeScript bindings:
    ```bash
    cd webview-src
    npm install # Or pnpm install / bun install
    npm run build # Or your equivalent script that runs tsc
    cd ..
    ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This plugin is licensed under the MIT OR Apache-2.0 license.
