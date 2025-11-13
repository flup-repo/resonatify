mod audio;
mod commands;
mod db;
mod scheduler;

use db::Database;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub audio: audio::AudioService,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let database = tauri::async_runtime::block_on(db::init_db(&handle))
                .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;

            let audio_service = audio::AudioService::new()
                .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;

            app.manage(AppState {
                database,
                audio: audio_service,
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::audio::validate_audio_file,
            commands::audio::play_audio_file,
            commands::audio::stop_audio,
            commands::audio::get_audio_status,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
