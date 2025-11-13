use chrono::Local;
use sqlx::{query, query_as, SqlitePool};
use uuid::Uuid;

use super::models::{CreateScheduleInput, Schedule, ScheduleRow, UpdateScheduleInput};
use super::{DatabaseError, DbResult};

#[derive(Clone)]
pub struct ScheduleRepository {
    pool: SqlitePool,
}

impl ScheduleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateScheduleInput) -> DbResult<Schedule> {
        let now = Local::now().to_rfc3339();
        let id = Uuid::new_v4().to_string();

        let CreateScheduleInput {
            name,
            audio_file_path,
            scheduled_time,
            enabled,
            repeat_type,
            volume,
        } = input;

        let repeat_type_json = serde_json::to_string(&repeat_type)?;
        let repeat_days_json = repeat_type
            .repeat_days()
            .map(|days| serde_json::to_string(&days))
            .transpose()?;

        query(
            r#"
                INSERT INTO schedules (
                    id,
                    name,
                    audio_file_path,
                    scheduled_time,
                    enabled,
                    repeat_type,
                    repeat_days,
                    volume,
                    created_at,
                    updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(name)
        .bind(audio_file_path)
        .bind(scheduled_time)
        .bind(enabled as i64)
        .bind(repeat_type_json)
        .bind(repeat_days_json)
        .bind(volume as i64)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        self.get_by_id(&id).await
    }

    pub async fn get_all(&self) -> DbResult<Vec<Schedule>> {
        let rows = query_as::<_, ScheduleRow>(r#"SELECT * FROM schedules ORDER BY scheduled_time"#)
            .fetch_all(&self.pool)
            .await?;

        rows.into_iter()
            .map(Schedule::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub async fn get_by_id(&self, id: &str) -> DbResult<Schedule> {
        let row = query_as::<_, ScheduleRow>(r#"SELECT * FROM schedules WHERE id = ?"#)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Schedule::try_from(row).map_err(DatabaseError::from)
    }

    pub async fn get_enabled(&self) -> DbResult<Vec<Schedule>> {
        let rows = query_as::<_, ScheduleRow>(
            r#"SELECT * FROM schedules WHERE enabled = 1 ORDER BY scheduled_time"#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(Schedule::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub async fn update(&self, id: &str, input: UpdateScheduleInput) -> DbResult<Schedule> {
        let mut current = self.get_by_id(id).await?;

        if let Some(name) = input.name {
            current.name = name;
        }
        if let Some(path) = input.audio_file_path {
            current.audio_file_path = path;
        }
        if let Some(time) = input.scheduled_time {
            current.scheduled_time = time;
        }
        if let Some(enabled) = input.enabled {
            current.enabled = enabled;
        }
        if let Some(repeat_type) = input.repeat_type {
            current.repeat_type = repeat_type;
        }
        if let Some(volume) = input.volume {
            current.volume = volume;
        }

        let now = Local::now().to_rfc3339();
        let repeat_type_json = serde_json::to_string(&current.repeat_type)?;
        let repeat_days_json = current
            .repeat_type
            .repeat_days()
            .map(|days| serde_json::to_string(&days))
            .transpose()?;

        query(
            r#"
                UPDATE schedules
                SET name = ?,
                    audio_file_path = ?,
                    scheduled_time = ?,
                    enabled = ?,
                    repeat_type = ?,
                    repeat_days = ?,
                    volume = ?,
                    updated_at = ?
                WHERE id = ?
            "#,
        )
        .bind(&current.name)
        .bind(&current.audio_file_path)
        .bind(&current.scheduled_time)
        .bind(current.enabled as i64)
        .bind(repeat_type_json)
        .bind(repeat_days_json)
        .bind(current.volume as i64)
        .bind(&now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: &str) -> DbResult<()> {
        query(r#"DELETE FROM schedules WHERE id = ?"#)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
