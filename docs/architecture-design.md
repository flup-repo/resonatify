# Architecture Design: Resonatify

**Version**: 1.1
**Date**: 2025-11-15
**Status**: MVP Implementation

---

## Overview

Resonatify is a Tauri 2.0 desktop application for scheduling audio playback. Built with React 19 (TypeScript) frontend and Rust backend, using SQLite for persistence and tokio for async scheduling.

### Tech Stack
- **Frontend**: React 19 + TypeScript 5.8, Vite 7, Tailwind CSS 4, Zustand 5, shadcn/ui
- **Backend**: Rust + Tauri 2.0, tokio (async), SQLite (sqlx), rodio (audio)
- **IPC**: Tauri commands with serde JSON serialization

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     React Frontend                          │
│                                                             │
│  Components → Zustand Stores → invoke(command)             │
│     ↓              ↓                  ↓                     │
│  UI State    Local State      IPC Messages                 │
└─────────────────────┬───────────────────────────────────────┘
                      │ Tauri IPC (JSON)
┌─────────────────────┴───────────────────────────────────────┐
│                    Rust Backend                             │
│                                                             │
│  Commands → Business Logic → Repositories → SQLite         │
│     ↓              ↓              ↓                         │
│  IPC Entry   SchedulerEngine  Database                     │
│              AudioService                                   │
│                                                             │
│  ┌──────────────────────────────────────────────┐          │
│  │ SchedulerEngine (tokio)                      │          │
│  │  - Spawns task per enabled schedule          │          │
│  │  - Calculates next execution time            │          │
│  │  - Triggers audio playback                   │          │
│  │  - Logs to playback_history                  │          │
│  └──────────────────────────────────────────────┘          │
└─────────────────────────────────────────────────────────────┘
```

---

## Component Structure

### Frontend (`src/`)

```
components/
├── ScheduleCard.tsx         # Display schedule with actions
├── ScheduleList.tsx         # List + empty state
├── ScheduleModal.tsx        # Add/edit dialog (react-hook-form)
├── ScheduleHeader.tsx       # Header with new schedule button
├── settings/                # Settings UI
│   ├── SettingsLayout.tsx   # Sidebar layout
│   ├── SettingsPanel.tsx    # Settings sections
│   └── ThemeIndicator.tsx   # Theme preview
└── ui/                      # shadcn/ui primitives

stores/
├── scheduleStore.ts         # Schedule CRUD + state
└── settingsStore.ts         # Settings with localStorage + backend sync

types/
├── schedule.ts              # Schedule, CreateScheduleInput, UpdateScheduleInput
├── repeat.ts                # RepeatType variants + converters
└── settings.ts              # Settings interface

hooks/
├── useScheduleShortcuts.ts  # Keyboard shortcuts (Cmd+N, etc.)
└── useThemeSync.ts          # Sync theme to HTML data-theme
```

### Backend (`src-tauri/src/`)

```
lib.rs                       # App setup, state, command registration
main.rs                      # Entry point

commands/                    # Tauri IPC handlers
├── schedules.rs             # Schedule CRUD
├── scheduler.rs             # Scheduler control
├── audio.rs                 # Audio validation + playback
├── settings.rs              # Settings CRUD
└── dialogs.rs               # File picker

scheduler/
├── engine.rs                # SchedulerEngine (tokio task manager)
├── time_calculator.rs       # Next execution time calculation
└── error.rs                 # SchedulerError

audio/
├── service.rs               # AudioService (singleton)
├── player.rs                # AudioPlayer (rodio wrapper)
├── validator.rs             # File format validation
└── error.rs                 # AudioError

db/
├── mod.rs                   # Database initialization
├── models.rs                # Schedule, RepeatType, ScheduleRow, etc.
├── schedules.rs             # ScheduleRepository
├── settings.rs              # SettingsRepository
└── playback_history.rs      # PlaybackHistoryRepository

tray.rs                      # System tray + tooltip updater
```

---

## Data Models

### TypeScript (Frontend)

```typescript
interface Schedule {
  id: string;
  name: string;
  audioFilePath: string;
  scheduledTime: string;        // HH:mm
  enabled: boolean;
  repeatType: RepeatType;
  volume: number;               // 0-100
  createdAt: string;
  updatedAt: string;
}

