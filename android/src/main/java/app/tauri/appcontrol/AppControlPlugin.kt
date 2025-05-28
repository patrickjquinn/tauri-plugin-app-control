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