# Resonator - AI Assistant Reference

> **Quick Context File for AI Assistants**
> This file provides essential information about the Resonator project to help AI assistants understand the codebase structure, conventions, and technical decisions.

---

## Project Overview

**Resonator** is a cross-platform desktop application that allows users to schedule audio files to play at specific times. It functions as a background process similar to a cron job execution engine.

- **Primary Platform**: macOS (initial release)
- **Future Platforms**: Windows, Linux
- **Development Status**: Planning/Initial Development Phase
- **Target Users**: Individuals needing audio reminders (meditation, breaks, alarms)

### Core Features
- Schedule audio files to play at specific times
- Multiple repeat patterns (once, daily, weekdays, weekends, custom)
- Background execution with system tray integration
- Simple, intuitive user interface
- Settings persistence and personalization
- macOS native integration (notifications, launch at login)

### Future Features (Post-MVP)
- Expanded actions (open apps, display messages, run scripts)
- In-app audio recording
- AI integration (API-driven tasks)
- Windows and Linux support

---

## Technology Stack

### Frontend
- **Framework**: React 18+ with TypeScript
- **Build Tool**: Vite
- **Styling**: Tailwind CSS
- **Component Library**: shadcn/ui
- **State Management**: Zustand
- **Forms**: react-hook-form
- **Date/Time**: date-fns
- **Testing**: Vitest + React Testing Library

### Backend (Rust via Tauri)
- **Framework**: Tauri 2.0
- **Runtime**: tokio (async)
- **Database**: SQLite with sqlx/rusqlite
- **Audio Playback**: rodio
- **Date/Time**: chrono
- **Serialization**: serde
- **Plugins**:
  - tauri-plugin-autostart (launch at login)
  - tauri-plugin-notification (system notifications)
  - tauri-plugin-updater (auto-updates)

### Development Tools
- **Language**: Rust (backend), TypeScript (frontend)
- **Package Manager**: npm (frontend), cargo (backend)
- **Version Control**: Git
- **CI/CD**: GitHub Actions
- **Testing**: Rust built-in testing + Vitest + Playwright

---

## Project Structure

```
resonator/
├── docs/
│   ├── plan-new-app.md           # Original requirements
│   └── app-plan-details.md       # Comprehensive development plan
├── src/                          # React frontend
│   ├── components/               # React components
│   │   ├── ScheduleCard.tsx     # Individual schedule display
│   │   ├── ScheduleList.tsx     # Main schedule list
│   │   ├── ScheduleModal.tsx    # Add/edit modal
│   │   ├── Header.tsx           # App header
│   │   └── settings/            # Settings components
│   ├── stores/                  # Zustand state stores
│   │   ├── scheduleStore.ts     # Schedule state management
│   │   └── settingsStore.ts     # Settings state management
│   ├── types/                   # TypeScript type definitions
│   │   └── schedule.ts          # Schedule-related types
│   ├── utils/                   # Utility functions
│   ├── hooks/                   # Custom React hooks
│   ├── App.tsx                  # Main app component
│   └── main.tsx                 # Entry point
├── src-tauri/                   # Rust backend
│   ├── src/
│   │   ├── main.rs              # Tauri app entry point
│   │   ├── audio/               # Audio playback module
│   │   │   ├── player.rs        # Audio player implementation
│   │   │   └── validator.rs    # Audio file validation
│   │   ├── scheduler/           # Scheduling engine
│   │   │   ├── engine.rs        # Main scheduler engine
│   │   │   ├── time_calculator.rs  # Time calculation logic
│   │   │   └── models.rs        # Schedule models
│   │   ├── db/                  # Database layer
│   │   │   ├── mod.rs           # Database initialization
│   │   │   ├── schedules.rs     # Schedule repository
│   │   │   └── settings.rs      # Settings repository
│   │   ├── commands/            # Tauri commands (IPC)
│   │   │   ├── schedules.rs     # Schedule CRUD commands
│   │   │   ├── settings.rs      # Settings commands
│   │   │   └── audio.rs         # Audio commands
│   │   └── tray.rs              # System tray implementation
│   ├── migrations/              # Database migrations
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── tests/                       # Integration and E2E tests
├── CLAUDE.md                    # This file
└── package.json                 # Node dependencies
```