type RepeatType =
  | { type: 'once' }
  | { type: 'daily' }
  | { type: 'weekdays' }
  | { type: 'weekends' }
  | { type: 'weekly'; days: number[] }       // 0=Sun, 6=Sat
  | { type: 'custom'; intervalMinutes: number };
```

### Rust (Backend)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: String,
    pub name: String,
    pub audio_file_path: String,
    pub scheduled_time: String,       // HH:mm
    pub enabled: bool,
    pub repeat_type: RepeatType,
    pub volume: u8,
    pub created_at: String,
    pub updated_at: String,
    pub last_run_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RepeatType {
    Once,
    Daily,
    Weekdays,
    Weekends,
    Weekly { days: Vec<Weekday> },   // chrono::Weekday
    Custom { interval_minutes: u32 },
}
```

---

## Database Schema

**Location**: `~/Library/Application Support/com.yourname.resonatify/resonatify.db`

```sql
CREATE TABLE schedules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    audio_file_path TEXT NOT NULL,
    scheduled_time TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    repeat_type TEXT NOT NULL,        -- JSON serialized RepeatType
    repeat_days TEXT,                 -- Optional JSON array
    volume INTEGER NOT NULL DEFAULT 100,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_run_at TEXT
);

CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE audio_playback_history (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    played_at TEXT NOT NULL,
    status TEXT NOT NULL,             -- 'success', 'failed', 'skipped'
    error_message TEXT,
    FOREIGN KEY (schedule_id) REFERENCES schedules(id) ON DELETE CASCADE
);

-- Auto-trim history to 1000 entries
CREATE TRIGGER trg_history_trim AFTER INSERT ON audio_playback_history ...
```

**Indexes**:
- `idx_schedules_enabled` on `schedules(enabled)`
- `idx_schedules_scheduled_time` on `schedules(scheduled_time)`
- `idx_history_schedule_id` on `audio_playback_history(schedule_id)`
- `idx_history_played_at` on `audio_playback_history(played_at DESC)`

---

## Data Flow

### Schedule Creation
```
User clicks "New Schedule"
  → ScheduleModal renders (react-hook-form)
  → User fills form + selects audio file
  → Form validation
  → scheduleStore.createSchedule(input)
  → invoke('create_schedule', { input })
  → Rust: commands::schedules::create_schedule()
  → ScheduleRepository.create()
  → SQLite INSERT
  → SchedulerEngine.reload_schedules()
  → Return Schedule
  → Update Zustand state
  → UI re-renders with new schedule
```

### Schedule Execution
```
SchedulerEngine spawns tokio task per enabled schedule
  → Task loop:
    1. Calculate next execution time (time_calculator.rs)
    2. tokio::time::sleep(delay).await
    3. Check if schedule still enabled
    4. AudioService.play(audio_file_path, volume)
    5. PlaybackHistoryRepository.insert(schedule_id, status)
    6. If RepeatType::Once: disable schedule + exit task
    7. Else: loop back to step 1
```

---

## Key Architectural Patterns

### Repository Pattern
All database access through repositories:
- `ScheduleRepository`: CRUD for schedules
- `SettingsRepository`: Key-value settings storage
- `PlaybackHistoryRepository`: Execution logs

Benefits: Testability, separation of concerns, consistent error handling

### Async/Await with Tokio
- Scheduler engine: One task per enabled schedule
- Non-blocking audio playback
- Background tray tooltip updates

### State Management
- **Zustand**: UI state + optimistic updates
- **SQLite**: Source of truth for persistence
- **localStorage**: Settings cache (synced with backend)

### Error Handling
- Rust: `thiserror` for typed errors
- IPC boundary: Convert errors to `String` via `.map_err(|e| e.to_string())`
- Frontend: Try-catch with user-friendly messages

---

## Scheduler Engine Design

### Implementation
**File**: `src-tauri/src/scheduler/engine.rs`

