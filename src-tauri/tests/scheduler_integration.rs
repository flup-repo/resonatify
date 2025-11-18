use resonatify_lib::db::models::{CreateScheduleInput, RepeatType};
use resonatify_lib::db::{Database, DatabaseError};
use resonatify_lib::scheduler::engine::{AudioController, SchedulerEngine};
use resonatify_lib::scheduler::SchedulerError as SchedulerErrorEnum;

use async_trait::async_trait;
use chrono::{Duration, Local};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration as StdDuration;
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
    async fn play(&self, _path: &str, volume: u8) -> Result<(), SchedulerErrorEnum> {
        self.plays.fetch_add(1, Ordering::SeqCst);
        let mut guard = self.last_volume.lock().await;
        *guard = Some(volume);
        Ok(())
    }

    async fn is_playing(&self) -> bool {
        false
    }
}

async fn setup_database() -> Result<Database, DatabaseError> {
    let connect_options = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(connect_options)
        .await?;
    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(Database::new(pool))
}

#[tokio::test]
async fn test_scheduler_flow_daily_repeat() {
    let database = setup_database().await.expect("Failed to setup DB");
    let schedule_repo = database.schedule_repository();

    // 1. Create a schedule set to run very soon
    let now = Local::now();
    // 1 second in the future
    let scheduled_time = (now + Duration::seconds(1)).format("%H:%M").to_string();

    let schedule = schedule_repo
        .create(CreateScheduleInput {
            name: "Daily Test".into(),
            audio_file_path: "test.mp3".into(),
            scheduled_time,
            enabled: true,
            repeat_type: RepeatType::Daily,
            volume: 50,
            last_run_at: None,
        })
        .await
        .expect("Failed to create schedule");

    // 2. Setup Scheduler with Mock Audio
    let audio_mock = Arc::new(MockAudioController::new());
    let controller: Arc<dyn AudioController> = audio_mock.clone();
    
    // we don't have AudioService instance, passing None is fine for the mock-based test
    // provided we use `with_audio_controller`
    let scheduler = SchedulerEngine::with_audio_controller(
        database.clone(),
        controller,
        None,
        None
    );

    // 3. Start Scheduler
    scheduler.start().await.expect("Failed to start scheduler");

    // 4. Verify it's running
    let status = scheduler.status().await;
    assert!(status.is_running);
    assert_eq!(status.total_schedules, 1);
    assert_eq!(status.schedules[0].id, schedule.id);

    // 5. Wait for execution (allow 2 seconds for the 1s delay + processing)
    tokio::time::sleep(StdDuration::from_secs(3)).await;

    // 6. Verify playback occurred
    assert_eq!(audio_mock.play_count(), 1, "Audio should have played once");

    // 7. Verify Schedule state
    let updated_schedule = schedule_repo.get_by_id(&schedule.id).await.unwrap();
    assert!(updated_schedule.enabled, "Daily schedule should remain enabled");
    
    // Check history
    let history = database.playback_history_repository().list_recent(10).await.unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].schedule_id, schedule.id);
    assert_eq!(history[0].status, resonatify_lib::db::models::PlaybackStatus::Success);

    // 8. Stop scheduler
    scheduler.stop().await.expect("Failed to stop scheduler");
}
