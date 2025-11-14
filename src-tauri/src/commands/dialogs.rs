use tauri::Window;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn open_audio_file_dialog(window: Window) -> Result<Option<String>, String> {
    let selection = window
        .dialog()
        .file()
        .add_filter(
            "Audio files",
            &["mp3", "wav", "flac", "ogg", "m4a", "aac", "oga"],
        )
        .blocking_pick_file();

    Ok(selection.map(|path| path.to_string()))
}
