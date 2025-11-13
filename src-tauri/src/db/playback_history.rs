use chrono::Local;
use sqlx::{query, query_as, SqlitePool};
use uuid::Uuid;

use super::models::{PlaybackHistory, PlaybackHistoryRow, PlaybackStatus};
use super::DbResult;

#[derive(Clone)]
pub struct PlaybackHistoryRepository {
    pool: SqlitePool,
}

impl PlaybackHistoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn record(
        &self,
        schedule_id: &str,
        status: PlaybackStatus,
        error_message: Option<String>,
    ) -> DbResult<PlaybackHistory> {
        let id = Uuid::new_v4().to_string();
        let status_str = match status {
            PlaybackStatus::Success => "success",
            PlaybackStatus::Failed => "failed",
            PlaybackStatus::Skipped => "skipped",
        };

        query(
            r#"
                INSERT INTO audio_playback_history (
                    id,
                    schedule_id,
                    played_at,
                    status,
                    error_message
                ) VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(schedule_id)
        .bind(Local::now().to_rfc3339())
        .bind(status_str)
        .bind(error_message)
        .execute(&self.pool)
        .await?;

        self.get_by_id(&id).await
    }

    pub async fn get_by_id(&self, id: &str) -> DbResult<PlaybackHistory> {
        let row = query_as::<_, PlaybackHistoryRow>(
            r#"SELECT * FROM audio_playback_history WHERE id = ?"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        PlaybackHistory::try_from(row).map_err(Into::into)
    }

    pub async fn list_recent(&self, limit: i64) -> DbResult<Vec<PlaybackHistory>> {
        let rows = query_as::<_, PlaybackHistoryRow>(
            r#"
                SELECT * FROM audio_playback_history
                ORDER BY played_at DESC
                LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(PlaybackHistory::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub async fn delete_for_schedule(&self, schedule_id: &str) -> DbResult<()> {
        query(r#"DELETE FROM audio_playback_history WHERE schedule_id = ?"#)
            .bind(schedule_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
