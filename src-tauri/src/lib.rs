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
    pub scheduler: scheduler::SchedulerEngine,
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

            let scheduler_engine =
                scheduler::SchedulerEngine::new(database.clone(), audio_service.clone());

            tauri::async_runtime::block_on(scheduler_engine.start())
                .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;

            app.manage(AppState {
                database,
                audio: audio_service,
                scheduler: scheduler_engine,
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::audio::validate_audio_file,
            commands::audio::play_audio_file,
            commands::audio::stop_audio,
            commands::audio::get_audio_status,
            commands::scheduler::start_scheduler,
            commands::scheduler::stop_scheduler,
            commands::scheduler::reload_scheduler,
            commands::scheduler::get_scheduler_status,
            commands::scheduler::get_upcoming_executions,
            commands::schedules::get_all_schedules,
            commands::schedules::create_schedule,
            commands::schedules::update_schedule,
            commands::schedules::delete_schedule,
            commands::schedules::toggle_schedule_enabled,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
