
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_image(path: &str) -> String {
    let image_data = std::fs::read(&path).expect("Unable to read image file");
    let image_base64 = base64::encode(&image_data);
    image_base64
}

#[tauri::command]
fn load_dir_by_file(file_path: &str) -> Vec<String> {
    let file = std::path::Path::new(file_path);
    let dir = file.parent();
    
    let entries = fs::read_dir(dir.unwrap())
        .unwrap()
        .map(|res| res.map(|e| e.path().display().to_string()))
        .collect::<Result<Vec<_>,_>>()
        .unwrap();
    entries
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_image,
            load_dir_by_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
