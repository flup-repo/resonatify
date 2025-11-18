# Resonatify

<div align="center">

**A desktop application for scheduling audio files to play at specific times**

Built with Tauri 2.0, React, and Rust

**Currently available for macOS 11 (Big Sur) and later**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE.md)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-24C8DB?logo=tauri)](https://tauri.app)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev)
[![Rust](https://img.shields.io/badge/Rust-Latest-orange?logo=rust)](https://www.rust-lang.org)

---

### 💖 Support This Project

If you find Resonatify useful and would like to support its continued development, consider buying me a coffee! Your support helps keep this project alive and enables new features.

[![Donate with PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.com/donate/?business=6SUDEH9BDL3WQ&no_recurring=0&item_name=Appreciate+your+kind+support%21&currency_code=EUR)

Every contribution, no matter how small, is greatly appreciated! 🙏

</div>

---

## Overview

**Resonatify** is a powerful yet simple audio scheduling application designed for users who need audio-based reminders, alarms, and scheduled playback. Whether you need a morning alarm, hourly reminders, or custom audio notifications, Resonatify has you covered.

### Key Highlights

- **Flexible Scheduling**: Support for one-time, daily, weekdays, weekends, weekly, and custom interval schedules
- **Native Performance**: Built with Rust backend for maximum efficiency and reliability
- **Modern UI**: Clean, intuitive interface with light/dark theme support
- **System Integration**: Lives in your system tray with minimal footprint
- **macOS Native**: Optimized for macOS 11 (Big Sur) and later versions

---

## Features

### Implemented
- ✅ **Schedule Management**: Create, edit, delete, and toggle schedules
- ✅ **Multiple Repeat Patterns**:
  - Once (single execution)
  - Daily
  - Weekdays (Monday-Friday)
  - Weekends (Saturday-Sunday)
  - Weekly (select specific days)
  - Custom intervals
- ✅ **Background Scheduler**: Efficient tokio-based engine runs in the background
- ✅ **Audio Playback**: High-quality playback with rodio (MP3, WAV, FLAC, OGG, M4A)
- ✅ **Volume Control**: Per-schedule volume settings (0-100%)
- ✅ **SQLite Database**: Persistent storage with automatic migrations
- ✅ **System Tray Integration**: Minimizes to tray with quick access
- ✅ **Theme System**: Light, dark, and system-matched themes
- ✅ **Settings Persistence**: All preferences saved automatically
- ✅ **Audio File Validation**: Ensures audio files are valid before scheduling
- ✅ **Playback History**: Track execution history with success/failure logs

### Upcoming
- 🔜 Notifications for schedule execution
- 🔜 Fade in/out audio effects
- 🔜 Snooze functionality
- 🔜 Import/export schedules
- 🔜 Cloud sync
- 🔜 Mobile companion app

---

## Tech Stack

### Frontend
- **Framework**: React 19 with TypeScript 5.8
- **Build Tool**: Vite 7
- **Styling**: Tailwind CSS 4
- **UI Components**: shadcn/ui (Radix UI primitives)
- **State Management**: Zustand 5
- **Form Handling**: react-hook-form
- **Testing**: Vitest + React Testing Library

### Backend
- **Framework**: Tauri 2.0
- **Language**: Rust (latest stable)
- **Async Runtime**: tokio
- **Database**: SQLite with sqlx
- **Audio Engine**: rodio
- **Date/Time**: chrono
- **Serialization**: serde

---

## Installation

### System Requirements
- **Operating System**: macOS 11 (Big Sur) or later
- **Architecture**: Intel or Apple Silicon (M1/M2/M3)

### Development Prerequisites
- **Node.js**: v18 or higher
- **Rust**: Latest stable (install via [rustup](https://rustup.rs))
- **Xcode Command Line Tools**: Required for macOS development
  ```bash
  xcode-select --install
  ```

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/flup-repo/resonatify.git
   cd resonatify
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   cargo tauri dev
   ```

   This will:
   - Start the Vite dev server
   - Compile the Rust backend
   - Launch the application with hot-reload

### Testing

Resonatify has a comprehensive test suite covering both frontend and backend.

#### 1. Backend Integration Tests (Rust)

Verify scheduler logic, database persistence, and execution flow.

```bash
# Run scheduler integration tests
cd src-tauri && cargo test --test scheduler_integration

# Run all backend tests (unit + integration)
cd src-tauri && cargo test
```

#### 2. Frontend E2E Tests (Playwright)

Verify UI interactions, CRUD operations, and settings management.

```bash
# Install Playwright browsers (first time only)
npx playwright install chromium

# Run E2E tests
npm run test:e2e

# Run with UI dashboard
npx playwright test --ui

# View test report
npx playwright show-report
```

#### 3. Unit Tests

```bash
# Frontend Unit Tests (Vitest)
npm run test

# Backend Unit Tests
cd src-tauri && cargo test
```

### Building for Production

```bash
# Build optimized production bundle
cargo tauri build
```

The built application will be in `src-tauri/target/release/bundle/`

---

## Usage

### Creating a Schedule

1. Click the **"New Schedule"** button or press `Cmd+N`
2. Fill in the schedule details:
   - **Name**: Descriptive name for your schedule
   - **Audio File**: Select an audio file (MP3, WAV, FLAC, OGG, M4A)
   - **Time**: Set the time (HH:mm format)
   - **Repeat**: Choose repeat pattern
   - **Volume**: Adjust volume (0-100%)
3. Click **"Create Schedule"**

### Managing Schedules

- **Toggle**: Click the switch to enable/disable a schedule
- **Edit**: Click the edit button on any schedule card
- **Delete**: Click the delete button (confirmation required)
- **Test**: Click the play button to preview audio

### Keyboard Shortcuts

- `Cmd + N`: Create new schedule
- `Cmd + ,`: Open settings
- `Cmd + Q`: Quit application

### Settings

Access settings via the gear icon or `Cmd + ,`:

- **Theme**: Light, Dark, or System
- **Launch at Login**: Start automatically with your system
- **Minimize to Tray**: Keep running in the background
- **Notifications**: Enable/disable playback notifications
- **Default Volume**: Set default volume for new schedules

---

## Project Structure

```
scheduler-app/
├── src/                          # React frontend
│   ├── components/               # React components
│   │   ├── ScheduleCard.tsx     # Schedule display card
│   │   ├── ScheduleList.tsx     # Schedule list with empty state
│   │   ├── ScheduleModal.tsx    # Add/edit dialog
│   │   ├── ScheduleHeader.tsx   # App header
│   │   ├── settings/            # Settings UI
│   │   └── ui/                  # shadcn/ui components
│   ├── stores/                  # Zustand state management
│   │   ├── scheduleStore.ts     # Schedule state
│   │   └── settingsStore.ts     # Settings state
│   ├── types/                   # TypeScript type definitions
│   ├── hooks/                   # Custom React hooks
│   ├── utils/                   # Utility functions
│   └── App.tsx                  # Main application component
│
├── src-tauri/                   # Rust backend
│   ├── src/
│   │   ├── main.rs             # Entry point
│   │   ├── lib.rs              # App setup & state
│   │   ├── commands/           # Tauri IPC commands
│   │   │   ├── schedules.rs    # Schedule CRUD
│   │   │   ├── scheduler.rs    # Scheduler control
│   │   │   ├── audio.rs        # Audio operations
│   │   │   ├── settings.rs     # Settings operations
│   │   │   └── dialogs.rs      # File dialogs
│   │   ├── scheduler/          # Scheduler engine
│   │   │   ├── engine.rs       # Main scheduler
│   │   │   └── time_calculator.rs
│   │   ├── audio/              # Audio service
│   │   │   ├── service.rs      # Audio playback
│   │   │   ├── player.rs       # Player state
│   │   │   └── validator.rs    # Format validation
│   │   ├── db/                 # Database layer
│   │   │   ├── models.rs       # SQLite models
│   │   │   ├── schedules.rs    # Schedule repository
│   │   │   ├── settings.rs     # Settings repository
│   │   │   └── playback_history.rs
│   │   └── tray.rs             # System tray
│   ├── migrations/             # Database migrations
│   └── Cargo.toml              # Rust dependencies
│
├── CONTRIBUTING.md              # Contributing guidelines
├── README.md                    # This file
├── LICENSE.md                   # License information
└── package.json                 # Node dependencies
```

---

## Architecture

### Data Flow

```
User Action → Component → Zustand Store → invoke(command) → Tauri Command
  → Business Logic → Repository → SQLite → Response → Store Update → UI
```

### Key Patterns

- **Repository Pattern**: Database access abstracted via repositories
- **State Management**: Zustand for UI state, SQLite for persistence
- **Async Runtime**: Tokio powers the background scheduler
- **IPC**: Type-safe communication between frontend and backend
- **Error Handling**: Rust's Result type with thiserror

### Database

**Location**: `~/Library/Application Support/com.yourname.resonatify/resonatify.db` (macOS)

**Tables**:
- `schedules`: User-defined audio schedules
- `settings`: Application settings (key-value store)
- `audio_playback_history`: Execution logs (auto-trimmed to 1000 entries)

---

## Development

### Code Quality

```bash
# Rust formatting and linting
cd src-tauri
cargo fmt
cargo clippy

# TypeScript linting
npm run lint
```

### Debugging

1. **Enable Rust debug logs**:
   ```bash
   RUST_LOG=debug cargo tauri dev
   ```

2. **Frontend DevTools**: Press `Cmd+Option+I` (macOS) or `F12` in dev mode

3. **Database inspection**:
   ```bash
   sqlite3 ~/Library/Application\ Support/com.yourname.resonatify/resonatify.db
   ```

---

## Contributing

We welcome contributions! Here's how to get started:

### Workflow

1. **Fork the repository**
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b bugfix/your-bugfix-name
   ```
3. **Make your changes**:
   - Follow code conventions (see CONTRIBUTING.md)
   - Write tests for new features
   - Run linters and formatters
4. **Commit your changes**:
   ```bash
   git commit -m "feat: add new feature"
   # Use conventional commits: feat, fix, docs, style, refactor, test, chore
   ```
5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
6. **Open a Pull Request** to the `main` branch

### Conventions

- **Commits**: Use [Conventional Commits](https://www.conventionalcommits.org/)
- **Code Style**:
  - TypeScript: ESLint + Prettier
  - Rust: `cargo fmt` + `cargo clippy`
- **Testing**: Write unit tests for all new features
- **Documentation**: Update documentation as needed

### Code Review

All PRs require:
- Passing CI checks
- Code review approval
- No merge conflicts
- Updated documentation (if applicable)

---

## Troubleshooting

### Schedule not executing

**Check**:
- Schedule is enabled (toggle on)
- Audio file exists at specified path
- System is not in sleep mode
- Check `audio_playback_history` table for error logs

**Debug**:
```bash
# View playback history
sqlite3 ~/Library/Application\ Support/com.yourname.resonatify/resonatify.db \
  "SELECT * FROM audio_playback_history ORDER BY played_at DESC LIMIT 10;"
```

### Audio file not playing

**Check**:
- File format is supported (MP3, WAV, FLAC, OGG, M4A)
- File is not corrupted
- Volume is not set to 0
- System audio is working

**Test playback**:
Use the test button on the schedule card to verify audio playback.

### Theme not applying

**Fix**:
1. Check Settings → Theme is set correctly
2. Try toggling between themes
3. Restart the application

### Build errors

**Common issues**:
- **Rust toolchain**: Ensure latest stable via `rustup update`
- **Node modules**: Try `rm -rf node_modules && npm install`
- **Tauri CLI**: Update via `npm install -g @tauri-apps/cli@latest`

---

## Roadmap

### Version 0.2.0
- [ ] Push notifications for schedule execution
- [ ] Fade in/out audio effects
- [ ] Snooze functionality
- [ ] Multi-language support

### Version 0.3.0
- [ ] **Windows 11 support**
- [ ] **Linux support** (Ubuntu, Fedora, Arch)
- [ ] Import/export schedules (JSON)
- [ ] Schedule templates library
- [ ] Audio playlist support
- [ ] Advanced repeat patterns (monthly, yearly)

### Version 1.0.0
- [ ] Cloud sync (optional)
- [ ] Mobile companion app
- [ ] Plugin system
- [ ] Web dashboard

---

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

**Copyright © 2025 flup-repo**

---

## Acknowledgments

- [Tauri](https://tauri.app) - For the amazing desktop app framework
- [shadcn/ui](https://ui.shadcn.com) - For the beautiful UI components
- [Rodio](https://github.com/RustAudio/rodio) - For reliable audio playback
- All contributors who help improve Resonatify

---

## Support

- **Issues**: [GitHub Issues](https://github.com/flup-repo/resonatify/issues)
- **Discussions**: [GitHub Discussions](https://github.com/flup-repo/resonatify/discussions)
- **Documentation**: See [CONTRIBUTING.md](CONTRIBUTING.md) for developer guidelines

---

<div align="center">

**Made with ❤️ by [flup-repo](https://github.com/flup-repo)**

If you find this project useful, please consider giving it a ⭐️

</div>