---

## Key Architecture Patterns

### Data Flow
1. **User Action** → Frontend UI Component
2. **Component** → Zustand Store Action
3. **Store** → Tauri IPC Command (`invoke()`)
4. **Tauri Command** → Rust Backend Logic
5. **Backend** → Database/Scheduler Engine/Audio Player
6. **Response** → Back through the chain to update UI

### Repository Pattern
- All database access goes through repository classes
- Repositories provide clean interfaces: `create()`, `get_all()`, `get_by_id()`, `update()`, `delete()`
- Example: `ScheduleRepository`, `SettingsRepository`

### State Management
- **Frontend State**: Zustand stores (lightweight, minimal boilerplate)
- **Persistent State**: SQLite database
- **Runtime State**: Scheduler engine maintains active schedules in memory

### Async Pattern
- Backend uses tokio for async operations
- Frontend uses async/await with Tauri's `invoke()`
- Scheduling uses `tokio::spawn` for background tasks

---

## Database Schema

### Schedules Table
```sql
CREATE TABLE schedules (
    id TEXT PRIMARY KEY,              -- UUID
    name TEXT NOT NULL,               -- User-friendly name
    audio_file_path TEXT NOT NULL,   -- Absolute path to audio file
    scheduled_time TEXT NOT NULL,    -- HH:mm format
    enabled INTEGER DEFAULT 1,        -- Boolean (1=enabled, 0=disabled)
    repeat_type TEXT,                 -- 'once', 'daily', 'weekly', 'weekdays', 'weekends'
    repeat_days TEXT,                 -- JSON array for custom days [0-6]
    volume INTEGER DEFAULT 100,       -- 0-100
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

### Settings Table
```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

### Playback History Table
```sql
CREATE TABLE audio_playback_history (
    id TEXT PRIMARY KEY,
    schedule_id TEXT,
    played_at TEXT DEFAULT CURRENT_TIMESTAMP,
    status TEXT,  -- 'success', 'failed', 'skipped'
    FOREIGN KEY (schedule_id) REFERENCES schedules(id)
);
```

---

## Key TypeScript Types

```typescript
// Schedule
interface Schedule {
    id: string;
    name: string;
    audioFilePath: string;
    scheduledTime: string;        // HH:mm format
    enabled: boolean;
    repeatType: RepeatType;
    repeatDays?: number[];        // 0-6 (Sun-Sat)
    volume: number;               // 0-100
    createdAt: string;
    updatedAt: string;
}

// Repeat Types
type RepeatType = 'once' | 'daily' | 'weekly' | 'weekdays' | 'weekends' | 'custom';

// Settings
interface Settings {
    theme: 'light' | 'dark' | 'system';
    launchAtLogin: boolean;
    showNotifications: boolean;
    notificationSound: boolean;
    defaultVolume: number;
    minimizeToTray: boolean;
}
```

---

## Key Rust Types

```rust
// Schedule Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: String,
    pub name: String,
    pub audio_file_path: String,
    pub scheduled_time: String,  // HH:mm format
    pub enabled: bool,
    pub repeat_type: RepeatType,
    pub repeat_days: Option<Vec<u8>>,
    pub volume: u8,
    pub created_at: String,
    pub updated_at: String,
}

// Repeat Type Enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatType {
    Once,
    Daily,
    Weekly { days: Vec<Weekday> },
    Weekdays,
    Weekends,
    Custom { interval_minutes: u32 },
}
```

---

## Important Conventions

### Naming Conventions
- **Frontend Files**: PascalCase for components (`ScheduleCard.tsx`), camelCase for utilities
- **Backend Files**: snake_case for all Rust files (`time_calculator.rs`)
- **Functions**: camelCase (TypeScript), snake_case (Rust)
- **Components**: PascalCase
- **Constants**: UPPER_SNAKE_CASE

