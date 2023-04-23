// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, io::Write};
use tauri::{api::dialog::blocking::FileDialogBuilder, Manager};

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit_all("single-instance", Payload { args: argv, cwd }).unwrap();
        })) // For Blocking Mutiple Instances
        .plugin(tauri_plugin_window_state::Builder::default().build()) // For Saving Window Size and Position
        .invoke_handler(tauri::generate_handler![save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// == SAVING FILE =============================================================== //
#[tauri::command(async)]
async fn save_file(win: tauri::Window, contents: Vec<String>) {
    let default_filename = if let Some(first_line) = contents.first() {
        Some(first_line.trim())
    } else {
        None
    };

    let filepath = FileDialogBuilder::new()
        .set_parent(&win)
        .add_filter("Text Document", &["txt"])
        .add_filter("XML Document", &["xml"])
        .add_filter("Markdown Document", &["md"])
        .set_file_name(default_filename.unwrap_or_default())
        .save_file()
        .unwrap_or_default();

    if !filepath.exists() {
        let mut file = File::create(filepath).unwrap();
        for line in contents {
            file.write_all(line.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    }
}


// == OPENING FILE ============================================================== //