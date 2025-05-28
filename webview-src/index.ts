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