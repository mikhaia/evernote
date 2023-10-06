// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn hello(name: &str) -> String {
    format!("Hello, {}", name)
}

fn main() {
    tauri::Builder::default()
        /*
        .invoke_handler(tauri::generate_handler!['test'])
        .setup(|app| {
            // let window = app.get_window("main").unwrap();
            window.eval("window.location.replace('https://google.com')")
            // Ok(())
        })
        */
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
