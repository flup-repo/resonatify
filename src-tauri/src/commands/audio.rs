use tauri::State;

use crate::audio::{AudioFileMetadata, AudioService, PlaybackState};
use crate::AppState;

fn audio_service(state: &State<'_, AppState>) -> AudioService {
    state.audio.clone()
}

#[tauri::command]
pub async fn validate_audio_file(
    path: String,
    state: State<'_, AppState>,
) -> Result<AudioFileMetadata, String> {
    audio_service(&state)
        .validate(path)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn play_audio_file(
    path: String,
    volume: u8,
    state: State<'_, AppState>,
) -> Result<PlaybackState, String> {
    if volume > 100 {
        return Err("volume must be between 0 and 100".into());
    }

    audio_service(&state)
        .play(path, volume)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn stop_audio(state: State<'_, AppState>) -> Result<PlaybackState, String> {
    audio_service(&state)
        .stop()
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn get_audio_status(state: State<'_, AppState>) -> Result<PlaybackState, String> {
    Ok(audio_service(&state).status().await)
}