### Code Style
- **Rust**: Follow `rustfmt` standard formatting
- **TypeScript**: ESLint + Prettier configuration
- **Imports**: Group by external → internal → relative
- **Error Handling**:
  - Rust: Use `Result<T, E>` with custom error types
  - TypeScript: Try-catch with user-friendly error messages

### Git Commit Messages
- Use conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
- Examples:
  - `feat: add schedule repeat configuration`
  - `fix: correct time zone calculation in scheduler`
  - `docs: update API documentation`

---

## Common Commands

### Development
```bash
# Start development server (both frontend and backend)
cargo tauri dev

# Run frontend only
npm run dev

# Run backend tests
cd src-tauri && cargo test

# Run frontend tests
npm run test

# Build for production
cargo tauri build
```

### Database
```bash
# Create new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Code Quality
```bash
# Format Rust code
cargo fmt

# Lint Rust code
cargo clippy

# Format TypeScript code
npm run format

# Lint TypeScript code
npm run lint
```

---

## Tauri IPC Commands

### Schedule Commands
```rust
// Get all schedules
get_all_schedules() -> Result<Vec<Schedule>, String>

// Create schedule
create_schedule(input: CreateScheduleInput) -> Result<Schedule, String>

// Update schedule
update_schedule(id: String, input: UpdateScheduleInput) -> Result<Schedule, String>

// Delete schedule
delete_schedule(id: String) -> Result<(), String>

// Toggle schedule enabled/disabled
toggle_schedule(id: String) -> Result<Schedule, String>
```

### Settings Commands
```rust
// Get all settings
get_settings() -> Result<HashMap<String, String>, String>

// Update settings
update_settings(settings: HashMap<String, String>) -> Result<(), String>

// Set launch at login
set_launch_at_login(enabled: bool) -> Result<(), String>
```

### Audio Commands
```rust
// Play audio file (manual trigger)
play_audio_file(path: String, volume: f32) -> Result<(), String>

// Stop current audio
stop_audio() -> Result<(), String>

// Get playback status
get_playback_status() -> Result<PlaybackStatus, String>
```

### Scheduler Commands
```rust
// Start scheduler engine
start_scheduler() -> Result<(), String>

// Get scheduler status
get_scheduler_status() -> Result<SchedulerStatus, String>

// Get upcoming executions
get_upcoming_executions(count: usize) -> Result<Vec<ScheduleExecution>, String>
```

---

## Frontend State Management

### Schedule Store (Zustand)
```typescript
const useScheduleStore = create<ScheduleStore>((set, get) => ({
    schedules: [],
    isLoading: false,
    error: null,

    fetchSchedules: async () => {
        const schedules = await invoke<Schedule[]>('get_all_schedules');
        set({ schedules });
    },

    createSchedule: async (input) => {
        const schedule = await invoke<Schedule>('create_schedule', { input });
        set(state => ({ schedules: [...state.schedules, schedule] }));
    },

    // ... other methods
}));
```

### Settings Store (Zustand with Persistence)
```typescript
const useSettingsStore = create<SettingsStore>()(
    persist(
        (set, get) => ({
            theme: 'system',
            launchAtLogin: false,
            // ... other settings

            updateTheme: async (theme) => {
                set({ theme });
                await invoke('update_settings', { settings: { theme } });
            },
        }),
        { name: 'scheduler-settings' }
    )
);
```

---

## Testing Strategy

### Unit Tests (Rust)
- Test time calculation logic
- Test repeat type logic
- Test database operations
- Test audio validation
- Location: `src-tauri/src/**/*_test.rs` or `#[cfg(test)]` blocks

### Unit Tests (TypeScript)
- Test utility functions
- Test components in isolation
- Test store logic
- Location: `src/**/*.test.ts` or `src/**/*.test.tsx`

### Integration Tests
- Test schedule creation → execution flow
- Test database persistence
- Test frontend-backend communication
- Location: `tests/integration/`

### E2E Tests (Playwright)
- Test complete user workflows
- Test UI interactions
- Location: `tests/e2e/`

