# Resonatify - AI Assistant Reference

> **Quick Context File for AI Assistants**
> This file provides essential information about the Resonatify project to help AI assistants understand the codebase structure, conventions, and technical decisions.

---

## Project Overview

**Resonatify** is a cross-platform desktop application for scheduling audio files to play at specific times. Built with Tauri 2.0, React, and Rust.

- **Status**: MVP in active development
- **Platform**: macOS (with cross-platform foundation)
- **Target**: Users needing audio-based reminders and alarms

### Implemented Features
- ✅ Schedule CRUD operations (create, read, update, delete)
- ✅ Multiple repeat patterns (Once, Daily, Weekdays, Weekends, Weekly with custom days)
- ✅ Background scheduler engine with tokio
- ✅ Audio playback service (rodio)
- ✅ SQLite database with migrations
- ✅ System tray integration
- ✅ Theme system (light/dark/system)
- ✅ Settings persistence
- ✅ File dialog for audio selection

---

## Technology Stack

### Frontend
- **Framework**: React 19 + TypeScript 5.8
- **Build**: Vite 7 + Tailwind CSS 4
- **UI**: shadcn/ui (Radix primitives)
- **State**: Zustand 5
- **Forms**: react-hook-form
- **Testing**: Vitest + React Testing Library

### Backend
- **Framework**: Tauri 2.0 (Rust)
- **Async**: tokio
- **Database**: SQLite (sqlx)
- **Audio**: rodio
- **Date/Time**: chrono
- **Plugins**: tauri-plugin-opener, tauri-plugin-dialog

---

## Project Structure

```
src/                              # React frontend
├── components/
│   ├── ScheduleCard.tsx         # Schedule display card
│   ├── ScheduleList.tsx         # Schedule list with empty state
│   ├── ScheduleModal.tsx        # Add/edit schedule dialog
│   ├── ScheduleHeader.tsx       # App header with actions
│   ├── settings/                # Settings UI
│   │   ├── SettingsLayout.tsx   # Settings panel with sidebar
│   │   ├── SettingsPanel.tsx    # Settings sections
│   │   └── ThemeIndicator.tsx   # Visual theme indicator
│   └── ui/                      # shadcn/ui components
├── stores/
│   ├── scheduleStore.ts         # Schedule state + Tauri commands
│   └── settingsStore.ts         # Settings state with persistence
├── types/
│   ├── schedule.ts              # Schedule types + mappers
│   ├── repeat.ts                # Repeat type definitions
│   └── settings.ts              # Settings types
├── hooks/
│   ├── useScheduleShortcuts.ts  # Keyboard shortcuts (Cmd+N, etc)
│   └── useThemeSync.ts          # Theme system sync
├── utils/
│   ├── cn.ts                    # Tailwind class merge
│   └── openLink.ts              # External link handler
└── App.tsx                      # Main app with routing

src-tauri/src/                   # Rust backend
├── lib.rs                       # App setup + state management
├── main.rs                      # Entry point
├── commands/                    # Tauri IPC commands
│   ├── audio.rs                 # Audio validation + playback
│   ├── dialogs.rs               # File picker dialog
│   ├── scheduler.rs             # Scheduler control
│   ├── schedules.rs             # Schedule CRUD
│   └── settings.rs              # Settings CRUD
├── audio/
│   ├── service.rs               # AudioService (rodio wrapper)
│   ├── player.rs                # Audio player state
│   ├── validator.rs             # File format validation
│   └── error.rs                 # Audio errors
├── scheduler/
│   ├── engine.rs                # SchedulerEngine (tokio tasks)
│   ├── time_calculator.rs       # Next execution time logic
│   └── error.rs                 # Scheduler errors
├── db/
│   ├── mod.rs                   # Database initialization
│   ├── models.rs                # SQLite models + conversions
│   ├── schedules.rs             # Schedule repository
│   ├── settings.rs              # Settings repository
│   └── playback_history.rs      # Playback log repository
├── tray.rs                      # System tray setup
└── migrations/
    ├── 001_create_core_tables.sql
    └── 002_add_last_run_at.sql
```

---

## Architecture Patterns

### Data Flow
```
User Action → Component → Zustand Store → invoke(command) → Tauri Command
  → Business Logic → Repository → SQLite → Response → Store Update → UI
```

### Key Patterns
- **Repository Pattern**: Database access via `ScheduleRepository`, `SettingsRepository`, `PlaybackHistoryRepository`
- **State Management**: Zustand stores for UI state + SQLite for persistence
- **Async Runtime**: Tokio for scheduler engine + background tasks
- **IPC**: Type-safe Tauri commands with serde serialization
- **Error Handling**: thiserror enums converted to String for IPC boundary

---

## Database Schema

**Location**: `~/Library/Application Support/com.yourname.resonatify/resonatify.db`

