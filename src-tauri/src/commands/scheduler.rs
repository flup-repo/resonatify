use tauri::State;

use crate::scheduler::{SchedulerEngine, SchedulerStatus, UpcomingExecution};
use crate::AppState;

fn scheduler(state: &State<'_, AppState>) -> SchedulerEngine {
    state.scheduler.clone()
}

#[tauri::command]
pub async fn start_scheduler(state: State<'_, AppState>) -> Result<(), String> {
    scheduler(&state)
        .start()
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn stop_scheduler(state: State<'_, AppState>) -> Result<(), String> {
    scheduler(&state)
        .stop()
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn reload_scheduler(state: State<'_, AppState>) -> Result<(), String> {
    scheduler(&state)
        .reload()
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn get_scheduler_status(state: State<'_, AppState>) -> Result<SchedulerStatus, String> {
    Ok(scheduler(&state).status().await)
}

#[tauri::command]
pub async fn get_upcoming_executions(
    count: usize,
    state: State<'_, AppState>,
) -> Result<Vec<UpcomingExecution>, String> {
    Ok(scheduler(&state).upcoming_executions(count).await)
}