```rust
pub struct SchedulerEngine {
    database: Database,
    audio_service: AudioService,
    tasks: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

impl SchedulerEngine {
    pub async fn start(&self) -> Result<()> {
        // Load all enabled schedules
        let schedules = self.database.schedules().get_enabled().await?;

        // Spawn task for each schedule
        for schedule in schedules {
            self.spawn_task(schedule);
        }
    }

    fn spawn_task(&self, schedule: Schedule) {
        let handle = tokio::spawn(async move {
            loop {
                let next_time = calculate_next_execution(&schedule, now());
                tokio::time::sleep_until(next_time).await;

                // Execute schedule
                self.audio_service.play(&schedule.audio_file_path, volume).await;

                // Log execution
                self.database.playback_history().insert(...).await;

                // Exit if RepeatType::Once
                if matches!(schedule.repeat_type, RepeatType::Once) {
                    break;
                }
            }
        });

        self.tasks.lock().await.insert(schedule.id, handle);
    }
}
```

### Time Calculator
**File**: `src-tauri/src/scheduler/time_calculator.rs`

Logic for calculating next execution time based on `RepeatType`:
- **Once**: Today at time, or None if past
- **Daily**: Today at time, or tomorrow if past
- **Weekdays**: Next Mon-Fri at time
- **Weekends**: Next Sat-Sun at time
- **Weekly**: Next occurrence of selected weekdays at time
- **Custom**: Every N minutes from now

---

## Audio Service Design

### Implementation
**File**: `src-tauri/src/audio/service.rs`

```rust
pub struct AudioService {
    player: Arc<Mutex<AudioPlayer>>,
}

impl AudioService {
    pub async fn play(&self, path: &str, volume: f32) -> Result<()> {
        let mut player = self.player.lock().await;

        // Stop current playback
        player.stop()?;

        // Create output stream + sink
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        // Load and decode audio file
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;

        // Set volume and play
        sink.set_volume(volume);
        sink.append(source);
        sink.play();

        Ok(())
    }
}
```

**Supported Formats**: MP3, WAV, FLAC, OGG, M4A/AAC (via rodio)

---

## System Tray Integration

**File**: `src-tauri/src/tray.rs`

- Menu items: Show/Hide, Quit
- Tooltip: Shows next upcoming schedule
- Background task updates tooltip every 30 seconds
- Click: Toggle main window visibility

---

## Theme System

**Implementation**:
1. Settings store persists theme preference (`light`, `dark`, `system`)
2. `useThemeSync` hook:
   - Reads theme from settings
   - If `system`, listens to `prefers-color-scheme` media query
   - Sets `<html data-theme="light|dark">`
3. CSS variables defined for each theme in `src/index.css`
4. Tailwind v4 references CSS variables for colors

---

## Security Considerations

1. **File Access**: File picker dialog ensures scoped access
2. **SQL Injection**: Prevented by sqlx parameterized queries
3. **Input Validation**: All inputs validated on backend
   - Schedule name max 100 chars
   - Volume 0-100
   - Time format HH:mm
4. **Tauri Security**: CSP configured, API allowlist enabled
5. **No network access** (except future auto-updates)

---

## Performance Characteristics

- **Startup time**: ~500ms (target: <1s)
- **Memory (idle)**: ~40MB (target: <50MB)
- **CPU (idle)**: <2% (target: <5%)
- **CPU (playing)**: ~8% (target: <20%)
- **Schedule capacity**: Tested with 100+ schedules
- **Database size**: ~100KB for 1000 schedules + history

---

## Testing Strategy

### Unit Tests
- **Rust**: Time calculator logic, repeat type logic (`#[cfg(test)]`)
- **TypeScript**: Utility functions (Vitest)

### Integration Tests
- Database repositories
- Schedule creation → execution flow
- Audio playback

### Manual Testing
- System tray behavior
- Theme switching
- Schedule execution accuracy
- Audio format compatibility

---

## Future Improvements

- [ ] Notifications (tauri-plugin-notification)
- [ ] Launch at login (tauri-plugin-autostart)
- [ ] Auto-updates (tauri-plugin-updater)
- [ ] Windows & Linux support
- [ ] Schedule groups/categories
- [ ] Audio fade in/out
- [ ] Multiple audio actions per schedule

---

**Document Maintainers**: Update this doc when making architectural changes
**Last Updated**: 2025-11-15
**Version**: 1.1