### Manual Testing Checklist
- See `docs/app-plan-details.md` Phase 5.3 for comprehensive checklist

---

## macOS Specific Considerations

### Permissions Required
- **Notifications**: Request on first run
- **File Access**: File dialog provides scoped access
- **Microphone**: Required for future audio recording feature

### Launch at Login
- Uses macOS LaunchAgent
- Configured via `tauri-plugin-autostart`
- Plist file created automatically

### Code Signing & Notarization
- Required for distribution outside App Store
- Apple Developer Program membership required ($99/year)
- Hardened runtime enabled
- Entitlements file: `src-tauri/entitlements.plist`

### System Tray
- Use template images (monochrome) for native look
- Support retina displays (2x, 3x assets)
- Update icon based on active/inactive state

---

## Audio Playback

### Supported Formats
- MP3
- WAV
- FLAC
- OGG/Vorbis
- M4A/AAC (requires symphonia crate)

### Audio Library (rodio)
```rust
use rodio::{Decoder, OutputStream, Sink};

pub struct AudioPlayer {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioPlayer {
    pub fn play(&self, file_path: &str) -> Result<(), AudioError> {
        let file = File::open(file_path)?;
        let source = Decoder::new(BufReader::new(file))?;
        self.sink.append(source);
        Ok(())
    }
}
```

### Volume Control
- Range: 0-100 (UI) → 0.0-1.0 (rodio)
- Per-schedule volume setting
- Default volume in settings

---

## Scheduler Engine

### Time Calculation
```rust
// Calculate next execution time
pub fn next_execution_time(
    schedule: &Schedule,
    now: DateTime<Local>
) -> Option<DateTime<Local>> {
    // 1. Parse scheduled_time (HH:mm)
    // 2. Check if should run today based on repeat_type
    // 3. If past today's time, calculate next valid day
    // 4. Return next execution DateTime
}
```

### Repeat Logic
- **Once**: Execute once, then disable
- **Daily**: Every day at specified time
- **Weekdays**: Monday-Friday only
- **Weekends**: Saturday-Sunday only
- **Weekly/Custom**: User-selected days (0=Sunday, 6=Saturday)

### Background Execution
```rust
// Each enabled schedule gets its own tokio task
tokio::spawn(async move {
    loop {
        let next_time = calculate_next_execution(&schedule);
        let delay = next_time - now();
        tokio::time::sleep(delay).await;

        execute_schedule(&schedule).await;

        if schedule.repeat_type == RepeatType::Once {
            break;
        }
    }
});
```

---

## Error Handling

### Backend Error Pattern
```rust
#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Audio error: {0}")]
    AudioError(#[from] AudioError),

    #[error("Schedule not found: {0}")]
    NotFound(String),
}

// Convert to string for Tauri commands
.map_err(|e| e.to_string())
```

### Frontend Error Pattern
```typescript
try {
    await invoke('create_schedule', { input });
} catch (error) {
    // Show user-friendly error message
    toast.error(error instanceof Error ? error.message : 'Failed to create schedule');
    console.error('Schedule creation error:', error);
}
```

---

## Performance Targets

- **Startup Time**: <1 second
- **Memory Usage (Idle)**: <50MB
- **CPU Usage (Idle)**: <5%
- **CPU Usage (Playing)**: <20%
- **Schedule Execution Accuracy**: 99.9% (within 1 second)
- **Max Schedules**: 1000+ without performance degradation

---

## Security Considerations

1. **File Access**: Only access user-selected files via dialog
2. **SQL Injection**: Use parameterized queries (sqlx handles this)
3. **Script Execution**: Validate and sanitize (future feature)
4. **API Keys**: Store securely in system keychain (future AI feature)
5. **Code Signing**: Required for macOS distribution
6. **Update Verification**: Verify signatures on auto-updates

---

## Deployment Checklist

