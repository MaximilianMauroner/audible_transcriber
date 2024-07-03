// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path::Path;
mod util;
use serde::{Deserialize, Serialize};
use util::fs::{copy_dir, create_dir_if_not_exists};

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
    success: bool,
}

#[tauri::command]
fn add_directory(dir_path: &str, app_handle: tauri::AppHandle) -> Response {
    let path = Path::new(dir_path);

    let app_data_path = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("Failed to resolve app data directory");

    if path.exists() && path.is_dir() {
        let folder_name = path.file_name().unwrap().to_str().unwrap();
        let result = copy_dir(
            &path.to_path_buf(),
            &app_data_path.join("assets").join(folder_name),
            true,
        );
        match result {
            Ok(_) => {
                let json = Response {
                    success: true,
                    message: format!("Directory: {} selected", dir_path),
                };
                json
            }
            Err(e) => {
                let json = Response {
                    success: false,
                    message: format!("Failed to copy directory: {:?}", e),
                };
                json
            }
        }
    } else {
        let json = Response {
            success: false,
            message: format!("Directory: {} not found", dir_path),
        };
        json
    }
}

fn appdata_setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let path = match app.path_resolver().app_data_dir() {
        Some(path) => path,
        None => panic!("Failed to resolve app data directory"),
    };
    create_dir_if_not_exists(&path).unwrap_or_else(|e| {
        panic!("Failed to create app data directory: {:?}", e);
    });
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(appdata_setup)
        .invoke_handler(tauri::generate_handler![add_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
