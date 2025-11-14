use tauri::State;

use crate::db::models::{CreateScheduleInput, Schedule, UpdateScheduleInput};
use crate::db::Database;
use crate::scheduler::SchedulerEngine;
use crate::AppState;

fn database(state: &State<'_, AppState>) -> Database {
    state.database.clone()
}

fn scheduler(state: &State<'_, AppState>) -> SchedulerEngine {
    state.scheduler.clone()
}

#[tauri::command]
pub async fn get_all_schedules(state: State<'_, AppState>) -> Result<Vec<Schedule>, String> {
    database(&state)
        .schedule_repository()
        .get_all()
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn create_schedule(
    input: CreateScheduleInput,
    state: State<'_, AppState>,
) -> Result<Schedule, String> {
    let repo = database(&state).schedule_repository();
    let created = repo.create(input).await.map_err(|err| err.to_string())?;

    scheduler(&state)
        .reload()
        .await
        .map_err(|err| err.to_string())?;

    Ok(created)
}

#[tauri::command]
pub async fn update_schedule(
    id: String,
    input: UpdateScheduleInput,
    state: State<'_, AppState>,
) -> Result<Schedule, String> {
    let repo = database(&state).schedule_repository();
    let updated = repo
        .update(&id, input)
        .await
        .map_err(|err| err.to_string())?;

    scheduler(&state)
        .reload()
        .await
        .map_err(|err| err.to_string())?;

    Ok(updated)
}

#[tauri::command]
pub async fn delete_schedule(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let repo = database(&state).schedule_repository();
    repo.delete(&id).await.map_err(|err| err.to_string())?;

    scheduler(&state)
        .reload()
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn toggle_schedule_enabled(
    id: String,
    enabled: bool,
    state: State<'_, AppState>,
) -> Result<Schedule, String> {
    let repo = database(&state).schedule_repository();
    let updated = repo
        .update(
            &id,
            UpdateScheduleInput {
                enabled: Some(enabled),
                ..Default::default()
            },
        )
        .await
        .map_err(|err| err.to_string())?;

    scheduler(&state)
        .reload()
        .await
        .map_err(|err| err.to_string())?;

    Ok(updated)
}