```sql
-- schedules: user-defined audio schedules
CREATE TABLE schedules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    audio_file_path TEXT NOT NULL,
    scheduled_time TEXT NOT NULL,     -- HH:mm format
    enabled INTEGER DEFAULT 1,        -- 0 or 1
    repeat_type TEXT NOT NULL,        -- JSON: RepeatType enum
    repeat_days TEXT,                 -- JSON: optional weekday array
    volume INTEGER DEFAULT 100,       -- 0-100
    created_at TEXT,
    updated_at TEXT,
    last_run_at TEXT                  -- Track last execution
);

-- settings: key-value configuration
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT
);

-- audio_playback_history: execution logs (auto-trimmed to 1000 entries)
CREATE TABLE audio_playback_history (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    played_at TEXT DEFAULT CURRENT_TIMESTAMP,
    status TEXT CHECK (status IN ('success', 'failed', 'skipped')),
    error_message TEXT,
    FOREIGN KEY (schedule_id) REFERENCES schedules(id) ON DELETE CASCADE
);
```

---

## Key Types

### TypeScript (Frontend)
```typescript
interface Schedule {
    id: string;
    name: string;
    audioFilePath: string;
    scheduledTime: string;        // HH:mm format
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
    | { type: 'weekly'; days: number[] }  // 0=Sun, 6=Sat
    | { type: 'custom'; intervalMinutes: number };

interface Settings {
    theme: 'light' | 'dark' | 'system';
    launchAtLogin: boolean;
    minimizeToTray: boolean;
    showNotifications: boolean;
    notificationSound: boolean;
    defaultVolume: number;
}
```

---

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

## Conventions

### Code Style
- **Naming**: PascalCase (components), camelCase (TS), snake_case (Rust)
- **Format**: `cargo fmt`, ESLint + Prettier
- **Error Handling**: thiserror → String for Tauri commands
- **Testing**: Unit tests for new features (Vitest for TS, `#[cfg(test)]` for Rust)

### Git Workflow
- **Commits**: Use conventional commits (`feat:`, `fix:`, `docs:`, etc.)
- **Branches**: `feature/*`, `bugfix/*`, `docs/*` → PR to `main`
- **Never commit directly to `main`** — all changes via PR

---

## Development Commands

```bash
# Development
cargo tauri dev              # Run app in dev mode
npm run test                 # Run frontend tests
cd src-tauri && cargo test   # Run backend tests

# Code quality
cargo fmt && cargo clippy    # Rust formatting + linting
npm run lint                 # TypeScript linting

# Build
cargo tauri build            # Production build
```

---

## Tauri IPC Commands

All commands registered in `src-tauri/src/lib.rs` and invoked via `invoke()`:

### Schedules
- `get_all_schedules()` → `Vec<Schedule>`
- `create_schedule(input)` → `Schedule`
- `update_schedule(id, input)` → `Schedule`
- `delete_schedule(id)` → `()`
- `toggle_schedule_enabled(id)` → `Schedule`

### Scheduler
- `start_scheduler()` → `()`
- `stop_scheduler()` → `()`
- `reload_scheduler()` → `()`
- `get_scheduler_status()` → `SchedulerStatus`
- `get_upcoming_executions(count)` → `Vec<ScheduleExecution>`

### Audio
- `validate_audio_file(path)` → `AudioFileInfo`
- `play_audio_file(path, volume)` → `()`
- `stop_audio()` → `()`
- `get_audio_status()` → `AudioStatus`

### Settings
- `get_settings()` → `SettingsSnapshot`
- `update_settings(settings)` → `()`
- `set_launch_at_login(enabled)` → `()`

### Dialogs
- `open_audio_file_dialog()` → `Option<String>`  # File path

---

## State Management

### Zustand Stores
**scheduleStore.ts**: Schedule list + CRUD operations via Tauri commands
**settingsStore.ts**: Settings with localStorage persistence + backend sync

Both stores use `invoke()` to communicate with Rust backend and update local state optimistically.

---

## Key Implementation Details

### Scheduler Engine
- **Location**: `src-tauri/src/scheduler/engine.rs`
- Each enabled schedule spawns a tokio task
- Time calculator determines next execution based on `RepeatType`
- Supports: Once, Daily, Weekdays, Weekends, Weekly (custom days), Custom (intervals)

### Audio Service
- **Location**: `src-tauri/src/audio/`
- Uses rodio for cross-platform audio playback
- Supported formats: MP3, WAV, FLAC, OGG, M4A/AAC
- Volume: 0-100 (UI) → 0.0-1.0 (rodio)

### System Tray
- **Location**: `src-tauri/src/tray.rs`
- Shows next upcoming schedule in tooltip
- Click to show/hide main window
- Updates tooltip every 30 seconds

### Theme System
- Three modes: light, dark, system
- `useThemeSync` hook applies theme to `<html data-theme>`
- Tailwind v4 CSS variables for theming
- Settings persist to both localStorage and SQLite

---

## Common Issues

### Dark/Light theme not applying
- **Fix**: Ensure `html[data-theme]` selectors exist, `@theme` directive positioned correctly
- **Reference**: src/index.css

### Schedule not executing
- Check: Schedule enabled, audio file exists, system not sleeping
- **Debug**: Check `audio_playback_history` table for errors

---

**Last Updated**: 2025-11-15
**Version**: 0.1.0
**Status**: MVP Development