- [ ] Update version in `package.json`, `Cargo.toml`, `tauri.conf.json`
- [ ] Run full test suite
- [ ] Build universal binary (`cargo tauri build --target universal-apple-darwin`)
- [ ] Sign the application
- [ ] Create DMG installer
- [ ] Notarize with Apple
- [ ] Staple notarization ticket
- [ ] Upload to GitHub Releases
- [ ] Update website download link
- [ ] Update changelog

---

## Useful Resources

### Documentation
- **Tauri Docs**: https://tauri.app/
- **Rust Book**: https://doc.rust-lang.org/book/
- **React Docs**: https://react.dev/
- **shadcn/ui**: https://ui.shadcn.com/

### Project Documentation
- **Requirements**: `docs/plan-new-app.md`
- **Detailed Plan**: `docs/app-plan-details.md`
- **This File**: `CLAUDE.md`

### Libraries
- **rodio**: https://github.com/RustAudio/rodio
- **chrono**: https://github.com/chronotope/chrono
- **tokio**: https://tokio.rs/
- **zustand**: https://github.com/pmndrs/zustand

### Design
- **macOS HIG**: https://developer.apple.com/design/human-interface-guidelines/macos
- **Tailwind**: https://tailwindcss.com/

---

## Common Issues & Solutions

### Issue: Schedule doesn't execute at exact time
- **Cause**: System sleep/wake, time zone changes
- **Solution**: Implement catch-up logic, handle power events

### Issue: Audio file not playing
- **Cause**: Unsupported format, file not found, audio device disconnected
- **Solution**: Validate file on selection, check format, handle device errors

### Issue: App won't launch on user's Mac
- **Cause**: Not signed/notarized
- **Solution**: Ensure proper code signing and notarization

### Issue: High memory usage
- **Cause**: Large audio files loaded into memory
- **Solution**: Use streaming for large files

### Issue: Schedules lost after app restart
- **Cause**: Database not persisted, scheduler not reloaded
- **Solution**: Ensure DB writes, reload schedules on startup

---

## Development Phases (Reference)

1. **✅ Research & Planning** - Weeks 1-2 (COMPLETE: 2025-11-13)
   - ✅ 1.1 Market Research & Competitive Analysis (Complete: 2025-11-13)
   - ✅ 1.2 User Experience Design (Complete: 2025-11-13)
   - ✅ 1.3 Architecture Design (Complete: 2025-11-13)
2. **⏳ Project Setup** - Week 2-3
3. **⏳ Core Development** - Weeks 3-10
4. **⏳ macOS Implementation** - Weeks 10-11
5. **⏳ Testing** - Weeks 11-13
6. **⏳ Deployment** - Weeks 13-14
7. **🔮 Future Features** - Week 14+

See `docs/app-plan-details.md` for complete phase breakdown.

---

## Quick Start for AI Assistants

When starting a new session:

1. **Read this file first** for project context
2. **Check `docs/app-plan-details.md`** for detailed implementation plans
3. **Review current phase** in development timeline
4. **Follow conventions** outlined in this document
5. **Use Tauri IPC commands** for frontend-backend communication
6. **Write tests** for new functionality
7. **Update this file** if architecture changes

---

## Project Goals

### MVP Goals
- ✅ Schedule audio playback at specific times
- ✅ Support multiple repeat patterns
- ✅ macOS native integration
- ✅ Simple, intuitive UI
- ✅ Reliable background execution

### Post-MVP Goals
- 🔮 Windows and Linux support
- 🔮 Expanded action types
- 🔮 In-app audio recording
- 🔮 AI integration
- 🔮 App Store distribution

---

**Last Updated**: 2025-11-13
**Version**: 1.0.0 (Pre-development)
**Status**: Planning Phase

---

## Notes for AI Assistants

- **Be consistent**: Follow the patterns established in this document
- **Be thorough**: Write tests, handle errors, document changes
- **Be pragmatic**: Focus on MVP features first, defer nice-to-haves
- **Be secure**: Validate inputs, handle permissions, follow best practices
- **Be helpful**: Explain decisions, suggest improvements, ask clarifying questions

Good luck building Resonator! 🎵⏰
