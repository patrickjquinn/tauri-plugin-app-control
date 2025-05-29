import { invoke, addPluginListener, type PluginListener } from '@tauri-apps/api/core';

/**
 * Represents the options for exiting the app.
 */
export interface ExitOptions {
  /** Whether to remove the app from the recents screen (Android only, default: true). */
  removeFromRecents?: boolean;
  /** Whether to forcefully kill the app process (Android only, default: false). */
  killProcess?: boolean;
}

/**
 * Represents the result of a minimize operation.
 */
export interface MinimizeResult {
  /** Indicates whether the operation was successful. */
  success: boolean;
  /** An optional message providing more details. */
  message?: string;
}

/**
 * Represents the result of a close operation.
 */
export interface CloseResult {
  /** Indicates whether the operation was successful. */
  success: boolean;
  /** An optional message providing more details. */
  message?: string;
}

/**
 * Represents the result of an exit operation.
 */
export interface ExitResult {
  /** Indicates whether the operation was successful. */
  success: boolean;
  /** An optional message providing more details. */
  message?: string;
}

/**
 * Represents the current state of the application.
 */
export interface AppState {
  /** Whether the app is currently in the foreground. */
  inForeground: boolean;
  /** Whether the main activity is in the process of finishing. */
  isFinishing: boolean;
  /** Whether the main activity has been destroyed (Android API 17+). */
  isDestroyed: boolean;
  /** The package name of the application. */
  packageName: string;
}

// --- Commands ---

/**
 * Minimizes the application.
 * This is equivalent to pressing the home button on Android.
 */
export async function minimizeApp(): Promise<MinimizeResult> {
  return await invoke('plugin:app-control|minimize_app');
}

/**
 * Closes the main activity of the application.
 * Note: This may not exit the app if there are other activities in the stack.
 */
export async function closeApp(): Promise<CloseResult> {
  return await invoke('plugin:app-control|close_app');
}

/**
 * Exits the application completely with configurable options.
 *
 * @param options Configuration for how to exit the app.
 */
export async function exitApp(options?: ExitOptions): Promise<ExitResult> {
  return await invoke('plugin:app-control|exit_app', { options: options ?? {} });
}

/**
 * Checks if the app is currently in the foreground and provides other state information.
 */
export async function isAppInForeground(): Promise<AppState> {
  return await invoke('plugin:app-control|is_app_in_foreground');
}

// --- Event Data Types ---

/**
 * Payload for the 'plugin-loaded' event.
 */
export interface PluginLoadedEvent {
  message: string;
  platform: string;
  apiLevel?: number; // Android specific
}

/**
 * Payload for the 'app-resumed' event (Android only).
 */
export interface AppResumedEvent {
  action: string;
  data?: string;
  categories?: string;
}

/**
 * Payload for the 'app-minimized' event (Android only).
 */
export interface AppMinimizedEvent extends MinimizeResult {}

/**
 * Payload for the 'app-closing' event (Android only).
 */
export interface AppClosingEvent {
  message: string;
  timestamp: number;
}

/**
 * Payload for the 'app-exiting' event (Android only).
 */
export interface AppExitingEvent {
  removeFromRecents: boolean;
  killProcess: boolean;
  timestamp: number;
}

// --- Event Listener Types ---

export type PluginLoadedListener = (event: PluginLoadedEvent) => void;
export type AppResumedListener = (event: AppResumedEvent) => void;
export type AppMinimizedListener = (event: AppMinimizedEvent) => void;
export type AppClosingListener = (event: AppClosingEvent) => void;
export type AppExitingListener = (event: AppExitingEvent) => void;

// --- Event Listener Functions ---

/**
 * Listens for the 'plugin-loaded' event, emitted when the native plugin initializes.
 */
export async function onPluginLoaded(
  handler: PluginLoadedListener
): Promise<PluginListener> {
  return await addPluginListener<PluginLoadedEvent>(
    'app-control',
    'plugin-loaded',
    handler
  );
}

/**
 * Listens for the 'app-resumed' event, emitted when the Android app is brought back to the foreground or a new intent is received.
 */
export async function onAppResumed(
  handler: AppResumedListener
): Promise<PluginListener> {
  return await addPluginListener<AppResumedEvent>(
    'app-control',
    'app-resumed',
    handler
  );
}

/**
 * Listens for the 'app-minimized' event, emitted when the app is successfully minimized on Android.
 */
export async function onAppMinimized(
  handler: AppMinimizedListener
): Promise<PluginListener> {
  return await addPluginListener<AppMinimizedEvent>(
    'app-control',
    'app-minimized',
    handler
  );
}

/**
 * Listens for the 'app-closing' event, emitted just before the main activity is closed on Android.
 */
export async function onAppClosing(
  handler: AppClosingListener
): Promise<PluginListener> {
  return await addPluginListener<AppClosingEvent>(
    'app-control',
    'app-closing',
    handler
  );
}

/**
 * Listens for the 'app-exiting' event, emitted just before the app exits based on the exitApp command on Android.
 */
export async function onAppExiting(
  handler: AppExitingListener
): Promise<PluginListener> {
  return await addPluginListener<AppExitingEvent>(
    'app-control',
    'app-exiting',
    handler
  );
}

// TODO: Implement event listeners based on AppControlPlugin.kt events
// Example:
// import { listen, Event } from '@tauri-apps/api/event';
// export async function onPluginLoaded(handler: (event: Event<any>) => void): Promise<() => void> {
//   return listen('plugin-loaded', handler, { target: 'plugin:app-control' });
// } 