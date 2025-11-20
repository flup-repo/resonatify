use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration as StdDuration;

use async_trait::async_trait;
use chrono::{DateTime, Local};
use serde::Serialize;
use tauri::Manager;
use tauri_plugin_notification::NotificationExt;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::audio::AudioService;
use crate::db::models::{PlaybackStatus, RepeatType, Schedule, UpdateScheduleInput};
use crate::db::Database;

use super::error::SchedulerError;
use super::time_calculator::next_execution_time;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleStatus {
    #[default]
    Idle,
    Waiting,
    Running,
    Disabled,
    Error,
    Stopped,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleRuntimeInfo {
    pub id: String,
    pub name: String,
    pub repeat_type: RepeatType,
    pub enabled: bool,
    pub next_run: Option<DateTime<Local>>,
    pub last_run: Option<DateTime<Local>>,
    pub status: ScheduleStatus,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct ScheduleExecutionState {
    next_run: Option<DateTime<Local>>,
    last_run: Option<DateTime<Local>>,
    status: ScheduleStatus,
    last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SchedulerStatus {
    pub is_running: bool,
    pub total_schedules: usize,
    pub schedules: Vec<ScheduleRuntimeInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpcomingExecution {
    pub schedule_id: String,
    pub name: String,
    pub scheduled_for: DateTime<Local>,
    pub repeat_type: RepeatType,
}

#[async_trait]
pub trait AudioController: Send + Sync + 'static {
    async fn play(&self, path: &str, volume: u8) -> Result<(), SchedulerError>;
    async fn is_playing(&self) -> bool;
}

#[async_trait]
impl AudioController for AudioService {
    async fn play(&self, path: &str, volume: u8) -> Result<(), SchedulerError> {
        self.play(path.to_string(), volume).await?;
        Ok(())
    }

    async fn is_playing(&self) -> bool {
        self.status().await.is_playing
    }
}

struct ScheduleData {
    schedule: RwLock<Schedule>,
    state: RwLock<ScheduleExecutionState>,
}

impl ScheduleData {
    fn new(schedule: Schedule) -> Self {
        Self {
            schedule: RwLock::new(schedule),
            state: RwLock::new(ScheduleExecutionState::default()),
        }
    }

    async fn schedule(&self) -> Schedule {
        self.schedule.read().await.clone()
    }

    async fn update_schedule(&self, schedule: Schedule) {
        let mut guard = self.schedule.write().await;
        *guard = schedule;
    }

    async fn state(&self) -> ScheduleExecutionState {
        self.state.read().await.clone()
    }

    async fn update_state<F>(&self, mut updater: F)
    where
        F: FnMut(&mut ScheduleExecutionState),
    {
        let mut guard = self.state.write().await;
        updater(&mut guard);
    }
}

struct ActiveSchedule {
    data: Arc<ScheduleData>,
    cancel_token: CancellationToken,
    handle: tauri::async_runtime::JoinHandle<()>,
}

#[derive(Default)]
struct EngineState {
    running: bool,
    schedules: HashMap<String, ActiveSchedule>,
}

struct SchedulerInner {
    database: Database,
    audio: Arc<dyn AudioController>,
    audio_service: Option<AudioService>, // Store concrete type for announcement validation
    app_handle: Option<tauri::AppHandle>,
    state: RwLock<EngineState>,
}

#[derive(Clone)]
pub struct SchedulerEngine {
    inner: Arc<SchedulerInner>,
}

impl SchedulerEngine {
    pub fn new(database: Database, audio: AudioService) -> Self {
        let audio_service = Some(audio.clone());
        let controller: Arc<dyn AudioController> = Arc::new(audio);
        Self::with_audio_controller(database, controller, audio_service, None)
    }

    pub fn new_with_app(database: Database, audio: AudioService, app_handle: tauri::AppHandle) -> Self {
        let audio_service = Some(audio.clone());
        let controller: Arc<dyn AudioController> = Arc::new(audio);
        Self::with_audio_controller(database, controller, audio_service, Some(app_handle))
    }

    pub fn with_audio_controller(database: Database, audio: Arc<dyn AudioController>, audio_service: Option<AudioService>, app_handle: Option<tauri::AppHandle>) -> Self {
        let inner = SchedulerInner {
            database,
            audio,
            audio_service,
            app_handle,
            state: RwLock::new(EngineState::default()),
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub async fn start(&self) -> Result<(), SchedulerError> {
        let schedules = self
            .inner
            .database
            .schedule_repository()
            .get_enabled()
            .await?;

        let mut active = Vec::new();

        for schedule in schedules {
            let data = Arc::new(ScheduleData::new(schedule.clone()));
            let cancel_token = CancellationToken::new();
            let task_data = Arc::clone(&data);
            let task_db = self.inner.database.clone();
            let task_audio = Arc::clone(&self.inner.audio);
            let task_app = self.inner.app_handle.clone();
            let task_cancel = cancel_token.clone();

            let task_audio_service = self.inner.audio_service.clone();
            let handle = tauri::async_runtime::spawn(async move {
                run_schedule_task(task_data, task_db, task_audio, task_audio_service, task_app, task_cancel).await;
            });

            active.push((
                schedule.id.clone(),
                ActiveSchedule {
                    data,
                    cancel_token,
                    handle,
                },
            ));
        }

        let mut state = self.inner.state.write().await;
        if state.running {
            return Err(SchedulerError::AlreadyRunning);
        }

        state.running = true;
        state.schedules = active.into_iter().collect();

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), SchedulerError> {
        let schedules = {
            let mut state = self.inner.state.write().await;
            if !state.running {
                return Err(SchedulerError::NotRunning);
            }

            state.running = false;
            std::mem::take(&mut state.schedules)
        };

        for (_, active) in schedules {
            active.cancel_token.cancel();
            let _ = active.handle.await;
        }

        Ok(())
    }

    pub async fn reload(&self) -> Result<(), SchedulerError> {
        let should_restart = {
            let state = self.inner.state.read().await;
            state.running
        };

        if should_restart {
            self.stop().await?;
            self.start().await?;
        } else {
            let mut state = self.inner.state.write().await;
            state.schedules.clear();
        }

        Ok(())
    }

    pub async fn status(&self) -> SchedulerStatus {
        let (is_running, handles) = {
            let state = self.inner.state.read().await;
            let handles = state
                .schedules
                .values()
                .map(|active| Arc::clone(&active.data))
                .collect::<Vec<_>>();
            (state.running, handles)
        };

        let mut schedules = Vec::with_capacity(handles.len());
        for data in handles {
            let schedule = data.schedule().await;
            let state = data.state().await;

            let Schedule {
                id,
                name,
                repeat_type,
                enabled,
                ..
            } = schedule;

            schedules.push(ScheduleRuntimeInfo {
                id,
                name,
                repeat_type,
                enabled,
                next_run: state.next_run,
                last_run: state.last_run,
                status: state.status,
                last_error: state.last_error,
            });
        }

        SchedulerStatus {
            is_running,
            total_schedules: schedules.len(),
            schedules,
        }
    }

    pub async fn upcoming_executions(&self, count: usize) -> Vec<UpcomingExecution> {
        let handles = {
            let state = self.inner.state.read().await;
            state
                .schedules
                .values()
                .map(|active| Arc::clone(&active.data))
                .collect::<Vec<_>>()
        };

        let mut upcoming = Vec::new();
        for data in handles {
            let schedule = data.schedule().await;
            let state = data.state().await;

            let Schedule {
                id,
                name,
                repeat_type,
                ..
            } = schedule;

            if let Some(next_run) = state.next_run {
                upcoming.push(UpcomingExecution {
                    schedule_id: id,
                    name,
                    scheduled_for: next_run,
                    repeat_type,
                });
            }
        }

        upcoming.sort_by_key(|entry| entry.scheduled_for);
        upcoming.truncate(count);
        upcoming
    }

    /// Pause all active schedules (stops execution temporarily without disabling in database)
    pub async fn pause_all(&self) -> Result<(), SchedulerError> {
        let is_running = {
            let state = self.inner.state.read().await;
            state.running
        };

        if !is_running {
            return Err(SchedulerError::NotRunning);
        }

        // Stop the engine (cancels all tasks)
        self.stop().await?;

        Ok(())
    }

    /// Resume all schedules (restarts the engine)
    pub async fn resume_all(&self) -> Result<(), SchedulerError> {
        let is_running = {
            let state = self.inner.state.read().await;
            state.running
        };

        if is_running {
            return Err(SchedulerError::AlreadyRunning);
        }

        // Start the engine again
        self.start().await?;

        Ok(())
    }

    /// Get upcoming executions (wrapper for commands module)
    pub async fn get_upcoming_executions(
        &self,
        count: usize,
    ) -> Result<Vec<UpcomingExecution>, SchedulerError> {
        Ok(self.upcoming_executions(count).await)
    }
}

async fn send_notification(
    app_handle: &Option<tauri::AppHandle>,
    database: &Database,
    title: &str,
    body: &str,
) {
    if let Some(app) = app_handle {
        // Check settings to see if notifications are enabled
        match database.settings_repository().get("show_notifications").await {
            Ok(Some(setting)) if setting.value == "true" => {
                // Send notification using tauri-plugin-notification
                if let Err(e) = app.notification()
                    .builder()
                    .title(title)
                    .body(body)
                    .show() {
                    eprintln!("Failed to send notification: {}", e);
                }
            }
            _ => {
                // Notifications disabled or error reading setting
            }
        }
    }
}

async fn run_schedule_task(
    data: Arc<ScheduleData>,
    database: Database,
    audio: Arc<dyn AudioController>,
    audio_service: Option<AudioService>,
    app_handle: Option<tauri::AppHandle>,
    cancel_token: CancellationToken,
) {
    let playback_repo = database.playback_history_repository();
    let schedule_repo = database.schedule_repository();

    loop {
        let schedule = data.schedule().await;
        let state_snapshot = data.state().await;

        if cancel_token.is_cancelled() {
            data.update_state(|state| {
                state.status = ScheduleStatus::Stopped;
                state.next_run = None;
            })
            .await;
            break;
        }

        if !schedule.enabled {
            data.update_state(|state| {
                state.status = ScheduleStatus::Disabled;
                state.next_run = None;
            })
            .await;
            break;
        }

        let now = Local::now();
        match next_execution_time(&schedule, now, state_snapshot.last_run) {
            Ok(Some(next_run)) => {
                data.update_state(|state| {
                    state.next_run = Some(next_run);
                    state.status = ScheduleStatus::Waiting;
                    state.last_error = None;
                })
                .await;

                let wait = match (next_run - now).to_std() {
                    Ok(duration) => duration,
                    Err(_) => StdDuration::from_secs(0),
                };

                tokio::select! {
                    _ = tokio::time::sleep(wait) => {}
                    _ = cancel_token.cancelled() => {
                        data.update_state(|state| {
                            state.status = ScheduleStatus::Stopped;
                            state.next_run = None;
                        }).await;
                        break;
                    }
                }

                if cancel_token.is_cancelled() {
                    data.update_state(|state| {
                        state.status = ScheduleStatus::Stopped;
                        state.next_run = None;
                    })
                    .await;
                    break;
                }

                data.update_state(|state| {
                    state.status = ScheduleStatus::Running;
                    state.next_run = None;
                })
                .await;

                // Check if announcement is enabled and play announcement first
                let settings_repo = database.settings_repository();
                let settings = settings_repo.get_all().await;
                if let Ok(settings_list) = settings {
                    let settings_snapshot: crate::db::models::SettingsSnapshot = settings_list.into();
                    if settings_snapshot.announcement_enabled && app_handle.is_some() {
                        // Get the announcement sound filename
                        let announcement_filename = match settings_snapshot.announcement_sound.as_str() {
                            "spell" => "light-spell-notifiation.wav",
                            _ => "light-spell-notifiation.wav", // Default to spell
                        };

                        // Resolve the announcement audio path
                        let announcement_path = if let Some(ref handle) = app_handle {
                            // Try production resource path first
                            match handle.path().resolve(announcement_filename, tauri::path::BaseDirectory::Resource) {
                                Ok(resource_path) if resource_path.exists() => {
                                    Some(resource_path)
                                }
                                _ => {
                                    // Fallback to dev mode: use path relative to current executable
                                    std::env::current_exe().ok().and_then(|exe_path| {
                                        // Go up from target/debug/resonatify to target/debug
                                        // Then to target, then to src-tauri, then to resources
                                        exe_path.parent().map(|dir| {
                                            dir.join("../../resources").join(announcement_filename)
                                        })
                                    }).and_then(|p| {
                                        // Canonicalize to resolve .. in path
                                        p.canonicalize().ok()
                                    })
                                }
                            }
                        } else {
                            None
                        };

                        if let Some(path) = announcement_path {
                            if let Some(path_str) = path.to_str() {
                                // Get the announcement duration by validating the audio file
                                let wait_duration = if let Some(ref service) = audio_service {
                                    // Validate to get metadata including duration
                                    match service.validate(path_str).await {
                                        Ok(metadata) => {
                                            if let Some(duration_ms) = metadata.duration_ms {
                                                // Use actual duration + 500ms buffer for smooth transition and startup latency
                                                Some(StdDuration::from_millis(duration_ms + 500))
                                            } else {
                                                // Duration unknown, use 3 second default
                                                eprintln!("Announcement duration unknown, using 3s default");
                                                Some(StdDuration::from_millis(3000))
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to validate announcement audio: {}, using 3s default", e);
                                            Some(StdDuration::from_millis(3000))
                                        }
                                    }
                                } else {
                                    // Fallback if we don't have audio service
                                    eprintln!("No audio service available for validation, using 3s default");
                                    Some(StdDuration::from_millis(3000))
                                };

                                // Play announcement at default volume with short fade-in
                                let play_result = if let Some(ref service) = audio_service {
                                    service.play_with_fade(path_str, 80, StdDuration::from_millis(50))
                                        .await
                                        .map(|_| ())
                                        .map_err(SchedulerError::Audio)
                                } else {
                                    audio.play(path_str, 80).await
                                };

                                match play_result {
                                    Ok(_) => {
                                        if let Some(duration) = wait_duration {
                                            // Wait for the announcement to finish based on its actual duration
                                            eprintln!("Playing announcement, waiting {}ms", duration.as_millis());
                                            tokio::time::sleep(duration).await;
                                            eprintln!("Announcement finished, playing scheduled audio");
                                        }
                                    }
                                    Err(e) => {
                                        // Log error but continue with scheduled audio
                                        eprintln!("Failed to play announcement: {}", e);
                                    }
                                }

                            }
                        } else {
                            eprintln!("Could not resolve announcement audio path for: {}", announcement_filename);
                        }
                    }
                }

                let play_result = audio.play(&schedule.audio_file_path, schedule.volume).await;

                match play_result {
                    Ok(_) => {
                        let executed_at = Local::now();

                        let _ = playback_repo
                            .record(&schedule.id, PlaybackStatus::Success, None)
                            .await;

                        // Send success notification
                        send_notification(
                            &app_handle,
                            &database,
                            "Schedule Executed",
                            &format!("\"{}\" played successfully", schedule.name),
                        ).await;

                        data.update_state(|state| {
                            state.last_run = Some(executed_at);
                            state.status = ScheduleStatus::Idle;
                        })
                        .await;
                    }
                    Err(err) => {
                        let message = err.to_string();
                        let _ = playback_repo
                            .record(&schedule.id, PlaybackStatus::Failed, Some(message.clone()))
                            .await;

                        // Send failure notification
                        send_notification(
                            &app_handle,
                            &database,
                            "Schedule Failed",
                            &format!("\"{}\" failed to play: {}", schedule.name, message),
                        ).await;

                        data.update_state(|state| {
                            state.last_error = Some(message.clone());
                            state.status = ScheduleStatus::Error;
                        })
                        .await;
                    }
                }

                if matches!(schedule.repeat_type, RepeatType::Once) {
                    if schedule.enabled {
                        match schedule_repo
                            .update(
                                &schedule.id,
                                UpdateScheduleInput {
                                    enabled: Some(false),
                                    ..Default::default()
                                },
                            )
                            .await
                        {
                            Ok(updated) => {
                                data.update_schedule(updated).await;
                                data.update_state(|state| {
                                    state.status = ScheduleStatus::Disabled;
                                    state.next_run = None;
                                })
                                .await;
                            }
                            Err(err) => {
                                let message = err.to_string();
                                data.update_state(|state| {
                                    state.last_error = Some(message.clone());
                                    state.status = ScheduleStatus::Error;
                                })
                                .await;
                            }
                        }
                    }

                    break;
                }
            }
            Ok(None) => {
                data.update_state(|state| {
                    state.status = ScheduleStatus::Disabled;
                    state.next_run = None;
                })
                .await;
                break;
            }
            Err(err) => {
                let message = err.to_string();
                let _ = playback_repo
                    .record(&schedule.id, PlaybackStatus::Skipped, Some(message.clone()))
                    .await;

                data.update_state(|state| {
                    state.last_error = Some(message.clone());
                    state.status = ScheduleStatus::Error;
                })
                .await;

                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::{CreateScheduleInput, RepeatType};
    use crate::db::{Database, DatabaseError};
    use chrono::Local;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::sync::Mutex;

    struct MockAudioController {
        plays: AtomicUsize,
        last_volume: Mutex<Option<u8>>,
    }

    impl MockAudioController {
        fn new() -> Self {
            Self {
                plays: AtomicUsize::new(0),
                last_volume: Mutex::new(None),
            }
        }

        fn play_count(&self) -> usize {
            self.plays.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl AudioController for MockAudioController {
        async fn play(&self, _path: &str, volume: u8) -> Result<(), SchedulerError> {
            self.plays.fetch_add(1, Ordering::SeqCst);
            let mut guard = self.last_volume.lock().await;
            *guard = Some(volume);
            Ok(())
        }

        async fn is_playing(&self) -> bool {
            // Mock always returns false (not playing)
            false
        }
    }

    async fn setup_database() -> Result<Database, DatabaseError> {
        use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
        use std::str::FromStr;

        let connect_options = SqliteConnectOptions::from_str("sqlite::memory:")?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(connect_options)
            .await
            .unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        Ok(Database::new(pool))
    }

    fn current_time_string() -> String {
        Local::now().format("%H:%M").to_string()
    }

    #[tokio::test]
    async fn scheduler_executes_once_schedule() {
        let database = setup_database().await.unwrap();
        let schedule_repo = database.schedule_repository();

        let schedule = schedule_repo
            .create(CreateScheduleInput {
                name: "Test".into(),
                audio_file_path: "/tmp/test.mp3".into(),
                scheduled_time: current_time_string(),
                enabled: true,
                repeat_type: RepeatType::Once,
                volume: 70,
                last_run_at: None,
            })
            .await
            .unwrap();

        let audio = Arc::new(MockAudioController::new());
        let controller: Arc<dyn AudioController> = audio.clone();
        let scheduler = SchedulerEngine::with_audio_controller(database.clone(), controller, None, None);

        scheduler.start().await.unwrap();

        tokio::time::sleep(StdDuration::from_millis(250)).await;

        scheduler.stop().await.unwrap();

        assert!(audio.play_count() >= 1);

        let updated = schedule_repo.get_by_id(&schedule.id).await.unwrap();
        assert!(!updated.enabled);

        let history = database
            .playback_history_repository()
            .list_recent(1)
            .await
            .unwrap();

        assert!(!history.is_empty());
        assert_eq!(history[0].schedule_id, schedule.id);
    }
}
