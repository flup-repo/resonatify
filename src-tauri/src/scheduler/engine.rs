use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration as StdDuration;

use async_trait::async_trait;
use chrono::{DateTime, Local};
use serde::Serialize;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::audio::AudioService;
use crate::db::models::{PlaybackStatus, RepeatType, Schedule, UpdateScheduleInput};
use crate::db::Database;

use super::error::SchedulerError;
use super::time_calculator::next_execution_time;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleStatus {
    Idle,
    Waiting,
    Running,
    Disabled,
    Error,
    Stopped,
}

impl Default for ScheduleStatus {
    fn default() -> Self {
        ScheduleStatus::Idle
    }
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
}

#[async_trait]
impl AudioController for AudioService {
    async fn play(&self, path: &str, volume: u8) -> Result<(), SchedulerError> {
        self.play(path.to_string(), volume).await?;
        Ok(())
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
    state: RwLock<EngineState>,
}

#[derive(Clone)]
pub struct SchedulerEngine {
    inner: Arc<SchedulerInner>,
}

impl SchedulerEngine {
    pub fn new(database: Database, audio: AudioService) -> Self {
        let controller: Arc<dyn AudioController> = Arc::new(audio);
        Self::with_audio_controller(database, controller)
    }

    pub fn with_audio_controller(database: Database, audio: Arc<dyn AudioController>) -> Self {
        let inner = SchedulerInner {
            database,
            audio,
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
            let task_cancel = cancel_token.clone();

            let handle = tauri::async_runtime::spawn(async move {
                run_schedule_task(task_data, task_db, task_audio, task_cancel).await;
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
}

async fn run_schedule_task(
    data: Arc<ScheduleData>,
    database: Database,
    audio: Arc<dyn AudioController>,
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

                let play_result = audio.play(&schedule.audio_file_path, schedule.volume).await;

                match play_result {
                    Ok(_) => {
                        let executed_at = Local::now();

                        let _ = playback_repo
                            .record(&schedule.id, PlaybackStatus::Success, None)
                            .await;

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
        let scheduler = SchedulerEngine::with_audio_controller(database.clone(), controller);

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
