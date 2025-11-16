use chrono::Weekday;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RepeatType {
    Once,
    Daily,
    Weekly { days: Vec<Weekday> },
    Weekdays,
    Weekends,
    Custom { interval_minutes: u32 },
}

impl RepeatType {
    pub fn repeat_days(&self) -> Option<Vec<u8>> {
        match self {
            RepeatType::Weekly { days } => Some(
                days.iter()
                    .map(|d| d.num_days_from_sunday() as u8)
                    .collect(),
            ),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: String,
    pub name: String,
    pub audio_file_path: String,
    pub scheduled_time: String,
    pub enabled: bool,
    pub repeat_type: RepeatType,
    pub volume: u8,
    pub created_at: String,
    pub updated_at: String,
    pub last_run_at: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct ScheduleRow {
    pub id: String,
    pub name: String,
    pub audio_file_path: String,
    pub scheduled_time: String,
    pub enabled: i64,
    pub repeat_type: String,
    pub repeat_days: Option<String>,
    pub volume: i64,
    pub created_at: String,
    pub updated_at: String,
    pub last_run_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScheduleInput {
    pub name: String,
    pub audio_file_path: String,
    pub scheduled_time: String,
    pub enabled: bool,
    pub repeat_type: RepeatType,
    pub volume: u8,
    pub last_run_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateScheduleInput {
    pub name: Option<String>,
    pub audio_file_path: Option<String>,
    pub scheduled_time: Option<String>,
    pub enabled: Option<bool>,
    pub repeat_type: Option<RepeatType>,
    pub volume: Option<u8>,
    pub last_run_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct SettingRow {
    pub key: String,
    pub value: String,
    pub updated_at: String,
}

impl From<SettingRow> for Setting {
    fn from(row: SettingRow) -> Self {
        Self {
            key: row.key,
            value: row.value,
            updated_at: row.updated_at,
        }
    }
}

impl From<Vec<Setting>> for SettingsSnapshot {
    fn from(records: Vec<Setting>) -> Self {
        let mut snapshot = SettingsSnapshot::default();
        for setting in records {
            match setting.key.as_str() {
                "theme" => snapshot.theme = setting.value,
                "launch_at_login" => {
                    snapshot.launch_at_login = setting.value == "true"
                }
                "minimize_to_tray" => {
                    snapshot.minimize_to_tray = setting.value == "true"
                }
                "show_notifications" => {
                    snapshot.show_notifications = setting.value == "true"
                }
                "notification_sound" => {
                    snapshot.notification_sound = setting.value == "true"
                }
                "default_volume" => {
                    if let Ok(value) = setting.value.parse::<u8>() {
                        snapshot.default_volume = value.min(100);
                    }
                }
                "announcement_enabled" => {
                    snapshot.announcement_enabled = setting.value == "true"
                }
                "announcement_sound" => {
                    snapshot.announcement_sound = setting.value
                }
                _ => {}
            }
        }

        snapshot
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSnapshot {
    pub theme: String,
    pub launch_at_login: bool,
    pub minimize_to_tray: bool,
    pub show_notifications: bool,
    pub notification_sound: bool,
    pub default_volume: u8,
    pub announcement_enabled: bool,
    pub announcement_sound: String,
}

impl Default for SettingsSnapshot {
    fn default() -> Self {
        Self {
            theme: "system".into(),
            launch_at_login: false,
            minimize_to_tray: true,
            show_notifications: true,
            notification_sound: true,
            default_volume: 80,
            announcement_enabled: false,
            announcement_sound: "spell".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlaybackStatus {
    Success,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackHistory {
    pub id: String,
    pub schedule_id: String,
    pub played_at: String,
    pub status: PlaybackStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct PlaybackHistoryRow {
    pub id: String,
    pub schedule_id: String,
    pub played_at: String,
    pub status: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Error)]
pub enum ModelConversionError {
    #[error("unknown playback status '{0}'")]
    UnknownPlaybackStatus(String),
}

impl TryFrom<ScheduleRow> for Schedule {
    type Error = serde_json::Error;

    fn try_from(row: ScheduleRow) -> Result<Self, Self::Error> {
        let repeat_type: RepeatType = serde_json::from_str(&row.repeat_type)?;

        Ok(Schedule {
            id: row.id,
            name: row.name,
            audio_file_path: row.audio_file_path,
            scheduled_time: row.scheduled_time,
            enabled: row.enabled != 0,
            repeat_type,
            volume: row.volume as u8,
            created_at: row.created_at,
            updated_at: row.updated_at,
            last_run_at: row.last_run_at,
        })
    }
}

impl TryFrom<PlaybackHistoryRow> for PlaybackHistory {
    type Error = ModelConversionError;

    fn try_from(row: PlaybackHistoryRow) -> Result<Self, Self::Error> {
        let status = match row.status.as_str() {
            "success" => PlaybackStatus::Success,
            "failed" => PlaybackStatus::Failed,
            "skipped" => PlaybackStatus::Skipped,
            other => {
                return Err(ModelConversionError::UnknownPlaybackStatus(
                    other.to_string(),
                ))
            }
        };

        Ok(Self {
            id: row.id,
            schedule_id: row.schedule_id,
            played_at: row.played_at,
            status,
            error_message: row.error_message,
        })
    }
}
