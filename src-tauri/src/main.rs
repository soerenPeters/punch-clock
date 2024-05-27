// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app;

fn main() {
  println!("{}", app::greet::greet("World"));
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![tauri_greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn tauri_greet(name: &str) -> String {
   app::greet::greet(name)
}
