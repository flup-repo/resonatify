mod audio;
mod commands;
mod db;
mod scheduler;
mod tray;

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
                scheduler: scheduler_engine.clone(),
            });

            // Set up system tray
            tray::create_tray(app.handle())?;

            // Set up window close handler (hide to tray instead of quit)
            if let Some(window) = app.get_webview_window("main") {
                tray::setup_window_close_handler(window);
            }

            // Start background task to update tray tooltip
            let tooltip_app = app.handle().clone();
            let tooltip_scheduler = scheduler_engine.clone();
            tauri::async_runtime::spawn(async move {
                tray::start_tray_tooltip_updater(tooltip_app, tooltip_scheduler).await;
            });

            Ok(())
        })
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::audio::validate_audio_file,
            commands::audio::play_audio_file,
            commands::audio::stop_audio,
            commands::audio::get_audio_status,
            commands::dialogs::open_audio_file_dialog,
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
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::set_launch_at_login,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
