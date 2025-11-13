use crate::audio::error::AudioError;
use crate::db::DatabaseError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("scheduler already running")]
    AlreadyRunning,
    #[error("scheduler is not running")]
    NotRunning,
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),
    #[error("audio error: {0}")]
    Audio(#[from] AudioError),
    #[error("invalid schedule time for {schedule_id}: {reason}")]
    InvalidScheduleTime { schedule_id: String, reason: String },
    #[error("failed to join schedule task: {0}")]
    TaskJoin(String),
    #[error("internal scheduler error: {0}")]
    Internal(String),
}

impl From<JoinError> for SchedulerError {
    fn from(error: JoinError) -> Self {
        SchedulerError::TaskJoin(error.to_string())
    }
}
