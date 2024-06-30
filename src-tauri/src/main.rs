// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    io::Read,
    path::{Path, PathBuf},
};
mod util;
use util::fs::{create_dir_if_not_exists, create_or_get_file, write_to_data};

#[tauri::command]
fn load_dir(dir_path: &str, app_handle: tauri::AppHandle) -> String {
    let path = Path::new(dir_path);
    if path.exists() && path.is_dir() {
        match write_to_data(&path.to_path_buf(), app_handle) {
            Ok(_) => format!("Directory: {} loaded", dir_path),
            Err(e) => format!("Failed to write to data.json: {:?}", e),
        }
    } else {
        format!("Directory: {} not found", dir_path)
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
    let mut file = create_or_get_file(&path.join("data.json")).unwrap_or_else(|e| {
        panic!("Failed to create data.json: {:?}", e);
    });

    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);

    if buffer.len() == 0 {
        let _ = write_to_data(&PathBuf::from(""), app.handle());
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(appdata_setup)
        .invoke_handler(tauri::generate_handler![load_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
