const COMMANDS: &[&str] = &[
    "minimize_app",
    "close_app",
    "exit_app",
    "is_app_in_foreground",
    // "on_app_state_change",
    // "on_volume_up_key_down",
    // "on_volume_down_key_down"
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios") 
        .build();
}
