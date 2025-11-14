pub mod models;
pub mod playback_history;
pub mod schedules;
pub mod settings;

use sqlx::migrate::MigrateError;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::str::FromStr;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

use self::models::ModelConversionError;
use self::playback_history::PlaybackHistoryRepository;
use self::schedules::ScheduleRepository;
use self::settings::SettingsRepository;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("failed to resolve application data directory")]
    AppDirectoryNotFound,
    #[error("failed to create database directory: {0}")]
    Io(#[from] std::io::Error),
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("invalid data: {0}")]
    InvalidData(String),
    #[error("migration error: {0}")]
    Migration(#[from] MigrateError),
}

pub type DbResult<T> = Result<T, DatabaseError>;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub fn schedule_repository(&self) -> ScheduleRepository {
        ScheduleRepository::new(self.pool.clone())
    }

    pub fn settings_repository(&self) -> SettingsRepository {
        SettingsRepository::new(self.pool.clone())
    }

    pub fn playback_history_repository(&self) -> PlaybackHistoryRepository {
        PlaybackHistoryRepository::new(self.pool.clone())
    }
}

pub async fn init_db(app_handle: &AppHandle) -> DbResult<Database> {
    let db_path = resolve_database_path(app_handle)?;

    if let Some(parent) = db_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let database_url = format!("sqlite://{}", db_path.to_string_lossy());

    let connect_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(Database::new(pool))
}

fn resolve_database_path(app_handle: &AppHandle) -> DbResult<PathBuf> {
    app_handle
        .path()
        .resolve("resonatify.db", BaseDirectory::AppData)
        .map_err(|_| DatabaseError::AppDirectoryNotFound)
}

impl From<ModelConversionError> for DatabaseError {
    fn from(error: ModelConversionError) -> Self {
        match error {
            ModelConversionError::UnknownPlaybackStatus(status) => {
                DatabaseError::InvalidData(format!("unknown playback status '{status}'"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::{CreateScheduleInput, RepeatType};

    async fn setup_test_database() -> Database {
        let connect_options = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(connect_options)
            .await
            .unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        Database::new(pool)
    }

    #[tokio::test]
    async fn migrations_apply_successfully() {
        let _database = setup_test_database().await;
    }

    #[tokio::test]
    async fn schedule_repository_round_trip() {
        let database = setup_test_database().await;
        let repo = database.schedule_repository();

        let created = repo
            .create(CreateScheduleInput {
                name: "Morning Meditation".into(),
                audio_file_path: "/Users/example/meditation.mp3".into(),
                scheduled_time: "07:00".into(),
                enabled: true,
                repeat_type: RepeatType::Daily,
                volume: 80,
                last_run_at: None,
            })
            .await
            .unwrap();

        assert_eq!(created.name, "Morning Meditation");

        let fetched = repo.get_by_id(&created.id).await.unwrap();
        assert_eq!(fetched.id, created.id);

        let all = repo.get_all().await.unwrap();
        assert_eq!(all.len(), 1);

        repo.delete(&created.id).await.unwrap();

        let none = repo.get_by_id(&created.id).await.err();
        assert!(none.is_some());
    }

    #[tokio::test]
    async fn settings_repository_upsert_and_get() {
        let database = setup_test_database().await;
        let repo = database.settings_repository();

        repo.upsert("theme", "dark").await.unwrap();
        repo.upsert("launch_at_login", "true").await.unwrap();

        let theme = repo.get("theme").await.unwrap().unwrap();
        assert_eq!(theme.value, "dark");

        let all = repo.get_all().await.unwrap();
        assert_eq!(all.len(), 2);

        repo.delete("launch_at_login").await.unwrap();
        let missing = repo.get("launch_at_login").await.unwrap();
        assert!(missing.is_none());
    }

    #[tokio::test]
    async fn playback_history_repository_records_entries() {
        let database = setup_test_database().await;
        let schedules = database.schedule_repository();
        let history = database.playback_history_repository();

        let schedule = schedules
            .create(CreateScheduleInput {
                name: "Focus".into(),
                audio_file_path: "/tmp/focus.mp3".into(),
                scheduled_time: "09:00".into(),
                enabled: true,
                repeat_type: RepeatType::Once,
                volume: 100,
                last_run_at: None,
            })
            .await
            .unwrap();

        let record = history
            .record(&schedule.id, super::models::PlaybackStatus::Success, None)
            .await
            .unwrap();

        assert_eq!(record.schedule_id, schedule.id);
        assert_eq!(record.status, super::models::PlaybackStatus::Success);

        let recent = history.list_recent(10).await.unwrap();
        assert_eq!(recent.len(), 1);

        history.delete_for_schedule(&schedule.id).await.unwrap();

        let recent_after_delete = history.list_recent(10).await.unwrap();
        assert!(recent_after_delete.is_empty());
    }
}
