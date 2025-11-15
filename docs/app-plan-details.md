# Comprehensive Development Plan: Cross-Platform Audio Scheduling Application

## Executive Summary

This document outlines a detailed, step-by-step plan for developing a cross-platform scheduling application that allows users to schedule audio files to play at specific times. The application will initially target macOS, with future expansion to Windows and Linux. The app will function as a background process with an intuitive UI, settings persistence, and donation-based monetization.

## Technology Stack Recommendation

Based on research of similar applications and cross-platform development best practices in 2025, the recommended technology stack is:

### Primary Framework: **Tauri 2.0**

**Rationale:**
- **Performance**: Tauri apps start in ~0.4s and use ~30-40MB RAM (vs Electron's 1.5s and 250MB)
- **Bundle Size**: Tauri apps are 2.5-3MB (vs Electron's 100MB+), using native OS web renderers
- **Cross-Platform**: Single codebase for macOS, Windows, Linux, with mobile (iOS/Android) support in Tauri 2.0
- **Native Integration**: Excellent system tray, notifications, and launch agent support
- **Modern**: Stable v2 release in 2025 with mature ecosystem
- **Security**: Rust-based backend provides memory safety and security benefits

**Alternative Consideration:**
- Electron if you need consistent rendering across all platforms or advanced WebView features
- JUCE if building a pure C++ audio-focused application

### Frontend: **React + TypeScript + Vite**

**Rationale:**
- Large ecosystem and component libraries
- TypeScript provides type safety for complex scheduling logic
- Vite offers fast development and hot module replacement
- Works seamlessly with Tauri

### Styling: **Tailwind CSS + shadcn/ui**

**Rationale:**
- Rapid UI development with utility classes
- shadcn/ui provides accessible, customizable components
- Easy to implement light/dark themes for macOS integration

### Backend (Rust via Tauri): **Core Libraries**

- **tokio**: Async runtime for scheduling engine
- **chrono**: Date/time manipulation for scheduling
- **rodio**: Cross-platform audio playback
- **serde**: Serialization for settings persistence
- **tauri-plugin-autostart**: Launch at login functionality
- **tauri-plugin-notification**: System notifications
- **sqlx or rusqlite**: Local database for schedules and settings

### State Management: **Zustand or Jotai**

**Rationale:**
- Lightweight state management for React
- Less boilerplate than Redux
- Perfect for managing schedules, settings, and UI state

### Testing:
- **Frontend**: Vitest + React Testing Library
- **Backend**: Rust's built-in testing framework
- **E2E**: Playwright or Tauri's testing tools

---

## Phase 1: Research & Planning

**Estimated Duration:** 1-2 weeks
**Complexity:** Low-Medium

### 1.1 Market Research & Competitive Analysis ✅ COMPLETE

**Completion Date:** 2025-11-13

**Tasks:**
1. ✅ Research existing macOS scheduling/reminder apps:
   - Analyzed apps: Timer+, Countdown Timer, Focus, Apimac Timer, Be Focused, Horo, Timeless, Due, Pester, Wake Up Time
   - Documented UI/UX patterns (menu bar placement, notification styles, timer input methods)
   - Noted pricing models and feature sets
   - Identified gaps in the market

2. ✅ Research audio playback apps:
   - Examined how apps handle audio file selection
   - Documented supported audio formats (MP3, WAV, M4A, FLAC, OGG)
   - Noted volume control and audio output device selection patterns
   - Analyzed players: Elmedia Player, VOX, IINA, Pine Player, Audirvana

3. ✅ Analyze background process implementations:
   - Studied how macOS apps implement launch agents
   - Reviewed system tray/menu bar app patterns
   - Documented user permission flows for background execution
   - Researched LaunchAgents, LaunchDaemons, and XPC login items

**Deliverables:**
- ✅ Competitive analysis document (`docs/competitive-analysis.md`)
- ✅ UI/UX inspiration board (`docs/ui-ux-inspiration.md`)
- ✅ Feature prioritization matrix (included in competitive analysis)

**Technical Considerations:**
- ✅ Focus on apps with minimal, non-intrusive interfaces
- ✅ Note how apps handle multiple simultaneous schedules
- ✅ Identify common pain points in existing solutions

**Key Findings:**
- Strong market demand for timer/scheduling apps
- Significant gap for dedicated audio file scheduling
- Most competitors are either too basic (Pester, Wake Up Time) or too complex (Apimac Timer)
- Users value menu bar integration, native macOS design, and reliability
- Freemium and donation models are most common
- Audio format support must include MP3, WAV, FLAC, M4A, OGG
- Launch agents are the recommended approach for background execution

### 1.2 User Experience Design ✅ COMPLETE

**Completion Date:** 2025-11-13

**Tasks:**
1. ✅ Define user personas:
   - Primary: Sarah (Mindful Professional), Mark (Remote Worker with ADHD), Linda (Yoga Instructor)
   - Secondary: Alex (Content Creator), Robert (Musician)
   - Detailed personas with demographics, goals, pain points, usage patterns

2. ✅ Create user journey maps:
   - Onboarding flow (first-time user experience)
   - Creating first schedule (step-by-step with emotions and opportunities)
   - Managing multiple schedules (review, edit, disable)
   - Editing settings (launch at login, theme, volume)
   - Schedule execution (passive experience)

3. ✅ Wireframe key screens:
   - Main window (empty state and with schedules)
   - Add/Edit schedule modal (complete form layout)
   - Settings panel (sidebar navigation with all 5 sections)
   - Menu bar icon states (idle, active, paused, error)
   - Menu bar menu (dynamic content)
   - Notification designs (execution, complete, error, pre-execution)
   - Confirmation dialogs (delete, apply to all)
   - Loading and error states
   - Responsive behavior (minimum 600px width)
   - Keyboard navigation map
   - Accessibility annotations

**Deliverables:**
- ✅ User personas document (`docs/user-personas.md`)
  - 5 detailed personas with usage matrix
  - Design implications for each persona
  - Persona validation checklist
- ✅ Wireframes (`docs/wireframes.md`)
  - 12+ screen wireframes (ASCII art style)
  - Complete component specifications
  - Interaction patterns documented
  - Accessibility requirements mapped
- ✅ User journey maps (`docs/user-journey-maps.md`)
  - 5 complete journeys with emotional arcs
  - Pain points and opportunities identified
  - Cross-journey insights and optimization checklist

**Technical Considerations:**
- ✅ Design for keyboard shortcuts (power users) - Complete navigation map
- ✅ Consider accessibility (VoiceOver support, contrast ratios) - WCAG AA/AAA targets set
- ✅ Plan for both light and dark modes - Color palettes defined in UI/UX inspiration doc

**Key Design Decisions:**
- Sidebar navigation for settings (macOS System Settings style)
- Card-based schedule list (scannable, modern)
- Auto-save for all settings (no Save button needed)
- Menu bar-first approach (app runs in background)
- Confirmation dialogs for destructive actions only
- Minimum 44x44px touch targets for accessibility
- Fast, responsive interactions (<200ms target)

### 1.3 Architecture Design ✅ COMPLETE

**Completion Date:** 2025-11-13

**Tasks:**
1. ✅ Design system architecture:
   - Frontend-backend communication flow (Tauri IPC with message-passing)
   - Scheduling engine architecture (Tokio-based async tasks)
   - Audio playback subsystem (Rodio with error handling)
   - Data persistence layer (SQLite with Repository pattern)
   - Background process lifecycle (Task management with JoinHandles)

2. ✅ Define data models:
   - Schedule entity (TypeScript & Rust types with full specifications)
   - Settings entity (Key-value store with type definitions)
   - Audio file metadata (Validation and info structures)
   - Playback history (Tracking execution success/failure)

3. ✅ Design API contracts:
   - Schedule commands (get_all, create, update, delete, toggle)
   - Settings commands (get, update, launch_at_login)
   - Audio commands (validate, play, stop, get_status)
   - Scheduler commands (start, stop, pause_all, resume_all, reload)
   - 20+ Tauri commands fully specified

**Deliverables:**
- ✅ Architecture design document (`docs/architecture-design.md`)
  - High-level system architecture with diagrams
  - Complete component architecture (frontend & backend)
  - Detailed data models (TypeScript & Rust)
  - Full API contract specifications
  - Database schema with migrations
  - Scheduling engine architecture with time calculation logic
  - Audio playback system design
  - Frontend architecture (Zustand stores, component structure)
  - Error handling strategy
  - Security considerations
  - Performance targets and optimizations
  - 3 architecture diagrams (ASCII art)

**Technical Considerations:**
- ✅ Use message-passing pattern for Tauri IPC - Implemented with invoke/emit pattern
- ✅ Design for eventual multi-platform support - Tauri abstractions in place
- ✅ Plan for data migration between versions - SQLite migrations system designed

**Key Architectural Decisions:**
- Repository pattern for testable data access
- Tokio spawn per schedule for independent task management
- Event-driven updates (frontend listens to backend events)
- Arc<Mutex<>> for shared state in Rust
- Zustand for lightweight frontend state management
- SQLite with sqlx runtime query builders (no offline preparation required)
- Rodio for cross-platform audio (with symphonia for M4A)

---

## Phase 2: Project Setup

**Estimated Duration:** 3-5 days
**Complexity:** Low

### 2.1 Development Environment Setup ✅ COMPLETE

**Status**: ✅ Complete
**Completion Date**: 2025-11-13

**Tasks:**
1. ✅ Install development tools:
   - ✅ Rust 1.91.1 installed via rustup
   - ✅ Cargo 1.91.1 installed (comes with Rust)
   - ✅ Node.js v22.17.0 (already installed)
   - ✅ npm 10.9.2 (already installed)
   - ✅ Xcode Command Line Tools (already installed)
   - ✅ Tauri CLI v2.9.4 installed via cargo

2. ✅ Configure IDE:
   - ✅ Created .vscode/settings.json with:
     - Format on save enabled
     - Rust analyzer configuration
     - ESLint and Prettier integration
     - Language-specific settings
     - Tailwind CSS IntelliSense configuration
     - Cargo bin added to PATH
   - ✅ Created .vscode/extensions.json with recommended extensions:
     - rust-analyzer (Rust development)
     - tauri-apps.tauri-vscode (Tauri support)
     - dbaeumer.vscode-eslint (ESLint)
     - esbenp.prettier-vscode (Prettier)
     - bradlc.vscode-tailwindcss (Tailwind CSS)
     - Additional utilities (GitLens, Error Lens, etc.)
   - ✅ Created .eslintrc.cjs with:
     - TypeScript ESLint configuration
     - React and React Hooks rules
     - JSX accessibility rules
     - Prettier integration
   - ✅ Created .prettierrc with:
     - Code formatting rules (100 char line width, single quotes, etc.)
     - Markdown-specific overrides
   - ✅ Created .prettierignore to exclude build artifacts

3. ✅ Set up version control:
   - ✅ Initialize Git repository (main branch)
   - ✅ Create .gitignore (comprehensive: Rust target/, node_modules/, build artifacts, OS files, IDE configs)
   - ✅ Set up GitHub remote (https://github.com/flup-repo/resonatify.git)
   - ✅ Create README.md with project overview
   - ✅ Initial commit with all Phase 1 documentation (commit: 62b7a9a)
   - ✅ Pushed to remote repository
   - ✅ Git branching strategy documented in CLAUDE.md

**Deliverables:**
- ✅ Configured development environment
  - Rust 1.91.1, Cargo 1.91.1, Tauri CLI 2.9.4
  - Node.js v22.17.0, npm 10.9.2
  - Xcode Command Line Tools
- ✅ VSCode configuration
  - settings.json (editor, formatting, language-specific settings)
  - extensions.json (16 recommended extensions)
  - ESLint configuration (.eslintrc.cjs)
  - Prettier configuration (.prettierrc, .prettierignore)
- ✅ Git repository with initial commit
  - Remote: https://github.com/flup-repo/resonatify.git
  - Branch: main (tracking origin/main)
  - Files: 12 files, 8,763 lines committed
  - Includes: README.md, BRANDING.md, CLAUDE.md, complete docs/ directory
  - Git branching strategy: feature branches with PR workflow

**Technical Considerations:**
- ✅ macOS development environment ready (Xcode Command Line Tools installed)
- ⏳ Code signing certificates setup deferred to Phase 4.3
- Cargo bin directory (~/.cargo/bin) added to PATH in VSCode terminal

**Progress Notes:**
- Development tools installed successfully on Apple Silicon Mac
- Git repository successfully initialized and pushed to GitHub
- Comprehensive .gitignore covers Tauri, React, Rust, and cross-platform development
- README.md provides professional project overview with badges and roadmap
- All Phase 1 documentation (8,700+ lines) committed and versioned
- VSCode workspace configured for Rust, TypeScript, React development
- Linting and formatting rules established for code quality
- Git branching strategy documented for consistent workflow

### 2.2 Initialize Tauri Project ✅ COMPLETE

**Status**: ✅ Complete
**Completion Date**: 2025-11-13

**Tasks:**
1. ✅ Create new Tauri project:
   ```bash
   cargo create-tauri-app
   # Choose: React + TypeScript
   ```

2. Configure project structure:
   ```
   scheduler-app/
   ├── src/               # React frontend
   │   ├── components/
   │   ├── hooks/
   │   ├── stores/
   │   ├── types/
   │   ├── utils/
   │   ├── App.tsx
   │   └── main.tsx
   ├── src-tauri/         # Rust backend
   │   ├── src/
   │   │   ├── audio/     # Audio playback module
   │   │   ├── scheduler/ # Scheduling engine
   │   │   ├── db/        # Database layer
   │   │   ├── commands/  # Tauri commands
   │   │   └── main.rs
   │   ├── Cargo.toml
   │   └── tauri.conf.json
   ├── docs/
   ├── tests/
   └── package.json
   ```

3. Install core dependencies:
   ```json
   // package.json
   {
     "dependencies": {
       "react": "^18.2.0",
       "@tauri-apps/api": "^2.0.0",
       "zustand": "^4.5.0",
       "react-hook-form": "^7.50.0",
       "date-fns": "^3.3.0"
     },
     "devDependencies": {
       "@tauri-apps/cli": "^2.0.0",
       "typescript": "^5.3.0",
       "tailwindcss": "^3.4.0",
       "vite": "^5.0.0",
       "vitest": "^1.2.0"
     }
   }
   ```

4. Configure Tauri:
   ```json
   // tauri.conf.json highlights
   {
     "identifier": "com.yourname.resonatify",
     "windows": [{
       "title": "Resonatify",
       "width": 800,
       "height": 600,
       "resizable": true,
       "fullscreen": false
     }],
     "systemTray": {
       "iconPath": "icons/tray-icon.png",
       "menuOnLeftClick": false
     }
   }
   ```

**Deliverables:**
- ✅ Initialized Tauri project with organized structure
  - Created with `npm create tauri-app@latest -- . -t react-ts -m npm --yes --force`
  - React 19.1.0 + TypeScript 5.8.3 + Vite 7.0.4
  - Tauri 2.9.2 (backend), @tauri-apps/api ^2 (frontend)
- ✅ Configuration files set up
  - package.json (name: resonatify)
  - tauri.conf.json (productName: Resonatify, identifier: com.flup.resonatify)
  - Cargo.toml (name: resonatify, lib: resonatify_lib)
- ✅ Project structure configured
  - Frontend: src/ with components/, hooks/, stores/, types/, utils/
  - Backend: src-tauri/src/ with audio/, scheduler/, db/, commands/
  - Tests: tests/ with integration/, e2e/
- ✅ Core dependencies installed
  - State management: zustand ^5.0.8
  - Forms: react-hook-form ^7.66.0
  - Date/time: date-fns ^4.1.0
  - Styling: tailwindcss ^4.1.17 (configured with @import in index.css)
  - Testing: vitest ^4.0.8, @testing-library/react ^16.3.0
- ✅ Rust dependencies added
  - Async runtime: tokio 1.x with full features
  - Date/time: chrono 0.4
  - Audio: rodio 0.19
  - Database: rusqlite 0.32 with bundled feature
- ✅ Project builds and runs successfully
  - Backend: `cargo check` passes (0.18s)
  - Frontend: `npm run build` succeeds (321ms, 195KB bundle)

**Technical Considerations:**
- ✅ Configure Tauri security (CSP, allowlist) - CSP set to null for development
- ✅ Set up icon assets for different platforms - Icons provided by template (icon.png, icon.icns, icon.ico)
- ✅ Configure app identifier for macOS bundle - Set to com.flup.resonatify

**Progress Notes:**
- Successfully created Tauri 2.9.2 project with React 19.1 and TypeScript 5.8
- Configured all necessary directories for Phase 3 development
- Installed all core dependencies from the plan
- Tailwind CSS v4 configured (uses new CSS-first architecture)
- System tray configuration deferred to Phase 3.6 (caused build errors in Phase 2.2)
- Both frontend and backend compile and build successfully
- Ready to begin Phase 3: Core Development

### 2.3 UI Foundation Setup ✅ COMPLETE

**Status**: ✅ Complete
**Completion Date**: 2025-11-13

**Tasks:**
1. ✅ Configure Tailwind CSS:
   ```bash
   npm install -D tailwindcss postcss autoprefixer
   npx tailwindcss init -p
   ```

2. Set up design system:
   - Define color palette (light/dark themes)
   - Set up typography scale
   - Define spacing and layout tokens

3. Install shadcn/ui:
   ```bash
   npx shadcn-ui@latest init
   ```

4. Create base components:
   - Button
   - Input
   - Modal/Dialog
   - Select/Dropdown
   - Switch/Toggle

**Deliverables:**
- ✅ Tailwind CSS v4 configured with custom theme
  - Custom color palette for light/dark modes (Indigo primary, Zinc neutrals)
  - Typography scale and spacing tokens defined
  - Border radius and shadow design tokens
  - CSS variables in @theme blocks for easy theme switching
  - Automatic dark mode support via prefers-color-scheme
- ✅ Base component library created (6 components)
  - Button (with variants: default, destructive, outline, secondary, ghost, link)
  - Input (with focus states and accessibility)
  - Dialog/Modal (with overlay, header, footer, close button)
  - Select/Dropdown (with scroll buttons and item indicators)
  - Switch/Toggle (with smooth transitions)
  - Label (for form accessibility)
- ✅ Design tokens documented
  - Color system: background, foreground, primary, secondary, muted, accent, destructive, success, warning, info
  - Border radius: sm (4px), md (6px), lg (8px), xl (12px)
  - Shadows: sm, md, lg
  - All tokens defined in src/index.css @theme block
- ✅ Component utilities
  - cn() utility function for className merging (clsx + tailwind-merge)
  - Radix UI primitives for accessibility (Dialog, Select, Switch, Label)
  - Class variance authority for component variants
  - Lucide React icons for UI elements
- ✅ TypeScript path alias configured (@/* resolves to ./src/*)
  - Updated tsconfig.json with baseUrl and paths
  - Updated vite.config.ts with alias resolution

**Technical Considerations:**
- ✅ Use CSS variables for theme switching - Implemented with @theme blocks and media queries
- ✅ Ensure components work with macOS native styles - Using system colors and standard UI patterns
- ✅ Test keyboard navigation - Radix UI components have built-in keyboard support

**Progress Notes:**
- Successfully configured Tailwind CSS v4 (new CSS-first architecture)
- Created professional component library following shadcn/ui patterns
- All components use Radix UI primitives for accessibility and keyboard navigation
- Design system supports light/dark modes automatically
- Components are fully typed with TypeScript
- Build verification: Frontend builds in 377ms (22.5KB CSS, 194KB JS)
- Backend check passes in 0.46s
- Ready for Phase 3: Core Development with complete UI foundation

---

## Phase 3: Core Development

**Estimated Duration:** 6-8 weeks
**Complexity:** High

### 3.1 Data Persistence Layer

**Estimated Duration:** 1 week
**Complexity:** Medium

**Status:** ✅ Complete (2025-11-13)

**Summary of Work:**
- Implemented a pooled SQLite connection layer backed by `sqlx`, including application data directory resolution and automatic migration execution (`src-tauri/src/db/mod.rs`).
- Added the initial migration (`001_create_core_tables.sql`) creating the `schedules`, `settings`, and `audio_playback_history` tables with indexes, constraints, and retention trigger.
- Built repository modules for schedules, settings, and playback history with strongly-typed models and JSON-backed repeat configuration.
- Switched to runtime `sqlx` query builders to eliminate the need for `DATABASE_URL` or offline preparation during builds.
- Established unit tests exercising migrations and repository CRUD behaviour using in-memory SQLite databases.

**Completed Tasks:**
1. ✅ Configured pooled SQLite access with automatic migrations (`sqlx`, WAL mode, foreign keys, app data directory handling).
2. ✅ Authored `001_create_core_tables.sql` covering schedules, settings, and playback history tables plus supporting indexes/triggers.
3. ✅ Implemented repository layer for schedules (CRUD, filtering), settings (upsert/delete), and playback history (record/query/cleanup).
4. ✅ Added comprehensive models for repeat logic and playback status serialization.
5. ✅ Wrote async unit tests validating migrations and repository operations using in-memory databases.

**Deliverables:**
- ✅ Database schema with migrations
- ✅ Repository pattern implementation
- ✅ Unit tests for data access layer

**Technical Considerations:**
- UUIDs generated via `uuid` crate for schedule/history identifiers.
- Timestamps stored in UTC using `strftime('%Y-%m-%dT%H:%M:%fZ', 'now')` defaults and `chrono` for runtime writes.
- Migrations executed on startup via `sqlx::migrate!` with graceful directory creation.
- Added indexes on enabled schedules, scheduled_time ordering, and playback history lookups; playback history capped at 1000 entries via trigger.

**Dependencies:** None (foundation layer)

### 3.2 Audio Playback System ✅ COMPLETE

**Status:** ✅ Complete
**Completion Date:** 2025-11-13

**Summary of Work:**
- Built a dedicated rodio-backed audio engine with automatic fade-in/out, per-track volume control, and safe device lifecycle management running on a background worker thread.
- Implemented reusable audio validation covering supported formats (MP3, WAV, FLAC, OGG, M4A/AAC), file size limits, and metadata extraction (duration, channels, sample rate), now including byte-length and extension hints for Symphonia so container detection stays reliable.
- Exposed Tauri commands (`validate_audio_file`, `play_audio_file`, `stop_audio`, `get_audio_status`) that return structured playback state for frontend integration.
- Added comprehensive error types that surface missing devices and validation failures cleanly, plus unit tests for validator and service flows.

**Completed Tasks:**
1. ✅ Implement audio playback engine using rodio with fade control and dedicated worker thread.
2. ✅ Add audio file validation (formats, size, metadata).
3. ✅ Create audio playback service managing state via command channels and shared status.
4. ✅ Expose Tauri commands for frontend integration with typed responses.
5. ✅ Add comprehensive error handling and unit tests for validator/service behaviour.

**Deliverables:**
- ✅ Cross-platform audio playback system (rodio + symphonia codecs)
- ✅ Audio file validation pipeline with metadata reporting
- ✅ Tauri IPC commands for audio control
- ✅ Unit tests covering validation and service workflows

**Technical Considerations:**
- Enabled rodio symphonia codecs and feed decoders with seekable byte-length + extension hints so M4A/AAC alongside MP3/WAV/FLAC/OGG decode consistently.
- Dedicated audio worker thread keeps non-Send rodio types off async executors while exposing async-friendly commands.
- Automatic fade-in/out implemented with short-lived worker threads to avoid abrupt transitions.
- Gracefully handles missing output devices and propagates structured errors to the UI.

**Dependencies:** 3.1 Data Persistence Layer

### 3.3 Scheduling Engine ✅ COMPLETE

**Status:** ✅ Complete (2025-11-13)

**Summary of Work:**
- Built a tokio-driven scheduler engine that loads enabled schedules, spawns cancellable per-schedule tasks, and orchestrates playback through the shared audio service.
- Implemented comprehensive time calculation utilities covering once, daily, weekday/weekend, weekly selections, and custom interval repeat modes with DST-safe conversions and a grace window for near-miss executions.
- Added runtime monitoring with serialized status snapshots, upcoming execution queries, and graceful start/stop/reload flows exposed as new Tauri commands.
- Persisted playback outcomes and automatic disabling of one-shot schedules using the repository layer while capturing history entries for auditability.

**Completed Tasks:**
1. ✅ Designed the scheduling architecture with managed async workers and centralized state tracking.
2. ✅ Implemented next-execution logic and repeat handling for every supported cadence, including custom interval math.
3. ✅ Wired execution loop to invoke audio playback, record success/failure history, and disable once-only schedules after they run.
4. ✅ Added scheduler monitoring endpoints (status, upcoming) plus lifecycle controls (start/stop/reload) surfaced via Tauri commands.
5. ✅ Authored unit tests for time calculations and the scheduler loop using a mock audio controller to validate behaviour without hardware dependencies.

**Deliverables:**
- ✅ Production-ready scheduling engine with cancellable background task management.
- ✅ Time calculation + repeat logic module with DST-aware helpers and grace handling.
- ✅ Scheduler runtime status + upcoming execution reporting pipeline.
- ✅ Tauri IPC commands for scheduler lifecycle and telemetry access.
- ✅ Test suite covering scheduler execution paths and temporal edge cases.

**Technical Considerations:**
- Leveraged `tokio-util` cancellation tokens to stop schedule workers cleanly during reloads and shutdowns.
- Stored runtime metadata in `Arc<RwLock<...>>` snapshots so commands can read status without blocking task progress.
- Applied a one-minute grace period so schedules due moments before startup execute immediately, mitigating sleep/wake drift.
- Recorded playback outcomes (success/failure/skipped) through the history repository for debugging and analytics.

**Dependencies:** 3.1 Data Persistence Layer, 3.2 Audio Playback System

### 3.4 Frontend: Schedule Management UI ✅ COMPLETE (2025-11-14)

**Estimated Duration:** 2 weeks
**Complexity:** Medium-High

**Summary:**
- Built full schedule management experience in React with Tailwind/shadcn components, Zustand store, and Tauri IPC plumbing aligned with backend repositories.
- Implemented CRUD flows wired to new Rust commands plus optimistic toggles, loading/empty/error states, and Cmd+N shortcut for quick creation.
- Add/Edit modal now supports audio file picker, time selector, repeat patterns (including weekly/custom intervals), weekday chips, and volume slider with validation.

**Deliverables:**
- ✅ Zustand schedule store with modal state, optimistic actions, and backend serialization helpers
- ✅ Schedule types/utilities mirroring backend repeat structures
- ✅ Schedule list, header, and card components with enable toggle, edit/delete, and responsive layout
- ✅ Schedule modal with audio selection, time picker, repeat config, weekday selector, volume control, and keyboard shortcut integration
- ✅ Rust schedule commands (get/create/update/delete/toggle) invoking repositories and reloading scheduler

**Technical Notes:**
- Error states surfaced inline with retry affordances; store tracks loading/errors.
- Repeat serialization normalizes snake_case interval_minutes and weekday indices.
- Keyboard shortcut (⌘/Ctrl+N) opens modal globally.
- Scheduler reload triggered after each mutating IPC command to keep engine in sync.
- Confirmation dialog for delete remains planned for later UX iteration.

**Dependencies:**
- 3.1 Data Persistence Layer
- 3.3 Scheduling Engine

### 3.5 Frontend: Settings UI ✅ COMPLETE (2025-11-14)

**Summary:**
- Delivered a full settings experience backed by new Tauri settings commands and a dedicated Zustand store with optimistic updates and persistence.
- Built a tabbed panel with sections for General, Notifications, Audio, About, and Support, each using shadcn primitives plus reusable toggle/slider components; wired donation and GitHub links via the opener plugin.
- Added theme switching (light/dark/system) with runtime CSS variable overrides and a `useThemeSync` hook so user preferences immediately update the UI.

**Deliverables:**
- ✅ Settings store with ensure/fetch/update actions and launch-at-login plumbing
- ✅ Settings UI (General/Notifications/Audio/About/Support) with Tailwind/shadcn components & toast-friendly error states
- ✅ Theme synchronization hook + data-theme overrides for smooth transitions
- ✅ Backend settings IPC commands persisting to SQLite repository

**Technical Notes:**
- SettingsSnapshot type aggregates DB rows ensuring default fallbacks when keys are missing.
- Frontend layout component handles nav + loading/error UI and reuses Radix inputs.
- Theme override relies on `document.documentElement.dataset.theme` to bypass prefers-color-scheme when users force modes.

**Dependencies:** 3.1 Data Persistence Layer

### 3.6 System Tray Integration ✅ COMPLETE (2025-11-15)

**Status:** ✅ Complete

**Summary:**
- Implemented full system tray integration with menu, window management, and tray icon functionality
- Added pause/resume controls and window hide/show behavior
- Created tray module with menu event handlers and window close interception

**Completed Tasks:**
1. ✅ Created system tray menu with Show, Pause All, Resume All, and Quit actions
2. ✅ Implemented dynamic tray icon states (framework in place, icons use default for now)
3. ✅ Handled window hide/show (hides to tray on close instead of quitting)
4. ✅ Added tooltip infrastructure (static for now, dynamic updates planned for future)
5. ✅ Integrated with existing scheduler engine (pause_all/resume_all methods added)

**Deliverables:**
- ✅ System tray module with menu event handlers (src-tauri/src/tray.rs)
- ✅ Tray menu with Show, Pause/Resume All, and Quit actions
- ✅ Window close handler (hides to tray instead of quitting)
- ✅ Scheduler engine pause/resume functionality
- ✅ Left-click tray icon toggles window visibility
- ✅ Graceful quit from tray menu

**Technical Considerations:**
- ✅ Enabled tray-icon feature in Tauri Cargo.toml
- ✅ Used Tauri's default icons (custom monochrome icons for later)
- ⏳ Dynamic tooltip updates deferred (requires tray icon state management)
- ✅ Integrated with existing scheduler and audio services

**Dependencies:** 3.3 Scheduling Engine

**Notes:**
- Tray icons currently use Tauri's default app icon
- Custom monochrome template icons for macOS can be added in future iterations
- Dynamic tooltip updates with upcoming schedule info planned for future enhancement
- All menu actions are functional and integrated with the scheduler engine

---

## Phase 3.7: UI Polish Pass ✅ COMPLETE (2025-11-15)

**Status:** ✅ Complete
**Estimated Duration:** 1-2 days
**Complexity:** Low
**Completion Date:** 2025-11-15

**Summary:**
Light polish pass applied before Phase 4 to improve visual design, user experience, and overall UI consistency based on wireframes and UI/UX inspiration docs.

**Completed Tasks:**
1. ✅ Enhanced empty state design
   - Added icon with gradient background
   - Improved messaging and added example text
   - Better visual hierarchy and spacing
2. ✅ Improved loading and error states
   - Animated spinner for loading
   - Icon and better messaging for errors
   - Retry button with improved styling
3. ✅ Polished schedule cards
   - Better typography hierarchy (larger time display)
   - Improved spacing and padding (p-5 instead of p-4)
   - Smooth hover effects with transition-all
   - Testing indicator with animated pulse effect
   - Responsive button layouts (hide Delete text on small screens)
   - Visual ring effect when testing audio
4. ✅ Enhanced app header
   - Logo icon with gradient background
   - Sticky header with backdrop blur effect
   - Improved tab switcher styling
   - Better spacing and visual hierarchy
5. ✅ Improved schedule header
   - Better typography (bold instead of semibold)
   - Keyboard shortcut badge (⌘N) with proper kbd styling
   - Larger, more prominent action button
   - Improved count display with singular/plural handling
6. ✅ Enhanced modal styling
   - Better spacing (gap-6 instead of space-y-6)
   - Larger dialog title (text-2xl)
   - Improved form layout (space-y-5)
   - Better visual hierarchy
7. ✅ Added global design improvements
   - Smooth color transitions on all elements
   - Custom scrollbar styling
   - Focus-visible styling for accessibility
   - Antialiased text rendering
   - Custom animations (slide-in, fade-in, scale-in)

**Deliverables:**
- ✅ Polished UI components following design system
- ✅ Improved empty, loading, and error states
- ✅ Enhanced visual feedback and hover states
- ✅ Better typography hierarchy throughout
- ✅ Smooth transitions and animations
- ✅ Consistent spacing and layout

**Technical Considerations:**
- ✅ All changes follow design system tokens from wireframes
- ✅ Improved accessibility with focus-visible styling
- ✅ Responsive design maintained
- ✅ Performance not impacted (build time: 857ms)
- ✅ Both frontend and backend verified to compile

**Files Modified:**
- src/App.tsx - Enhanced header with logo and sticky positioning
- src/components/ScheduleList.tsx - Improved empty, loading, error states
- src/components/ScheduleCard.tsx - Better card design and hover effects
- src/components/ScheduleHeader.tsx - Polished header with kbd element
- src/components/ScheduleModal.tsx - Enhanced modal spacing and typography
- src/index.css - Added global transitions, animations, and custom scrollbar

**Dependencies:** 3.1-3.6 (Core Development phases)

**Key Design Improvements:**
- Empty state now includes icon, gradient, and example text (from wireframes.md)
- Cards have better visual hierarchy with larger time display
- Smooth transitions create more polished feel
- Keyboard shortcut badges use semantic kbd element
- Testing indicator with pulse animation provides clear feedback
- Custom scrollbar matches app design system

---

## Phase 4: macOS Specific Implementation

**Estimated Duration:** 1 week
**Complexity:** Medium

### 4.1 Launch at Login ✅ COMPLETE

**Status:** ✅ Complete
**Completion Date:** 2025-11-15

**Summary:**
- Implemented full launch at login functionality using tauri-plugin-autostart
- Configured macOS LaunchAgent with minimized flag
- Updated set_launch_at_login command to enable/disable system autostart
- Settings UI already integrated and functional

**Completed Tasks:**
1. ✅ Add tauri-plugin-autostart dependency to Cargo.toml
2. ✅ Configure plugin for macOS LaunchAgent in lib.rs
3. ✅ Update set_launch_at_login command to enable/disable autostart
4. ✅ Verify integration with existing settings UI

**Deliverables:**
- ✅ Launch at login functionality with macOS LaunchAgent
- ✅ Settings integration via toggleLaunchAtLogin in settingsStore
- ✅ Backend command properly enables/disables autostart and persists to database

**Technical Considerations:**
- ✅ Respects user preferences (setting persisted to SQLite)
- ✅ Uses macOS LaunchAgent for proper system integration
- ✅ App launches minimized when auto-started (--minimized flag)
- ⏳ Testing on different macOS versions (12+) - to be done in manual testing phase

**Dependencies:** 3.5 Settings UI

### 4.2 Notifications ✅ COMPLETE

**Status:** ✅ Complete
**Completion Date:** 2025-11-15

**Summary:**
- Implemented native macOS notifications using tauri-plugin-notification
- Integrated notifications with scheduler engine to send alerts on schedule execution
- Added settings integration to respect user notification preferences
- Notifications automatically respect macOS Do Not Disturb mode

**Completed Tasks:**
1. ✅ Add tauri-plugin-notification dependency to Cargo.toml
2. ✅ Configure notification plugin in lib.rs
3. ✅ Send notifications for schedule execution (success and failure)
4. ✅ Integrate with scheduler engine via AppHandle
5. ✅ Respect settings (show_notifications) before sending

**Deliverables:**
- ✅ Native macOS notifications via tauri-plugin-notification
- ✅ Permission handling (automatic via Tauri plugin)
- ✅ Integration with scheduler engine
- ✅ Success notifications: "Schedule Executed" with schedule name
- ✅ Failure notifications: "Schedule Failed" with error details
- ✅ Settings-aware notification system

**Technical Considerations:**
- ✅ Notifications respect user preferences (show_notifications setting)
- ✅ macOS automatically handles Do Not Disturb mode (via native notifications)
- ✅ Graceful fallback when app handle not available (tests)
- ✅ Updated SchedulerEngine to accept AppHandle for notification access
- ⏳ Testing on different macOS versions (12+) - to be done in manual testing phase

**Implementation Details:**
- Added send_notification helper function that checks settings before sending
- Modified SchedulerEngine to include optional AppHandle
- Notifications sent on successful playback (line 479-484 in engine.rs)
- Notifications sent on failed playback (line 498-504 in engine.rs)
- Updated tests to use new SchedulerEngine::with_audio_controller signature

**Dependencies:** 3.3 Scheduling Engine, 3.5 Settings UI

### 4.3 Code Signing & Notarization Setup

**Tasks:**
1. Set up Apple Developer account ($99/year)
2. Create App ID and certificates (Developer ID Application)
3. Configure code signing in Tauri
4. Create entitlements file
5. Set up notarization credentials
6. Create build script for signing and notarization

**Deliverables:**
- Code signing setup
- Notarization pipeline
- Build scripts

**Technical Considerations:**
- Universal binary (Intel + Apple Silicon)
- Hardened runtime required for notarization
- Test on both Intel and Apple Silicon Macs
- Keep certificates secure

---

## Phase 5: Testing

**Estimated Duration:** 1-2 weeks
**Complexity:** Medium

> **Policy Update:** Beginning with this phase (and retroactively enforced), **every code change must include or update unit tests** covering the affected logic. Work items are not considered complete until corresponding tests are merged and passing locally/CI.

### 5.1 Unit Testing

**Tasks:**
1. Backend unit tests:
   - Time calculation logic
   - Repeat type logic
   - Database operations
   - Audio validation
2. Frontend unit tests:
   - Utility functions
   - Component tests
   - Store tests

**Deliverables:**
- Unit tests for all core modules
- >80% code coverage for critical paths
- Test documentation

**Technical Considerations:**
- Mock external dependencies
- Test edge cases (time zones, DST)
- Test error handling

### 5.2 Integration Testing

**Tasks:**
1. Test schedule creation and execution flow
2. Test database persistence
3. Test frontend-backend integration with Playwright
4. Test audio playback integration

**Deliverables:**
- Integration tests for critical flows
- End-to-end test suite
- CI/CD pipeline integration

**Technical Considerations:**
- Use test databases
- Clean up test data
- Test with real audio files
- Simulate time for testing schedules

### 5.3 Manual Testing

**Tasks:**
1. Create comprehensive testing checklist:
   - Installation on fresh macOS system
   - Permission granting
   - All CRUD operations
   - Schedule execution verification
   - Audio playback with various formats
   - System tray interactions
   - Notifications
   - Theme switching
   - App restart (persistence)
   - System sleep/wake
   - Time zone changes
   - Edge cases (midnight, DST)

2. Performance testing:
   - Memory usage over 24 hours
   - CPU usage during idle and playback
   - App startup time
   - Responsiveness with 50+ schedules

3. Compatibility testing:
   - macOS 12-15 (Monterey through Sequoia)
   - Intel and Apple Silicon Macs

-**Deliverables:**
- Testing checklist completed
- Bug reports and fixes
- Performance benchmarks
- Compatibility matrix

---

## Phase 6: Deployment & Distribution

**Estimated Duration:** 1 week
**Complexity:** Medium

### 6.1 Build Pipeline

**Tasks:**
1. Set up GitHub Actions for automated builds
2. Configure certificate import in CI
3. Create release script for version management
4. Test build process on clean environment

**Deliverables:**
- Automated build pipeline
- Release scripts
- Version management system

**Technical Considerations:**
- Store certificates and keys securely
- Semantic versioning
- Universal binary support

### 6.2 Distribution

**Tasks:**
1. Create landing page with:
   - Product description
   - Screenshots
   - Features list
   - Download button
   - Installation instructions
   - System requirements

2. Write documentation:
   - Getting started guide
   - User manual
   - FAQ
   - Troubleshooting
   - Changelog

3. Set up distribution channels:
   - Direct download (DMG from website)
   - GitHub Releases
   - Consider Homebrew cask (later)

4. Implement auto-update mechanism using tauri-plugin-updater

5. Add update UI in app

**Deliverables:**
- Landing page
- Documentation site
- Distribution setup
- Auto-update functionality

**Technical Considerations:**
- Host update manifests
- Incremental updates (delta updates)
- Rollback mechanism

### 6.3 Analytics & Crash Reporting

**Tasks:**
1. Add crash reporting (Sentry, opt-in)
2. Add privacy-respecting analytics (self-hosted Plausible)
3. Create privacy policy
4. Implement user consent flow

**Deliverables:**
- Crash reporting setup
- Privacy policy
- Analytics dashboard

**Technical Considerations:**
- GDPR compliance
- User consent
- Anonymize data
- Clear privacy policy

---

## Phase 7: Future Features Planning

**Estimated Duration:** Ongoing
**Complexity:** Varies

### 7.1 Windows & Linux Support

**Tasks:**
1. Adapt platform-specific code:
   - Launch at startup (different per platform)
   - Notifications (different APIs)
   - Test audio playback cross-platform
2. Build pipelines for Windows and Linux
3. Cross-platform testing

**Deliverables:**
- Windows build (NSIS installer)
- Linux builds (.deb, .AppImage)
- Cross-platform testing

**Technical Considerations:**
- Test on Windows 10/11
- Test on Ubuntu, Fedora, Arch
- Handle platform-specific bugs

### 7.2 Expanded Actions

**Tasks:**
1. Define action interface/trait system
2. Implement new action types:
   - Open Application
   - Display Message
   - Run Script
   - Send Notification
   - Open URL
3. Update UI for action selection and configuration

**Deliverables:**
- Action framework
- 5+ action types
- UI for action configuration

**Technical Considerations:**
- Security (script execution)
- Error handling per action type
- Action chaining (sequences)

### 7.3 In-App Audio Recording

**Tasks:**
1. Add audio recording capability using cpal
2. Build recording UI with:
   - Start/stop recording
   - Duration timer
   - Waveform visualization
3. Request microphone permissions
4. Audio file management

**Deliverables:**
- Audio recording functionality
- Recording UI
- Audio file management

**Technical Considerations:**
- Microphone permissions
- Support multiple formats
- Edit/trim capabilities

### 7.4 AI Integration

**Tasks:**
1. Define AI action types (API calls to Perplexity, OpenAI, etc.)
2. Implement API integration
3. Create AI action configuration UI
4. Add response handling (display, speak, log)
5. Implement rate limiting and error handling

**Deliverables:**
- AI API integration
- AI action type
- Configuration UI

**Technical Considerations:**
- API key management (secure storage)
- Rate limiting
- Cost management
- Privacy considerations

---

## Monetization Strategy

### PayPal Donation Integration

**Tasks:**
1. Create PayPal business account
2. Generate donation button/link
3. Add donation button to:
   - Settings page
   - About section
   - Website
4. Create thank you message/notification

**Technical Considerations:**
- Clear messaging (optional donation)
- No feature gating
- Privacy (no tracking of donors)
- Consider other platforms (Buy Me a Coffee, GitHub Sponsors)

---

## Project Timeline Summary

| Phase | Duration | Start Week | End Week | Status |
|-------|----------|------------|----------|--------|
| Research & Planning | 1-2 weeks | Week 1 | Week 2 | ✅ Complete (1.1 ✅, 1.2 ✅, 1.3 ✅) |
| Project Setup | 3-5 days | Week 2 | Week 3 | ✅ Complete (2.1 ✅, 2.2 ✅, 2.3 ✅) |
| Core Development | 6-8 weeks | Week 3 | Week 10 | ✅ Complete (3.1-3.7 ✅) |
| macOS Implementation | 1 week | Week 10 | Week 11 | 🔄 In Progress (4.1 ✅, 4.2 ✅) |
| Testing | 1-2 weeks | Week 11 | Week 13 | ⚪ Not Started |
| Deployment | 1 week | Week 13 | Week 14 | ⚪ Not Started |
| **Total (MVP)** | **~10-14 weeks** | - | - | **📅 Est. Q2 2025** |
| Future Features | Ongoing | Week 14+ | - | ⚪ Not Started |

**Project Started:** 2025-11-13
**Last Updated:** 2025-11-15
**Phase 1 Completed:** 2025-11-13 🎉
**Phase 2 Started:** 2025-11-13
**Phase 2 Completed:** 2025-11-13 🎉
**Phase 3 Started:** 2025-11-13
**Phase 3 Completed (including UI Polish):** 2025-11-15 🎉
**Phase 4 Started:** 2025-11-15

**Phase 1 Progress:** 3/3 complete (100%)
- ✅ 1.1 Market Research & Competitive Analysis
- ✅ 1.2 User Experience Design
- ✅ 1.3 Architecture Design

**Phase 2 Progress:** 3/3 tasks complete (100%) 🎉
- ✅ 2.1 Development Environment Setup (Complete: 2025-11-13)
  - ✅ Step 1: Install development tools (Rust 1.91.1, Cargo 1.91.1, Tauri CLI 2.9.4)
  - ✅ Step 2: Configure IDE (VSCode settings, extensions, ESLint, Prettier)
  - ✅ Step 3: Set up version control (Git, GitHub, branching strategy)
- ✅ 2.2 Initialize Tauri Project (Complete: 2025-11-13)
  - ✅ Created Tauri 2.9.2 project with React 19.1 + TypeScript 5.8
  - ✅ Configured directory structure (frontend, backend, tests)
  - ✅ Installed all core dependencies (zustand, react-hook-form, date-fns, tailwindcss)
  - ✅ Added Rust dependencies (tokio, chrono, rodio, rusqlite)
  - ✅ Configured Tauri settings (identifier: com.flup.resonatify)
  - ✅ Verified builds: Backend (cargo check) and Frontend (npm build)
- ✅ 2.3 UI Foundation Setup (Complete: 2025-11-13)
  - ✅ Configured Tailwind CSS v4 with custom theme and design system
  - ✅ Created 6 base UI components (Button, Input, Dialog, Select, Switch, Label)
  - ✅ Installed Radix UI primitives for accessibility
  - ✅ Set up TypeScript path aliases (@/*)
  - ✅ Verified builds: Frontend (377ms) and Backend (0.46s)

**Phase 3 Progress:** 7/7 tasks complete (100%) 🎉
- ✅ 3.1 Data Persistence Layer — SQLite migrations, repositories, and tests in place
- ✅ 3.2 Audio Playback System — rodio engine, validation, and Tauri commands implemented
- ✅ 3.3 Scheduling Engine — async scheduler core, time calculators, commands, and tests delivered
- ✅ 3.4 Schedule Management UI — Complete (2025-11-14)
- ✅ 3.5 Settings UI — Complete (2025-11-14)
- ✅ 3.6 System Tray Integration — Complete (2025-11-15)
- ✅ 3.7 UI Polish Pass — Complete (2025-11-15) ✨

**Phase 4 Progress:** 2/3 tasks complete (67%)
- ✅ 4.1 Launch at Login — Complete (2025-11-15)
- ✅ 4.2 Notifications — Complete (2025-11-15)
- ⚪ 4.3 Code Signing & Notarization Setup — Not Started

---

## Risk Assessment & Mitigation

### Technical Risks

1. **Audio Playback Issues**
   - Risk: Cross-platform audio playback inconsistencies
   - Mitigation: Test early on target platforms, use well-maintained libraries (rodio)

2. **Scheduling Precision**
   - Risk: Schedules not executing at exact times
   - Mitigation: Account for system sleep/wake, test extensively, implement catch-up logic

3. **macOS Permissions**
   - Risk: Users denying critical permissions
   - Mitigation: Clear permission explanations, graceful degradation, helpful error messages

4. **Code Signing Issues**
   - Risk: App won't run on users' machines without proper signing
   - Mitigation: Set up signing/notarization early, test on multiple Macs

### Project Risks

1. **Scope Creep**
   - Risk: Adding too many features delays launch
   - Mitigation: Strict MVP definition, defer features to post-launch

2. **Time Estimation**
   - Risk: Tasks take longer than estimated
   - Mitigation: Add 20% buffer, prioritize ruthlessly, iterate

3. **User Adoption**
   - Risk: App doesn't meet user needs
   - Mitigation: User testing during development, gather feedback early

---

## Success Metrics

### Technical Metrics
- App starts in <1 second
- Memory usage <50MB idle
- CPU usage <5% idle, <20% during playback
- 99.9% schedule execution accuracy
- Zero critical bugs in production

### User Metrics
- 100+ downloads in first month
- 4.5+ star rating (if on app store later)
- <5% crash rate
- Positive user feedback

### Business Metrics
- Sustainable donation rate (goal: $50/month by month 3)
- Active user retention (50% after 30 days)

---

## Next Steps

1. **Immediate Actions:**
   - Review and approve this plan
   - Set up development environment
   - Create GitHub repository
   - Set up project management tool (GitHub Projects, Trello, etc.)

2. **Week 1 Goals:**
   - Complete competitive analysis
   - Create wireframes
   - Design architecture
   - Set up development environment
   - Initialize Tauri project

3. **Regular Reviews:**
   - Weekly progress reviews
   - Bi-weekly feature demos
   - Monthly milestone assessments

---

## Resources & References

### Documentation
- Tauri Documentation: https://tauri.app/
- Rust Book: https://doc.rust-lang.org/book/
- React Documentation: https://react.dev/

### Libraries
- rodio (audio): https://github.com/RustAudio/rodio
- chrono (time): https://github.com/chronotope/chrono
- tokio (async): https://tokio.rs/

### Design Resources
- macOS Human Interface Guidelines: https://developer.apple.com/design/human-interface-guidelines/macos
- shadcn/ui: https://ui.shadcn.com/

### Distribution
- Apple Developer Program: https://developer.apple.com/programs/
- Code Signing Guide: https://developer.apple.com/support/code-signing/

---

## Conclusion

This comprehensive plan provides a clear roadmap for developing a cross-platform audio scheduling application. By following this phased approach, focusing on the macOS MVP first, and maintaining high code quality standards, the project is well-positioned for success.

The modular architecture and cross-platform foundation (Tauri) will enable smooth expansion to Windows and Linux in future releases. The plan balances ambition with pragmatism, ensuring a solid foundation while keeping the door open for exciting future enhancements like AI integration and expanded action types.

**Estimated Total Development Time:** 10-14 weeks for macOS MVP
**Estimated Cost:** Developer time + $99/year Apple Developer Program
**Target Launch:** Q2 2025 (adjust based on start date)

Good luck with the development! 🚀
