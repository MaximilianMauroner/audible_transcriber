// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::OpenOptions, io::Write, os::windows::fs::MetadataExt};

use tauri::api::path::audio_dir;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn append_chunk_to_file(path: String, chunk: Vec<u8>) -> Result<u64, String> {
    let audio_dir = audio_dir();

    match audio_dir {
        Some(dir) => {
            let folder = dir.join("audible/temp");
            if !folder.exists() {
                std::fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
            }

            let file_path = folder.join(path);
            println!("Path: {:?}", file_path);

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .map_err(|e| e.to_string())?;

            file.write_all(&chunk).map_err(|e| e.to_string())?;

            return Ok(file.metadata().unwrap().file_size());
        }
        None => Err("String".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![append_chunk_to_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
