use sqlx::SqlitePool;

use super::models::{Setting, SettingRow};
use super::DbResult;

#[derive(Clone)]
pub struct SettingsRepository {
    pool: SqlitePool,
}

impl SettingsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> DbResult<Vec<Setting>> {
        let rows = sqlx::query_as!(SettingRow, r#"SELECT * FROM settings"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Setting::from).collect())
    }

    pub async fn get(&self, key: &str) -> DbResult<Option<Setting>> {
        let row = sqlx::query_as!(
            SettingRow,
            r#"SELECT * FROM settings WHERE key = ?"#,
            key
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Setting::from))
    }

    pub async fn upsert(&self, key: &str, value: &str) -> DbResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO settings (key, value)
                VALUES (?, ?)
                ON CONFLICT(key) DO UPDATE SET
                    value = excluded.value,
                    updated_at = CURRENT_TIMESTAMP
            "#,
            key,
            value
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, key: &str) -> DbResult<()> {
        sqlx::query!(r#"DELETE FROM settings WHERE key = ?"#, key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
