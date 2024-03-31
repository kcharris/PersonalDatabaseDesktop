// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
use db::db_init::establish_sql_connection;

use crate::db::db_init;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|_app|{
            db_init::init();
            establish_sql_connection();
            Ok(())
            }
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
