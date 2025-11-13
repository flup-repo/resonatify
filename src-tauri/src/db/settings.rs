use sqlx::{query, query_as, SqlitePool};

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
        let rows = query_as::<_, SettingRow>(r#"SELECT * FROM settings"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Setting::from).collect())
    }

    pub async fn get(&self, key: &str) -> DbResult<Option<Setting>> {
        let row = query_as::<_, SettingRow>(r#"SELECT * FROM settings WHERE key = ?"#)
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Setting::from))
    }

    pub async fn upsert(&self, key: &str, value: &str) -> DbResult<()> {
        query(
            r#"
                INSERT INTO settings (key, value)
                VALUES (?, ?)
                ON CONFLICT(key) DO UPDATE SET
                    value = excluded.value,
                    updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, key: &str) -> DbResult<()> {
        query(r#"DELETE FROM settings WHERE key = ?"#)
            .bind(key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
