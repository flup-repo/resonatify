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
- Implemented reusable audio validation covering supported formats (MP3, WAV, FLAC, OGG, M4A/AAC), file size limits, and metadata extraction (duration, channels, sample rate).
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
- Enabled rodio symphonia codecs to support M4A/AAC alongside MP3/WAV/FLAC/OGG.
- Dedicated audio worker thread keeps non-Send rodio types off async executors while exposing async-friendly commands.
- Automatic fade-in/out implemented with short-lived worker threads to avoid abrupt transitions.
- Gracefully handles missing output devices and propagates structured errors to the UI.

**Dependencies:** 3.1 Data Persistence Layer

### 3.3 Scheduling Engine

**Estimated Duration:** 2 weeks
**Complexity:** High

**Tasks:**
1. Design scheduling architecture
2. Implement time calculation logic (next execution time, should execute now)
3. Implement repeat logic (once, daily, weekly, weekdays, weekends, custom)
4. Implement scheduler engine core:
   - Start/stop engine
   - Add/remove/update schedules dynamically
   - Background task management using tokio
5. Implement execution logic (play audio at scheduled time)
6. Add scheduler monitoring (status, upcoming executions)
7. Expose Tauri commands
8. Write comprehensive tests (time zones, DST, edge cases)

**Deliverables:**
- Fully functional scheduling engine
- Time calculation and repeat logic
- Background task management
- Comprehensive test suite

**Technical Considerations:**
- Use tokio::time::sleep for async delays
- Handle system sleep/wake (macOS power events)
- Persist schedule state to survive app restarts
- Consider missed schedules (if app was closed)
- Handle time zone changes
- Implement graceful shutdown
- Add logging for debugging

**Dependencies:**
- 3.1 Data Persistence Layer
- 3.2 Audio Playback System

### 3.4 Frontend: Schedule Management UI

**Estimated Duration:** 2 weeks
**Complexity:** Medium-High

**Tasks:**
1. Create state management store (Zustand)
2. Create type definitions
3. Build main schedule list component
4. Build schedule card component
5. Build add/edit schedule modal with:
   - Audio file selection
   - Time picker
   - Repeat configuration
   - Weekday selector for custom repeats
   - Volume control
6. Build header with actions
7. Implement Tauri commands (CRUD operations)

**Deliverables:**
- Complete schedule management UI
- Add/edit/delete functionality
- Audio file selection
- Repeat configuration
- Volume control
- Responsive design

**Technical Considerations:**
- Use optimistic updates for better UX
- Implement proper error handling and user feedback
- Add loading states for async operations
- Validate audio file selection
- Show helpful error messages
- Implement keyboard shortcuts (Cmd+N for new schedule)
- Add confirmation dialogs for destructive actions

**Dependencies:**
- 3.1 Data Persistence Layer
- 3.3 Scheduling Engine

### 3.5 Frontend: Settings UI

**Estimated Duration:** 1 week
**Complexity:** Medium

**Tasks:**
1. Create settings store with persistence
2. Build settings page with sections:
   - General (theme, launch at login, minimize to tray)
   - Notifications (show notifications, notification sound)
   - Audio (default volume)
   - About (version info, GitHub link)
   - Support (donation button)
3. Build reusable settings components
4. Implement theme switching (light/dark/system)
5. Implement Tauri commands for settings

**Deliverables:**
- Complete settings UI
- Theme switching
- Launch at login configuration
- Notification preferences
- Audio settings

**Technical Considerations:**
- Persist settings to database
- Sync with Tauri backend
- Respect system theme preferences
- Implement smooth theme transitions
- Validate settings before saving

**Dependencies:** 3.1 Data Persistence Layer

### 3.6 System Tray Integration

**Estimated Duration:** 3-4 days
**Complexity:** Medium

**Tasks:**
1. Create system tray menu (show, upcoming schedules, pause all, quit)
2. Implement dynamic tray icon (active/inactive states)
3. Handle window hide/show (hide to tray on close)
4. Update tray tooltip with next schedule
5. Create tray icon assets (normal and active states)

**Deliverables:**
- System tray with menu
- Dynamic tray icon
- Hide to tray functionality
- Context menu with actions

**Technical Considerations:**
- Use monochrome icons for macOS (template mode)
- Handle retina displays (2x, 3x assets)
- Update tray state in real-time
- Graceful shutdown when quit from tray

**Dependencies:** 3.3 Scheduling Engine

---

## Phase 4: macOS Specific Implementation

**Estimated Duration:** 1 week
**Complexity:** Medium

### 4.1 Launch at Login

**Tasks:**
1. Add tauri-plugin-autostart dependency
2. Configure plugin for macOS LaunchAgent
3. Create helper commands (enable/disable/check autostart)
4. Integrate with settings UI

**Deliverables:**
- Launch at login functionality
- Settings integration

**Technical Considerations:**
- Respect user preferences
- Handle permission requests gracefully
- Test on different macOS versions (12+)

### 4.2 Notifications

**Tasks:**
1. Add tauri-plugin-notification
2. Request notification permissions
3. Send notifications for schedule execution and failures
4. Integrate with scheduler engine
5. Respect settings (show notifications, notification sound)

**Deliverables:**
- Native macOS notifications
- Permission handling
- Integration with scheduler

**Technical Considerations:**
- Handle notification permissions gracefully
- Respect Do Not Disturb mode
- Test on different macOS versions

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

**Deliverables:**
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
| Core Development | 6-8 weeks | Week 3 | Week 10 | 🟡 In Progress (3.1 ✅) |
| macOS Implementation | 1 week | Week 10 | Week 11 | ⚪ Not Started |
| Testing | 1-2 weeks | Week 11 | Week 13 | ⚪ Not Started |
| Deployment | 1 week | Week 13 | Week 14 | ⚪ Not Started |
| **Total (MVP)** | **~10-14 weeks** | - | - | **📅 Est. Q2 2025** |
| Future Features | Ongoing | Week 14+ | - | ⚪ Not Started |

**Project Started:** 2025-11-13
**Last Updated:** 2025-11-13
**Phase 1 Completed:** 2025-11-13 🎉
**Phase 2 Started:** 2025-11-13
**Phase 2 Completed:** 2025-11-13 🎉

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

**Phase 3 Progress:** 2/6 tasks complete (33%)
- ✅ 3.1 Data Persistence Layer — SQLite migrations, repositories, and tests in place
- ✅ 3.2 Audio Playback System — rodio engine, validation, and Tauri commands implemented
- ⏳ 3.3 Scheduling Engine — pending persistence-driven scheduling logic
- ⏳ 3.4 Schedule Management UI — pending CRUD integration
- ⏳ 3.5 Settings UI — pending persistence wiring
- ⏳ 3.6 System Tray Integration — pending scheduler engine

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
