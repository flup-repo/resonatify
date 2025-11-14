use serde::Deserialize;
use tauri::State;

use crate::db::models::{Setting, SettingsSnapshot};
use crate::db::Database;
use crate::AppState;

fn database(state: &State<'_, AppState>) -> Database {
    state.database.clone()
}

pub type SettingsResponse = SettingsSnapshot;

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsPayload {
    pub theme: Option<String>,
    pub launch_at_login: Option<bool>,
    pub minimize_to_tray: Option<bool>,
    pub show_notifications: Option<bool>,
    pub notification_sound: Option<bool>,
    pub default_volume: Option<u8>,
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<SettingsResponse, String> {
    let records: Vec<Setting> = database(&state)
        .settings_repository()
        .get_all()
        .await
        .map_err(|err| err.to_string())?;

    Ok(SettingsSnapshot::from(records))
}

#[tauri::command]
pub async fn update_settings(
    payload: UpdateSettingsPayload,
    state: State<'_, AppState>,
) -> Result<SettingsResponse, String> {
    let repo = database(&state).settings_repository();

    if let Some(theme) = payload.theme {
        repo.upsert("theme", &theme)
            .await
            .map_err(|err| err.to_string())?;
    }

    if let Some(flag) = payload.launch_at_login {
        repo.upsert("launch_at_login", &flag.to_string())
            .await
            .map_err(|err| err.to_string())?;
    }

    if let Some(flag) = payload.minimize_to_tray {
        repo.upsert("minimize_to_tray", &flag.to_string())
            .await
            .map_err(|err| err.to_string())?;
    }

    if let Some(flag) = payload.show_notifications {
        repo.upsert("show_notifications", &flag.to_string())
            .await
            .map_err(|err| err.to_string())?;
    }

    if let Some(flag) = payload.notification_sound {
        repo.upsert("notification_sound", &flag.to_string())
            .await
            .map_err(|err| err.to_string())?;
    }

    if let Some(volume) = payload.default_volume {
        repo.upsert("default_volume", &volume.to_string())
            .await
            .map_err(|err| err.to_string())?;
    }

    get_settings(state).await
}

#[tauri::command]
pub async fn set_launch_at_login(enabled: bool, state: State<'_, AppState>) -> Result<(), String> {
    let repo = database(&state).settings_repository();
    repo.upsert("launch_at_login", &enabled.to_string())
        .await
        .map_err(|err| err.to_string())
}
