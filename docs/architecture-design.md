# Architecture Design: Resonator

**Date**: 2025-11-13
**Phase**: 1.3 Architecture Design
**Status**: Complete
**Version**: 1.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [System Architecture Overview](#system-architecture-overview)
3. [Component Architecture](#component-architecture)
4. [Data Models](#data-models)
5. [API Contracts (Tauri IPC)](#api-contracts-tauri-ipc)
6. [Data Flow Patterns](#data-flow-patterns)
7. [Database Design](#database-design)
8. [Scheduling Engine Architecture](#scheduling-engine-architecture)
9. [Audio Playback Architecture](#audio-playback-architecture)
10. [Frontend Architecture](#frontend-architecture)
11. [Error Handling Strategy](#error-handling-strategy)
12. [Security Considerations](#security-considerations)
13. [Performance Considerations](#performance-considerations)
14. [Architecture Diagrams](#architecture-diagrams)

---

## Executive Summary

Resonator is built using **Tauri 2.0**, combining a **React + TypeScript** frontend with a **Rust** backend. The architecture is designed for:

- **Performance**: <1s startup, <50MB memory, <5% CPU idle
- **Reliability**: 99.9% schedule execution accuracy
- **Scalability**: Support 1000+ schedules without degradation
- **Maintainability**: Clear separation of concerns, testable components
- **Cross-platform readiness**: macOS first, Windows/Linux later

**Key Architectural Decisions:**
- Tauri IPC for frontend-backend communication (message-passing)
- Tokio-based async scheduler engine (background tasks)
- SQLite for data persistence (lightweight, embedded)
- Rodio for audio playback (cross-platform)
- Repository pattern for data access (testable, maintainable)
- Event-driven architecture for real-time updates

---

## System Architecture Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         USER INTERFACE                          │
│                                                                 │
│  ┌───────────────┐  ┌───────────────┐  ┌──────────────────┐   │
│  │  Main Window  │  │  Menu Bar     │  │  Notifications   │   │
│  │  (React)      │  │  Integration  │  │  (System)        │   │
│  └───────────────┘  └───────────────┘  └──────────────────┘   │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │          Zustand State Management Layer                  │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ↕ ↕ ↕
                    Tauri IPC (JSON-based messaging)
                              ↕ ↕ ↕
┌─────────────────────────────────────────────────────────────────┐
│                      RUST BACKEND (Tauri)                       │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                   Tauri Command Layer                     │  │
│  │  (Exposes functions to frontend via #[tauri::command])   │  │
│  └──────────────────────────────────────────────────────────┘  │
│                              ↓                                  │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────────────┐    │
│  │  Scheduler  │  │    Audio     │  │   Data Access      │    │
│  │   Engine    │  │   Playback   │  │   Repositories     │    │
│  │  (Tokio)    │  │   (Rodio)    │  │   (SQLite)         │    │
│  └─────────────┘  └──────────────┘  └────────────────────┘    │
│         ↓                 ↓                      ↓              │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────────────┐    │
│  │  Schedule   │  │  Audio File  │  │   Database         │    │
│  │  Queue      │  │  System      │  │   (scheduler.db)   │    │
│  └─────────────┘  └──────────────┘  └────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                      OPERATING SYSTEM                           │
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐ │
│  │  File System │  │  Audio       │  │  System Tray /       │ │
│  │  (Audio      │  │  Devices     │  │  Launch Agents       │ │
│  │   Files)     │  │              │  │                      │ │
│  └──────────────┘  └──────────────┘  └──────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Technology Stack

**Frontend:**
- **Framework**: React 18.2+ with TypeScript 5.3+
- **Build Tool**: Vite 5.0+
- **Styling**: Tailwind CSS 3.4+
- **Component Library**: shadcn/ui (Radix UI primitives)
- **State Management**: Zustand 4.5+
- **Forms**: react-hook-form 7.50+
- **Date/Time**: date-fns 3.3+
- **Icons**: Lucide React
- **Testing**: Vitest + React Testing Library

**Backend (Rust):**
- **Framework**: Tauri 2.0+
- **Runtime**: tokio 1.35+ (async)
- **Database**: sqlx 0.7+ with SQLite
- **Audio**: rodio 0.17+ (+ symphonia for M4A)
- **Date/Time**: chrono 0.4+
- **Serialization**: serde 1.0+ with serde_json
- **Error Handling**: thiserror 1.0+
- **Logging**: tracing 0.1+
- **Testing**: Built-in Rust testing framework

**Tauri Plugins:**
- `tauri-plugin-autostart` - Launch at login
- `tauri-plugin-notification` - System notifications
- `tauri-plugin-updater` - Auto-updates (future)

---

## Component Architecture

### Frontend Components

```
src/
├── components/
│   ├── layout/
│   │   ├── Header.tsx              # Top bar with app name and actions
│   │   ├── EmptyState.tsx          # No schedules state
│   │   └── Layout.tsx              # Main layout wrapper
│   ├── schedule/
│   │   ├── ScheduleCard.tsx        # Individual schedule card
│   │   ├── ScheduleList.tsx        # List of all schedules
│   │   ├── ScheduleModal.tsx       # Add/Edit schedule form
│   │   └── ScheduleStatus.tsx      # Status indicator (playing, idle, error)
│   ├── settings/
│   │   ├── SettingsLayout.tsx      # Sidebar layout
│   │   ├── GeneralSettings.tsx     # General section
│   │   ├── NotificationSettings.tsx# Notification section
│   │   ├── AudioSettings.tsx       # Audio section
│   │   ├── AboutSettings.tsx       # About section
│   │   └── SupportSettings.tsx     # Support section
│   ├── ui/                         # shadcn/ui components
│   │   ├── button.tsx
│   │   ├── input.tsx
│   │   ├── select.tsx
│   │   ├── slider.tsx
│   │   ├── switch.tsx
│   │   ├── dialog.tsx
│   │   └── toast.tsx
│   └── MenuBarIcon.tsx             # Menu bar integration (Tauri API)
├── stores/
│   ├── scheduleStore.ts            # Schedule state management
│   ├── settingsStore.ts            # Settings state management
│   └── playbackStore.ts            # Playback status state
├── types/
│   ├── schedule.ts                 # Schedule type definitions
│   ├── settings.ts                 # Settings type definitions
│   └── api.ts                      # API response types
├── hooks/
│   ├── useSchedules.ts             # Schedule CRUD operations
│   ├── useSettings.ts              # Settings operations
│   ├── useAudioPlayer.ts           # Audio playback controls
│   └── useKeyboardShortcuts.ts     # Keyboard shortcut handler
├── utils/
│   ├── timeUtils.ts                # Time formatting and calculation
│   ├── scheduleUtils.ts            # Schedule-related utilities
│   └── validation.ts               # Form validation helpers
├── App.tsx                         # Main app component with routing
└── main.tsx                        # Entry point
```

### Backend Components

```
src-tauri/src/
├── main.rs                         # Tauri app entry, registers commands
├── commands/
│   ├── mod.rs                      # Re-exports all command modules
│   ├── schedules.rs                # Schedule CRUD commands
│   ├── settings.rs                 # Settings commands
│   ├── audio.rs                    # Audio playback commands
│   └── scheduler.rs                # Scheduler control commands
├── scheduler/
│   ├── mod.rs                      # Public API
│   ├── engine.rs                   # Main scheduler engine
│   ├── time_calculator.rs          # Next execution time calculation
│   ├── task_manager.rs             # Tokio task management
│   └── models.rs                   # Schedule models and types
├── audio/
│   ├── mod.rs                      # Public API
│   ├── player.rs                   # Audio player implementation
│   ├── validator.rs                # Audio file validation
│   └── error.rs                    # Audio-specific errors
├── db/
│   ├── mod.rs                      # Database initialization
│   ├── models.rs                   # Database models (sqlx)
│   ├── schedules.rs                # Schedule repository
│   ├── settings.rs                 # Settings repository
│   └── migrations/                 # SQL migration files
│       ├── 001_initial.sql
│       └── 002_add_history.sql
├── state.rs                        # Global app state (Arc<Mutex<>>)
├── tray.rs                         # System tray implementation
├── error.rs                        # Global error types
└── lib.rs                          # Library exports for testing
```

---

## Data Models

### Frontend TypeScript Types

```typescript
// src/types/schedule.ts

export type RepeatType = 'once' | 'daily' | 'weekdays' | 'weekends' | 'custom';

export interface Schedule {
  id: string;                    // UUID v4
  name: string;                  // User-friendly name (max 100 chars)
  audioFilePath: string;         // Absolute path to audio file
  scheduledTime: string;         // HH:mm format (24-hour)
  enabled: boolean;              // Schedule is active
  repeatType: RepeatType;        // How schedule repeats
  repeatDays?: number[];         // Days for 'custom' (0=Sun, 6=Sat)
  volume: number;                // 0-100
  createdAt: string;             // ISO 8601 timestamp
  updatedAt: string;             // ISO 8601 timestamp
}

export interface CreateScheduleInput {
  name: string;
  audioFilePath: string;
  scheduledTime: string;
  repeatType: RepeatType;
  repeatDays?: number[];
  volume: number;
  enabled?: boolean;             // Default: true
}

export interface UpdateScheduleInput {
  name?: string;
  audioFilePath?: string;
  scheduledTime?: string;
  repeatType?: RepeatType;
  repeatDays?: number[];
  volume?: number;
  enabled?: boolean;
}

export interface ScheduleExecution {
  scheduleId: string;
  scheduleName: string;
  nextExecutionTime: string;     // ISO 8601 timestamp
}

export interface PlaybackStatus {
  isPlaying: boolean;
  scheduleId?: string;
  scheduleName?: string;
  audioFilePath?: string;
  startedAt?: string;            // ISO 8601 timestamp
}

// src/types/settings.ts

export interface Settings {
  // General
  theme: 'light' | 'dark' | 'system';
  launchAtLogin: boolean;
  startMinimized: boolean;
  minimizeToTray: boolean;

  // Notifications
  showNotifications: boolean;
  notificationSound: boolean;
  showPreNotification: boolean;  // 2 min before
  notificationStyle: 'banner' | 'alert';
  overrideDnd: boolean;          // Do Not Disturb

  // Audio
  defaultVolume: number;         // 0-100
  audioOutputDevice?: string;    // Device ID or 'system'
  fadeIn: boolean;               // Future
  fadeOut: boolean;              // Future
}

export interface SettingsUpdate {
  [key: string]: string | number | boolean;
}

// src/types/api.ts

export interface ApiError {
  message: string;
  code?: string;
}

export type ApiResult<T> = {
  ok: true;
  data: T;
} | {
  ok: false;
  error: ApiError;
};
```

### Backend Rust Types

```rust
// src-tauri/src/scheduler/models.rs

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local, NaiveTime};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RepeatType {
    Once,
    Daily,
    Weekdays,
    Weekends,
    Custom { days: Vec<u8> }, // 0=Sun, 6=Sat
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: String,                    // UUID
    pub name: String,
    pub audio_file_path: String,
    pub scheduled_time: String,        // HH:mm format
    pub enabled: bool,
    pub repeat_type: RepeatType,
    pub volume: u8,                    // 0-100
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduleInput {
    pub name: String,
    pub audio_file_path: String,
    pub scheduled_time: String,
    pub repeat_type: RepeatType,
    pub volume: u8,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct UpdateScheduleInput {
    pub name: Option<String>,
    pub audio_file_path: Option<String>,
    pub scheduled_time: Option<String>,
    pub repeat_type: Option<RepeatType>,
    pub volume: Option<u8>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ScheduleExecution {
    pub schedule_id: String,
    pub schedule_name: String,
    pub next_execution_time: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlaybackStatus {
    pub is_playing: bool,
    pub schedule_id: Option<String>,
    pub schedule_name: Option<String>,
    pub audio_file_path: Option<String>,
    pub started_at: Option<DateTime<Local>>,
}

// src-tauri/src/db/models.rs

use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ScheduleRow {
    pub id: String,
    pub name: String,
    pub audio_file_path: String,
    pub scheduled_time: String,
    pub enabled: i64,              // SQLite BOOLEAN as INTEGER
    pub repeat_type: String,       // JSON serialized
    pub volume: i64,               // SQLite INTEGER
    pub created_at: String,        // ISO 8601 string
    pub updated_at: String,        // ISO 8601 string
}

impl From<ScheduleRow> for Schedule {
    fn from(row: ScheduleRow) -> Self {
        Schedule {
            id: row.id,
            name: row.name,
            audio_file_path: row.audio_file_path,
            scheduled_time: row.scheduled_time,
            enabled: row.enabled != 0,
            repeat_type: serde_json::from_str(&row.repeat_type).unwrap(),
            volume: row.volume as u8,
            created_at: DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap()
                .with_timezone(&Local),
            updated_at: DateTime::parse_from_rfc3339(&row.updated_at)
                .unwrap()
                .with_timezone(&Local),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct SettingRow {
    pub key: String,
    pub value: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct PlaybackHistoryRow {
    pub id: String,
    pub schedule_id: String,
    pub played_at: String,
    pub status: String,            // 'success', 'failed', 'skipped'
}
```

---

## API Contracts (Tauri IPC)

All commands follow the pattern:
```rust
#[tauri::command]
async fn command_name(param: Type, state: State<'_, AppState>) -> Result<ReturnType, String>
```

Frontend invokes via:
```typescript
import { invoke } from '@tauri-apps/api/tauri';
const result = await invoke<ReturnType>('command_name', { param: value });
```

### Schedule Commands

```rust
// src-tauri/src/commands/schedules.rs

/// Get all schedules
#[tauri::command]
async fn get_all_schedules(
    state: State<'_, AppState>
) -> Result<Vec<Schedule>, String>

/// Get a single schedule by ID
#[tauri::command]
async fn get_schedule_by_id(
    id: String,
    state: State<'_, AppState>
) -> Result<Schedule, String>

/// Create a new schedule
#[tauri::command]
async fn create_schedule(
    input: CreateScheduleInput,
    state: State<'_, AppState>
) -> Result<Schedule, String>

/// Update an existing schedule
#[tauri::command]
async fn update_schedule(
    id: String,
    input: UpdateScheduleInput,
    state: State<'_, AppState>
) -> Result<Schedule, String>

/// Delete a schedule
#[tauri::command]
async fn delete_schedule(
    id: String,
    state: State<'_, AppState>
) -> Result<(), String>

/// Toggle schedule enabled/disabled
#[tauri::command]
async fn toggle_schedule(
    id: String,
    state: State<'_, AppState>
) -> Result<Schedule, String>

/// Get upcoming executions (next N schedules)
#[tauri::command]
async fn get_upcoming_executions(
    count: usize,
    state: State<'_, AppState>
) -> Result<Vec<ScheduleExecution>, String>
```

### Settings Commands

```rust
// src-tauri/src/commands/settings.rs

/// Get all settings as key-value map
#[tauri::command]
async fn get_settings(
    state: State<'_, AppState>
) -> Result<HashMap<String, String>, String>

/// Get a single setting by key
#[tauri::command]
async fn get_setting(
    key: String,
    state: State<'_, AppState>
) -> Result<String, String>

/// Update multiple settings at once
#[tauri::command]
async fn update_settings(
    settings: HashMap<String, String>,
    state: State<'_, AppState>
) -> Result<(), String>

/// Update a single setting
#[tauri::command]
async fn update_setting(
    key: String,
    value: String,
    state: State<'_, AppState>
) -> Result<(), String>

/// Set launch at login (platform-specific)
#[tauri::command]
async fn set_launch_at_login(
    enabled: bool,
    state: State<'_, AppState>
) -> Result<(), String>

/// Check if launch at login is enabled
#[tauri::command]
async fn is_launch_at_login_enabled(
    state: State<'_, AppState>
) -> Result<bool, String>
```

### Audio Commands

```rust
// src-tauri/src/commands/audio.rs

/// Validate audio file (format, existence, readability)
#[tauri::command]
async fn validate_audio_file(
    file_path: String,
    state: State<'_, AppState>
) -> Result<AudioFileInfo, String>

/// Play audio file manually (for testing)
#[tauri::command]
async fn play_audio_file(
    file_path: String,
    volume: f32,              // 0.0-1.0
    state: State<'_, AppState>
) -> Result<(), String>

/// Stop current audio playback
#[tauri::command]
async fn stop_audio_playback(
    state: State<'_, AppState>
) -> Result<(), String>

/// Get current playback status
#[tauri::command]
async fn get_playback_status(
    state: State<'_, AppState>
) -> Result<PlaybackStatus, String>

/// Get list of available audio output devices
#[tauri::command]
async fn get_audio_devices(
    state: State<'_, AppState>
) -> Result<Vec<AudioDevice>, String>

#[derive(Debug, Serialize)]
pub struct AudioFileInfo {
    pub format: String,           // "mp3", "wav", etc.
    pub duration_secs: Option<u64>,
    pub file_size_bytes: u64,
    pub sample_rate: Option<u32>,
    pub channels: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}
```

### Scheduler Commands

```rust
// src-tauri/src/commands/scheduler.rs

/// Start the scheduler engine
#[tauri::command]
async fn start_scheduler(
    state: State<'_, AppState>
) -> Result<(), String>

/// Stop the scheduler engine
#[tauri::command]
async fn stop_scheduler(
    state: State<'_, AppState>
) -> Result<(), String>

/// Pause all schedules temporarily (don't execute until resumed)
#[tauri::command]
async fn pause_all_schedules(
    state: State<'_, AppState>
) -> Result<(), String>

/// Resume all schedules
#[tauri::command]
async fn resume_all_schedules(
    state: State<'_, AppState>
) -> Result<(), String>

/// Get scheduler status
#[tauri::command]
async fn get_scheduler_status(
    state: State<'_, AppState>
) -> Result<SchedulerStatus, String>

/// Force reload all schedules (after system wake, etc.)
#[tauri::command]
async fn reload_schedules(
    state: State<'_, AppState>
) -> Result<(), String>

#[derive(Debug, Serialize)]
pub struct SchedulerStatus {
    pub is_running: bool,
    pub is_paused: bool,
    pub active_schedules_count: usize,
    pub next_execution: Option<ScheduleExecution>,
}
```

### Frontend Invocation Examples

```typescript
// Get all schedules
const schedules = await invoke<Schedule[]>('get_all_schedules');

// Create schedule
const newSchedule = await invoke<Schedule>('create_schedule', {
  input: {
    name: 'Morning Meditation',
    audioFilePath: '/Users/sarah/meditation.mp3',
    scheduledTime: '07:00',
    repeatType: 'daily',
    volume: 80,
    enabled: true,
  }
});

// Update schedule
const updated = await invoke<Schedule>('update_schedule', {
  id: 'schedule-uuid',
  input: {
    volume: 70,
  }
});

// Delete schedule
await invoke<void>('delete_schedule', { id: 'schedule-uuid' });

// Pause all schedules
await invoke<void>('pause_all_schedules');

// Get playback status
const status = await invoke<PlaybackStatus>('get_playback_status');
```

---

## Data Flow Patterns

### Pattern 1: Creating a Schedule

```
User Action (Click "New Schedule")
         ↓
Frontend Component (ScheduleModal)
         ↓
Form Validation (react-hook-form)
         ↓
Zustand Action (scheduleStore.createSchedule)
         ↓
Tauri IPC Invoke ('create_schedule')
         ↓
Backend Command (commands/schedules.rs::create_schedule)
         ↓
Repository (db/schedules.rs::create)
         ↓
SQLite INSERT
         ↓
Scheduler Engine Reload (reload active schedules)
         ↓
Return Schedule to Frontend
         ↓
Zustand State Update (add to schedules array)
         ↓
UI Re-render (new schedule card appears)
         ↓
Success Toast Notification
```

### Pattern 2: Schedule Execution

```
Scheduler Engine (background tokio task)
         ↓
Time Check (is current time >= scheduled time?)
         ↓
Execute Schedule
         ↓
Audio Player (play audio file)
         ↓
Send Notification (tauri-plugin-notification)
         ↓
Log Execution (playback_history table)
         ↓
Update UI (if window is open)
         ↓
Calculate Next Execution Time
         ↓
Wait for next execution
```

### Pattern 3: Real-Time UI Updates

```
Backend Event (schedule executing, error, etc.)
         ↓
Tauri Event Emit (app.emit('schedule:event', payload))
         ↓
Frontend Event Listener (useEffect with listen())
         ↓
Zustand State Update
         ↓
UI Re-render (schedule card shows "playing" state)
```

**Event Types:**
- `schedule:executing` - Schedule started playing
- `schedule:completed` - Schedule finished playing
- `schedule:failed` - Schedule failed to execute
- `scheduler:paused` - All schedules paused
- `scheduler:resumed` - All schedules resumed

---

## Database Design

### Database File Location

- **macOS**: `~/Library/Application Support/com.yourname.resonator/resonator.db`
- **Windows**: `%APPDATA%/Resonator/resonator.db`
- **Linux**: `~/.local/share/resonator/resonator.db`

### Schema

```sql
-- migrations/001_initial.sql

CREATE TABLE schedules (
    id TEXT PRIMARY KEY,                    -- UUID v4
    name TEXT NOT NULL,                     -- Schedule name
    audio_file_path TEXT NOT NULL,         -- Absolute path
    scheduled_time TEXT NOT NULL,          -- HH:mm (24-hour)
    enabled INTEGER DEFAULT 1,             -- Boolean (0/1)
    repeat_type TEXT NOT NULL,             -- JSON serialized RepeatType
    volume INTEGER DEFAULT 80,             -- 0-100
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,  -- ISO 8601
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,  -- ISO 8601

    CHECK (length(name) <= 100),
    CHECK (volume >= 0 AND volume <= 100)
);

CREATE INDEX idx_schedules_enabled ON schedules(enabled);
CREATE INDEX idx_schedules_scheduled_time ON schedules(scheduled_time);

CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Default settings
INSERT INTO settings (key, value) VALUES
    ('theme', 'system'),
    ('launch_at_login', 'false'),
    ('start_minimized', 'false'),
    ('minimize_to_tray', 'true'),
    ('show_notifications', 'true'),
    ('notification_sound', 'true'),
    ('show_pre_notification', 'false'),
    ('notification_style', 'banner'),
    ('override_dnd', 'false'),
    ('default_volume', '80'),
    ('audio_output_device', 'system');

-- migrations/002_add_history.sql

CREATE TABLE playback_history (
    id TEXT PRIMARY KEY,                    -- UUID v4
    schedule_id TEXT NOT NULL,             -- Foreign key to schedules
    played_at TEXT DEFAULT CURRENT_TIMESTAMP,  -- ISO 8601
    status TEXT NOT NULL,                   -- 'success', 'failed', 'skipped'
    error_message TEXT,                     -- If failed

    FOREIGN KEY (schedule_id) REFERENCES schedules(id) ON DELETE CASCADE
);

CREATE INDEX idx_history_schedule_id ON playback_history(schedule_id);
CREATE INDEX idx_history_played_at ON playback_history(played_at);

-- Cleanup old history (keep last 1000 entries)
CREATE TRIGGER cleanup_history AFTER INSERT ON playback_history
BEGIN
    DELETE FROM playback_history
    WHERE id NOT IN (
        SELECT id FROM playback_history
        ORDER BY played_at DESC
        LIMIT 1000
    );
END;
```

### Repository Pattern

```rust
// src-tauri/src/db/schedules.rs

use sqlx::{SqlitePool, Result};
use uuid::Uuid;
use chrono::Local;

pub struct ScheduleRepository {
    pool: SqlitePool,
}

impl ScheduleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateScheduleInput) -> Result<Schedule> {
        let id = Uuid::new_v4().to_string();
        let now = Local::now().to_rfc3339();
        let repeat_type_json = serde_json::to_string(&input.repeat_type)?;

        sqlx::query!(
            r#"
            INSERT INTO schedules (id, name, audio_file_path, scheduled_time,
                                   enabled, repeat_type, volume, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            id,
            input.name,
            input.audio_file_path,
            input.scheduled_time,
            input.enabled as i64,
            repeat_type_json,
            input.volume as i64,
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        self.get_by_id(&id).await
    }

    pub async fn get_all(&self) -> Result<Vec<Schedule>> {
        let rows = sqlx::query_as!(ScheduleRow, "SELECT * FROM schedules ORDER BY scheduled_time")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Schedule::from).collect())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Schedule> {
        let row = sqlx::query_as!(ScheduleRow, "SELECT * FROM schedules WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Schedule::from(row))
    }

    pub async fn update(&self, id: &str, input: UpdateScheduleInput) -> Result<Schedule> {
        let now = Local::now().to_rfc3339();

        // Build dynamic UPDATE query based on provided fields
        // (Simplified for brevity - actual implementation would be more robust)

        if let Some(name) = input.name {
            sqlx::query!("UPDATE schedules SET name = ?, updated_at = ? WHERE id = ?",
                         name, now, id)
                .execute(&self.pool)
                .await?;
        }

        // ... (similar for other fields)

        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM schedules WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_enabled(&self) -> Result<Vec<Schedule>> {
        let rows = sqlx::query_as!(
            ScheduleRow,
            "SELECT * FROM schedules WHERE enabled = 1 ORDER BY scheduled_time"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Schedule::from).collect())
    }
}
```

---

## Scheduling Engine Architecture

### Engine Overview

The scheduler engine runs in the background using Tokio async tasks. Each enabled schedule gets its own task that calculates the next execution time and sleeps until then.

```
┌─────────────────────────────────────────────────────────┐
│              Scheduler Engine (Arc<Mutex<>>)            │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Task Manager:                                          │
│  ┌─────────────────────────────────────────────────┐   │
│  │  HashMap<ScheduleId, JoinHandle<()>>            │   │
│  │  (One tokio task per enabled schedule)          │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  For each schedule:                                     │
│  ┌─────────────────────────────────────────────────┐   │
│  │  tokio::spawn(async move {                      │   │
│  │      loop {                                      │   │
│  │          let next_time = calculate_next();      │   │
│  │          let delay = next_time - now();         │   │
│  │          tokio::time::sleep(delay).await;       │   │
│  │          execute_schedule().await;              │   │
│  │          if schedule.repeat_type == Once {      │   │
│  │              break;                              │   │
│  │          }                                       │   │
│  │      }                                           │   │
│  │  })                                              │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  State:                                                 │
│  - is_running: bool                                     │
│  - is_paused: bool                                      │
│  - active_tasks: HashMap                                │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Time Calculation Logic

```rust
// src-tauri/src/scheduler/time_calculator.rs

use chrono::{Local, DateTime, Datelike, Timelike, Duration, NaiveTime};

pub fn calculate_next_execution(
    schedule: &Schedule,
    from_time: DateTime<Local>,
) -> Option<DateTime<Local>> {
    let scheduled_time = parse_time(&schedule.scheduled_time)?;

    match &schedule.repeat_type {
        RepeatType::Once => {
            let today_at_time = from_time
                .date()
                .and_hms_opt(scheduled_time.hour(), scheduled_time.minute(), 0)?;

            if today_at_time > from_time {
                Some(today_at_time)
            } else {
                None // Already passed, won't execute
            }
        }

        RepeatType::Daily => {
            let today_at_time = from_time
                .date()
                .and_hms_opt(scheduled_time.hour(), scheduled_time.minute(), 0)?;

            if today_at_time > from_time {
                Some(today_at_time)
            } else {
                Some(today_at_time + Duration::days(1))
            }
        }

        RepeatType::Weekdays => {
            find_next_weekday(from_time, scheduled_time)
        }

        RepeatType::Weekends => {
            find_next_weekend(from_time, scheduled_time)
        }

        RepeatType::Custom { days } => {
            find_next_custom_day(from_time, scheduled_time, days)
        }
    }
}

fn parse_time(time_str: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(time_str, "%H:%M").ok()
}

fn find_next_weekday(
    from: DateTime<Local>,
    time: NaiveTime,
) -> Option<DateTime<Local>> {
    let mut candidate = from.date();

    for _ in 0..8 { // Check next 8 days (safety limit)
        let weekday = candidate.weekday();

        // Monday = 0, Friday = 4 (chrono uses Monday = 0)
        if weekday.num_days_from_monday() < 5 {
            let candidate_time = candidate.and_hms_opt(time.hour(), time.minute(), 0)?;
            if candidate_time > from {
                return Some(candidate_time);
            }
        }

        candidate = candidate.succ();
    }

    None
}

fn find_next_weekend(
    from: DateTime<Local>,
    time: NaiveTime,
) -> Option<DateTime<Local>> {
    let mut candidate = from.date();

    for _ in 0..8 {
        let weekday = candidate.weekday();

        // Saturday = 5, Sunday = 6 (num_days_from_monday)
        if weekday.num_days_from_monday() >= 5 {
            let candidate_time = candidate.and_hms_opt(time.hour(), time.minute(), 0)?;
            if candidate_time > from {
                return Some(candidate_time);
            }
        }

        candidate = candidate.succ();
    }

    None
}

fn find_next_custom_day(
    from: DateTime<Local>,
    time: NaiveTime,
    days: &[u8], // 0=Sun, 1=Mon, ..., 6=Sat
) -> Option<DateTime<Local>> {
    let mut candidate = from.date();

    for _ in 0..8 {
        let weekday = candidate.weekday();
        let day_num = (weekday.num_days_from_sunday()) as u8;

        if days.contains(&day_num) {
            let candidate_time = candidate.and_hms_opt(time.hour(), time.minute(), 0)?;
            if candidate_time > from {
                return Some(candidate_time);
            }
        }

        candidate = candidate.succ();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_schedule() {
        // Test that daily schedule returns correct next time
        // (Comprehensive tests would go here)
    }

    #[test]
    fn test_weekdays_schedule() {
        // Test weekdays logic
    }
}
```

### Task Management

```rust
// src-tauri/src/scheduler/engine.rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct SchedulerEngine {
    state: Arc<Mutex<EngineState>>,
    audio_player: Arc<AudioPlayer>,
    schedule_repo: Arc<ScheduleRepository>,
    app_handle: tauri::AppHandle,
}

struct EngineState {
    is_running: bool,
    is_paused: bool,
    active_tasks: HashMap<String, JoinHandle<()>>,
}

impl SchedulerEngine {
    pub fn new(
        audio_player: Arc<AudioPlayer>,
        schedule_repo: Arc<ScheduleRepository>,
        app_handle: tauri::AppHandle,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(EngineState {
                is_running: false,
                is_paused: false,
                active_tasks: HashMap::new(),
            })),
            audio_player,
            schedule_repo,
            app_handle,
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        let mut state = self.state.lock().await;

        if state.is_running {
            return Ok(()); // Already running
        }

        state.is_running = true;
        drop(state); // Release lock

        self.reload_schedules().await?;

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), String> {
        let mut state = self.state.lock().await;

        state.is_running = false;

        // Cancel all active tasks
        for (_, handle) in state.active_tasks.drain() {
            handle.abort();
        }

        Ok(())
    }

    pub async fn pause_all(&self) -> Result<(), String> {
        let mut state = self.state.lock().await;
        state.is_paused = true;
        Ok(())
    }

    pub async fn resume_all(&self) -> Result<(), String> {
        let mut state = self.state.lock().await;
        state.is_paused = false;
        Ok(())
    }

    pub async fn reload_schedules(&self) -> Result<(), String> {
        let mut state = self.state.lock().await;

        // Cancel existing tasks
        for (_, handle) in state.active_tasks.drain() {
            handle.abort();
        }

        // Load enabled schedules
        let schedules = self.schedule_repo.get_enabled().await
            .map_err(|e| e.to_string())?;

        // Spawn new tasks
        for schedule in schedules {
            let handle = self.spawn_schedule_task(schedule.clone());
            state.active_tasks.insert(schedule.id.clone(), handle);
        }

        Ok(())
    }

    fn spawn_schedule_task(&self, schedule: Schedule) -> JoinHandle<()> {
        let state = Arc::clone(&self.state);
        let audio_player = Arc::clone(&self.audio_player);
        let app_handle = self.app_handle.clone();

        tokio::spawn(async move {
            loop {
                // Check if paused
                let is_paused = {
                    let state = state.lock().await;
                    state.is_paused
                };

                if is_paused {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }

                // Calculate next execution time
                let now = Local::now();
                let next_time = match calculate_next_execution(&schedule, now) {
                    Some(time) => time,
                    None => {
                        // Schedule won't execute again (e.g., RepeatType::Once in past)
                        break;
                    }
                };

                // Sleep until execution time
                let delay = (next_time - now).to_std().unwrap_or_default();
                tokio::time::sleep(delay).await;

                // Execute schedule
                execute_schedule(&schedule, &audio_player, &app_handle).await;

                // If RepeatType::Once, exit loop
                if matches!(schedule.repeat_type, RepeatType::Once) {
                    break;
                }
            }
        })
    }
}

async fn execute_schedule(
    schedule: &Schedule,
    audio_player: &Arc<AudioPlayer>,
    app_handle: &tauri::AppHandle,
) {
    // Emit event to frontend
    let _ = app_handle.emit_all("schedule:executing", schedule.clone());

    // Send notification
    send_notification(app_handle, &format!("Playing: {}", schedule.name), "").await;

    // Play audio
    let volume = (schedule.volume as f32) / 100.0;
    match audio_player.play(&schedule.audio_file_path, volume).await {
        Ok(_) => {
            // Log success
            log_playback_history(schedule, "success", None).await;

            // Emit completion event
            let _ = app_handle.emit_all("schedule:completed", schedule.clone());
        }
        Err(e) => {
            // Log failure
            log_playback_history(schedule, "failed", Some(&e.to_string())).await;

            // Send error notification
            send_notification(
                app_handle,
                &format!("Failed to play: {}", schedule.name),
                &e.to_string(),
            ).await;

            // Emit failure event
            let _ = app_handle.emit_all("schedule:failed", schedule.clone());
        }
    }
}

async fn send_notification(app_handle: &tauri::AppHandle, title: &str, body: &str) {
    // Use tauri-plugin-notification
    use tauri_plugin_notification::NotificationExt;

    let _ = app_handle
        .notification()
        .builder()
        .title(title)
        .body(body)
        .show();
}

async fn log_playback_history(schedule: &Schedule, status: &str, error: Option<&str>) {
    // Implementation would insert into playback_history table
}
```

---

## Audio Playback Architecture

### Audio Player

```rust
// src-tauri/src/audio/player.rs

use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AudioPlayer {
    state: Arc<Mutex<PlayerState>>,
}

struct PlayerState {
    _stream: Option<OutputStream>,  // Keep alive
    sink: Option<Sink>,
    current_schedule_id: Option<String>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, AudioError> {
        Ok(Self {
            state: Arc::new(Mutex::new(PlayerState {
                _stream: None,
                sink: None,
                current_schedule_id: None,
            })),
        })
    }

    pub async fn play(&self, file_path: &str, volume: f32) -> Result<(), AudioError> {
        let mut state = self.state.lock().await;

        // Stop current playback if any
        if let Some(sink) = &state.sink {
            sink.stop();
        }

        // Create new stream and sink
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| AudioError::DeviceError(e.to_string()))?;

        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| AudioError::DeviceError(e.to_string()))?;

        // Load and decode audio file
        let file = File::open(file_path)
            .map_err(|e| AudioError::FileNotFound(file_path.to_string()))?;

        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| AudioError::UnsupportedFormat(e.to_string()))?;

        // Set volume and play
        sink.set_volume(volume.clamp(0.0, 1.0));
        sink.append(source);
        sink.play();

        // Update state
        state._stream = Some(stream);
        state.sink = Some(sink);

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), AudioError> {
        let mut state = self.state.lock().await;

        if let Some(sink) = &state.sink {
            sink.stop();
        }

        state.sink = None;
        state._stream = None;
        state.current_schedule_id = None;

        Ok(())
    }

    pub async fn is_playing(&self) -> bool {
        let state = self.state.lock().await;
        state.sink.as_ref().map(|s| !s.is_paused()).unwrap_or(false)
    }

    pub async fn get_status(&self) -> PlaybackStatus {
        let state = self.state.lock().await;

        PlaybackStatus {
            is_playing: state.sink.as_ref().map(|s| !s.is_paused()).unwrap_or(false),
            schedule_id: state.current_schedule_id.clone(),
            schedule_name: None, // Would need to look up
            audio_file_path: None,
            started_at: None,
        }
    }
}

// src-tauri/src/audio/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Audio file not found: {0}")]
    FileNotFound(String),

    #[error("Unsupported audio format: {0}")]
    UnsupportedFormat(String),

    #[error("Audio device error: {0}")]
    DeviceError(String),

    #[error("Playback error: {0}")]
    PlaybackError(String),
}
```

### Audio Validation

```rust
// src-tauri/src/audio/validator.rs

use std::fs;
use std::path::Path;

const SUPPORTED_FORMATS: &[&str] = &["mp3", "wav", "flac", "m4a", "ogg", "aac"];

pub fn validate_audio_file(file_path: &str) -> Result<AudioFileInfo, AudioError> {
    let path = Path::new(file_path);

    // Check file exists
    if !path.exists() {
        return Err(AudioError::FileNotFound(file_path.to_string()));
    }

    // Check file is readable
    if !path.is_file() {
        return Err(AudioError::FileNotFound(format!("{} is not a file", file_path)));
    }

    // Check extension
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .ok_or_else(|| AudioError::UnsupportedFormat("No file extension".to_string()))?;

    if !SUPPORTED_FORMATS.contains(&extension.as_str()) {
        return Err(AudioError::UnsupportedFormat(format!(
            "Format '{}' not supported. Supported formats: {}",
            extension,
            SUPPORTED_FORMATS.join(", ")
        )));
    }

    // Get file metadata
    let metadata = fs::metadata(path)
        .map_err(|e| AudioError::FileNotFound(format!("Cannot read file: {}", e)))?;

    Ok(AudioFileInfo {
        format: extension,
        duration_secs: None, // Could use a library to parse audio metadata
        file_size_bytes: metadata.len(),
        sample_rate: None,
        channels: None,
    })
}
```

---

## Frontend Architecture

### State Management (Zustand)

```typescript
// src/stores/scheduleStore.ts

import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

interface ScheduleStore {
  schedules: Schedule[];
  isLoading: boolean;
  error: string | null;

  // Actions
  fetchSchedules: () => Promise<void>;
  createSchedule: (input: CreateScheduleInput) => Promise<Schedule>;
  updateSchedule: (id: string, input: UpdateScheduleInput) => Promise<Schedule>;
  deleteSchedule: (id: string) => Promise<void>;
  toggleSchedule: (id: string) => Promise<Schedule>;

  // Real-time updates
  handleScheduleEvent: (event: ScheduleEvent) => void;
}

export const useScheduleStore = create<ScheduleStore>((set, get) => ({
  schedules: [],
  isLoading: false,
  error: null,

  fetchSchedules: async () => {
    set({ isLoading: true, error: null });
    try {
      const schedules = await invoke<Schedule[]>('get_all_schedules');
      set({ schedules, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  createSchedule: async (input) => {
    set({ isLoading: true, error: null });
    try {
      const schedule = await invoke<Schedule>('create_schedule', { input });
      set(state => ({
        schedules: [...state.schedules, schedule],
        isLoading: false,
      }));
      return schedule;
    } catch (error) {
      set({ error: String(error), isLoading: false });
      throw error;
    }
  },

  updateSchedule: async (id, input) => {
    set({ isLoading: true, error: null });
    try {
      const schedule = await invoke<Schedule>('update_schedule', { id, input });
      set(state => ({
        schedules: state.schedules.map(s => s.id === id ? schedule : s),
        isLoading: false,
      }));
      return schedule;
    } catch (error) {
      set({ error: String(error), isLoading: false });
      throw error;
    }
  },

  deleteSchedule: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await invoke<void>('delete_schedule', { id });
      set(state => ({
        schedules: state.schedules.filter(s => s.id !== id),
        isLoading: false,
      }));
    } catch (error) {
      set({ error: String(error), isLoading: false });
      throw error;
    }
  },

  toggleSchedule: async (id) => {
    try {
      const schedule = await invoke<Schedule>('toggle_schedule', { id });
      set(state => ({
        schedules: state.schedules.map(s => s.id === id ? schedule : s),
      }));
      return schedule;
    } catch (error) {
      set({ error: String(error) });
      throw error;
    }
  },

  handleScheduleEvent: (event) => {
    // Handle real-time updates from backend
    const { schedules } = get();
    // Update schedule status based on event type
  },
}));

// Set up event listeners on app start
export function initScheduleEvents() {
  listen('schedule:executing', (event) => {
    useScheduleStore.getState().handleScheduleEvent(event.payload as ScheduleEvent);
  });

  listen('schedule:completed', (event) => {
    useScheduleStore.getState().handleScheduleEvent(event.payload as ScheduleEvent);
  });

  listen('schedule:failed', (event) => {
    useScheduleStore.getState().handleScheduleEvent(event.payload as ScheduleEvent);
  });
}
```

---

## Error Handling Strategy

### Backend Error Types

```rust
// src-tauri/src/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Audio error: {0}")]
    Audio(#[from] AudioError),

    #[error("Schedule not found: {0}")]
    ScheduleNotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

// Convert AppError to String for Tauri commands
impl From<AppError> for String {
    fn from(error: AppError) -> String {
        error.to_string()
    }
}
```

### Frontend Error Handling

```typescript
// src/utils/errorHandling.ts

export function handleApiError(error: unknown): string {
  if (typeof error === 'string') {
    return error;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return 'An unknown error occurred';
}

// Usage in components
try {
  await createSchedule(input);
  toast.success('Schedule created successfully');
} catch (error) {
  const message = handleApiError(error);
  toast.error(message);
  console.error('Failed to create schedule:', error);
}
```

---

## Security Considerations

1. **File Access**:
   - Only access user-selected files via native file dialog
   - Validate file paths (no path traversal)
   - Check file permissions before reading

2. **SQL Injection**:
   - Use parameterized queries (sqlx prevents this)
   - Never concatenate user input into SQL

3. **Input Validation**:
   - Validate all inputs on backend (don't trust frontend)
   - Sanitize file paths
   - Limit string lengths (schedule name: 100 chars)
   - Validate volume range (0-100)
   - Validate time format (HH:mm)

4. **Tauri Security**:
   - Configure CSP (Content Security Policy)
   - Use allowlist for Tauri APIs
   - Enable signature verification for updates

5. **Data Privacy**:
   - No telemetry without user consent
   - All data stored locally
   - No network requests (except updates)

---

## Performance Considerations

**Startup Time**: Target <1 second
- Lazy load schedules
- Use SQLite connection pool
- Minimize initial render

**Memory Usage**: Target <50MB idle
- Limit in-memory schedules (paginate if needed)
- Stream large audio files (don't load entirely into memory)
- Clean up old playback history

**CPU Usage**: Target <5% idle, <20% playing
- Use async/await (tokio) for non-blocking operations
- Efficient time calculations (no busy loops)
- Audio playback handled by rodio (optimized)

**Database Performance**:
- Index on `enabled` and `scheduled_time`
- Limit playback history (keep last 1000)
- Use VACUUM periodically

---

## Architecture Diagrams

### System Context Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                          User                               │
│                            ↓                                │
│  ┌───────────────────────────────────────────────────────┐ │
│  │                                                       │ │
│  │             Resonator Application              │ │
│  │                                                       │ │
│  │  ┌──────────────┐            ┌──────────────────┐   │ │
│  │  │   Frontend   │ ←→ IPC ←→  │   Backend        │   │ │
│  │  │   (React)    │            │   (Rust/Tauri)   │   │ │
│  │  └──────────────┘            └──────────────────┘   │ │
│  │                                       ↓              │ │
│  └───────────────────────────────────────┼──────────────┘ │
│                                          ↓                 │
│  ┌───────────────┐  ┌───────────────┐  ┌──────────────┐  │
│  │  File System  │  │ Audio Devices │  │   System     │  │
│  │ (Audio Files) │  │   (Speakers)  │  │ (Tray/Notif) │  │
│  └───────────────┘  └───────────────┘  └──────────────┘  │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

### Component Interaction Diagram

```
Frontend (React)                Backend (Rust)                   System
─────────────────              ──────────────────              ───────────

User Action
    ↓
[Component]
    ↓
[Store Action]
    ↓
invoke('command')  ──────────→  [Tauri Command]
                                       ↓
                                [Business Logic]
                                       ↓
                                [Repository]
                                       ↓
                                [SQLite Query] ────────→  [Database]
                                       ↓                      ↓
                                [Result]        ←────────  [Data]
                                       ↓
Response  ←──────────────────  [JSON Result]
    ↓
[Store Update]
    ↓
[UI Re-render]
```

### Scheduler Engine Flow

```
[Scheduler Engine Start]
         ↓
[Load Enabled Schedules from DB]
         ↓
For each schedule:
    ↓
[Spawn Tokio Task]
    ↓
    ┌────────────────────────────┐
    │    Schedule Task Loop      │
    │                            │
    │  1. Check if paused        │
    │  2. Calculate next time    │
    │  3. Sleep until time       │
    │  4. Execute schedule       │
    │     ↓                      │
    │  [Play Audio]              │
    │  [Send Notification]       │
    │  [Log History]             │
    │  [Emit Event]              │
    │     ↓                      │
    │  5. If Once: break         │
    │  6. Else: loop             │
    │                            │
    └────────────────────────────┘
```

---

## Summary & Next Steps

### Architecture Highlights

✅ **Separation of Concerns**: Clear layers (UI, State, IPC, Business Logic, Data)
✅ **Testability**: Repository pattern, pure functions, isolated components
✅ **Performance**: Async/await, efficient scheduling, minimal memory
✅ **Scalability**: Can handle 1000+ schedules without degradation
✅ **Maintainability**: TypeScript + Rust type safety, clear patterns
✅ **Cross-Platform Ready**: Tauri provides abstraction for OS differences

### Key Technologies Justified

- **Tauri**: Smaller, faster, more secure than Electron
- **Rust**: Memory safety, performance, async capabilities
- **SQLite**: Embedded, zero-config, reliable
- **Tokio**: Best-in-class async runtime for Rust
- **Rodio**: Cross-platform audio, well-maintained
- **Zustand**: Lightweight state management, minimal boilerplate
- **shadcn/ui**: Accessible, customizable, modern components

### Ready for Implementation

This architecture provides:
- Clear data models for both frontend and backend
- Complete API contract (all Tauri commands defined)
- Database schema with migrations
- Scheduling engine design with time calculation logic
- Audio playback system architecture
- Error handling strategy
- Security and performance considerations

**Phase 1 is now complete!** Ready to proceed to Phase 2: Project Setup.

---

**Document Status**: ✅ Complete
**Next Phase**: 2.1 Development Environment Setup
**Owner**: Development Team

---

**Last Updated**: 2025-11-13
**Version**: 1.0
