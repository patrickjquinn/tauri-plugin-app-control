const COMMANDS: &[&str] = &[
    "minimize_app",
    "close_app", 
    "exit_app",
    "is_app_in_foreground"
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
