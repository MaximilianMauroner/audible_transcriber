// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    fs,
    path::{Path, PathBuf},
};
mod util;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use util::fs::{create_dir_if_not_exists, create_or_get_file, read_json};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AudibleDataLocation {
    path: PathBuf,
}

#[tauri::command]
fn load_dir(dir_path: &str) -> String {
    let path = Path::new(dir_path);
    if path.exists() && path.is_dir() {
        format!("Directory: {} found", dir_path)
    } else {
        format!("Directory: {} not found", dir_path)
    }
}

fn appdata_setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    app.handle().path_resolver().app_data_dir();
    let path = match app.path_resolver().app_data_dir() {
        Some(path) => path,
        None => panic!("Failed to resolve app data directory"),
    };
    println!("App data directory: {:?}", path);
    create_dir_if_not_exists(&path).unwrap_or_else(|e| {
        panic!("Failed to create app data directory: {:?}", e);
    });
    let file_path = path.join("data.json");
    let file = create_or_get_file(&path.join("data.json")).unwrap_or_else(|e| {
        panic!("Failed to create data.json: {:?}", e);
    });

    let data: AudibleDataLocation =
        util::fs::read_json(&file_path).context("failed to read manager save data");

    println!("With text:\n{data}");

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(appdata_setup)
        .invoke_handler(tauri::generate_handler![load_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
