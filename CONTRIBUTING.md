# Contributing to Resonatify

Thank you for your interest in contributing to Resonatify! This guide will help you understand our codebase structure, conventions, and development workflow.

---

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Conventions](#code-conventions)
- [Git Workflow](#git-workflow)
- [Testing](#testing)
- [Architecture Overview](#architecture-overview)
- [Project Structure](#project-structure)
- [Common Development Tasks](#common-development-tasks)

---

## Getting Started

### Prerequisites

- **Node.js**: v18 or higher
- **Rust**: Latest stable (install via [rustup](https://rustup.rs))
- **Xcode Command Line Tools** (macOS): `xcode-select --install`

### Development Setup

1. Fork and clone the repository
2. Install dependencies: `npm install`
3. Run in development mode: `cargo tauri dev`

---

## Code Conventions

### Naming Conventions

- **TypeScript/React**:
  - Components: `PascalCase` (e.g., `ScheduleCard.tsx`)
  - Functions/variables: `camelCase` (e.g., `scheduleStore.ts`)
  - Constants: `UPPER_SNAKE_CASE`

- **Rust**:
  - Files/modules: `snake_case` (e.g., `schedule_repository.rs`)
  - Structs/Enums: `PascalCase` (e.g., `ScheduleRepository`)
  - Functions/variables: `snake_case`

### Code Style

- **TypeScript**: ESLint + Prettier
  ```bash
  npm run lint
  ```

- **Rust**: cargo fmt + cargo clippy
  ```bash
  cd src-tauri
  cargo fmt
  cargo clippy
  ```

### Error Handling

- **Rust**: Use `thiserror` for error types
- **Tauri Commands**: Convert errors to `String` at IPC boundary
- **TypeScript**: Handle errors gracefully with try-catch and user feedback

---

## Git Workflow

### Branch Naming

- **Features**: `feature/description` (e.g., `feature/add-snooze`)
- **Bug fixes**: `bugfix/description` (e.g., `bugfix/fix-audio-playback`)
- **Documentation**: `docs/description` (e.g., `docs/update-readme`)

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>: <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
```
feat: add snooze functionality to schedules
fix: resolve audio playback issue on system wake
docs: update installation instructions
```

### Pull Request Process

1. **Never commit directly to `main`** — all changes via PR
2. Create a feature branch from `main`
3. Make your changes following code conventions
4. Write/update tests as needed
5. Run linters and formatters
6. Push to your fork
7. Open a PR to `main` branch

### PR Requirements

- ✅ Passing CI checks (if configured)
- ✅ Code review approval
- ✅ No merge conflicts
- ✅ Updated documentation (if applicable)
- ✅ Tests for new features

---

## Testing

### Running Tests

**Frontend**:
```bash
npm run test
```

**Backend**:
```bash
cd src-tauri
cargo test
```

**All tests**:
```bash
npm run test && cd src-tauri && cargo test
```

### Testing Guidelines

- Write unit tests for all new features
- Frontend: Use Vitest + React Testing Library
- Backend: Use Rust's built-in test framework (`#[cfg(test)]`)
- Aim for meaningful test coverage, not just high percentages

---

## Architecture Overview

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

### Technology Stack

**Frontend**:
- React 19 + TypeScript 5.8
- Vite 7 + Tailwind CSS 4
- shadcn/ui (Radix primitives)
- Zustand 5
- react-hook-form
- Vitest + React Testing Library

**Backend**:
- Tauri 2.0 (Rust)
- tokio (async runtime)
- SQLite with sqlx
- rodio (audio playback)
- chrono (date/time)
- serde (serialization)

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

## Common Development Tasks

### Adding a New Schedule Feature

1. Define types in `src/types/schedule.ts` and `src-tauri/src/db/models.rs`
2. Add database migration if needed in `src-tauri/src/migrations/`
3. Update repository in `src-tauri/src/db/schedules.rs`
4. Create/update Tauri command in `src-tauri/src/commands/schedules.rs`
5. Update Zustand store in `src/stores/scheduleStore.ts`
6. Update UI components as needed
7. Write tests for new functionality

### Adding a New Tauri Command

1. Define the command in appropriate file under `src-tauri/src/commands/`
2. Register it in `src-tauri/src/lib.rs` in the `invoke_handler!` macro
3. Update TypeScript types if needed
4. Call from frontend using `invoke('command_name', { args })`

### Database Migrations

1. Create new SQL file in `src-tauri/src/migrations/`
2. Use sequential numbering: `00X_description.sql`
3. Migrations run automatically on app startup
4. Test thoroughly before committing

### Debugging

**Enable Rust debug logs**:
```bash
RUST_LOG=debug cargo tauri dev
```

**Frontend DevTools**:
- Press `Cmd+Option+I` (macOS) or `F12` in dev mode

**Database inspection**:
```bash
sqlite3 ~/Library/Application\ Support/com.yourname.resonatify/resonatify.db
```

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

## Database Schema

**Location**: `~/Library/Application Support/com.yourname.resonatify/resonatify.db`

### schedules
```sql
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
```

### settings
```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT
);
```

### audio_playback_history
```sql
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
- `open_audio_file_dialog()` → `Option<String>`

---

## Type Definitions

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

## Common Issues

### Dark/Light theme not applying
- **Fix**: Ensure `html[data-theme]` selectors exist, `@theme` directive positioned correctly
- **Reference**: `src/index.css`

### Schedule not executing
- Check: Schedule enabled, audio file exists, system not sleeping
- **Debug**: Check `audio_playback_history` table for errors

### Build errors
- **Rust toolchain**: Ensure latest stable via `rustup update`
- **Node modules**: Try `rm -rf node_modules && npm install`
- **Tauri CLI**: Update via `npm install -g @tauri-apps/cli@latest`

---

## Questions?

If you have questions or need help:

- Open a [GitHub Discussion](https://github.com/flup-repo/resonatify/discussions)
- Check existing [GitHub Issues](https://github.com/flup-repo/resonatify/issues)
- Review the [README.md](README.md) for project overview

---

**Thank you for contributing to Resonatify!**
