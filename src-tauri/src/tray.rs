use crate::scheduler::SchedulerEngine;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime, WebviewWindow,
};

/// Build and configure the system tray
pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    // Create menu items
    let show_item = MenuItem::with_id(app, "show", "Show Resonatify", true, None::<&str>)?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let pause_item = MenuItem::with_id(app, "pause_all", "Pause All Schedules", true, None::<&str>)?;
    let resume_item = MenuItem::with_id(app, "resume_all", "Resume All Schedules", true, None::<&str>)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit Resonatify", true, None::<&str>)?;

    // Build the menu
    let menu = Menu::with_items(
        app,
        &[
            &show_item,
            &separator1,
            &pause_item,
            &resume_item,
            &separator2,
            &quit_item,
        ],
    )?;

    // Create the tray icon
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("Resonatify - No schedules active")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "pause_all" => {
                let app_clone = app.clone();
                tauri::async_runtime::spawn(async move {
                    if let Some(state) = app_clone.try_state::<crate::AppState>() {
                        if let Err(e) = state.scheduler.pause_all().await {
                            eprintln!("Failed to pause all schedules: {}", e);
                        }
                    }
                });
            }
            "resume_all" => {
                let app_clone = app.clone();
                tauri::async_runtime::spawn(async move {
                    if let Some(state) = app_clone.try_state::<crate::AppState>() {
                        if let Err(e) = state.scheduler.resume_all().await {
                            eprintln!("Failed to resume all schedules: {}", e);
                        }
                    }
                });
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}

/// Handle window close event - hide to tray instead of quitting
pub fn setup_window_close_handler(window: WebviewWindow) {
    let window_clone = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            // Prevent the window from closing
            api.prevent_close();
            // Hide the window instead
            let _ = window_clone.hide();
        }
    });
}

/// Background task to update tray tooltip with upcoming schedules
/// Note: Dynamic tooltip updates will be implemented in a future iteration
pub async fn start_tray_tooltip_updater<R: Runtime>(
    _app: AppHandle<R>,
    _scheduler: SchedulerEngine,
) {
    // TODO: Implement dynamic tooltip updates
    // This requires storing a reference to the tray icon or using Tauri's event system
    // For now, the tooltip is set once during tray creation
}
