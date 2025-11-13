# Resonator

> Schedule audio files to play at specific times. Simple, reliable, beautiful.

![Status](https://img.shields.io/badge/status-planning-blue)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey)
![License](https://img.shields.io/badge/license-MIT-green)

**Resonator** is a cross-platform desktop application that allows you to schedule audio files to play at specific times. Perfect for meditation reminders, work breaks, practice sessions, and building audio-based habits.

---

## 🎯 Project Status

**Current Phase**: Planning & Design (Phase 1 Complete ✅)
- ✅ Market Research & Competitive Analysis
- ✅ User Experience Design
- ✅ Architecture Design
- ⏳ **Next**: Phase 2 - Project Setup

**Target Launch**: Q2 2025

---

## ✨ Features (Planned)

### Core Features (MVP)
- 🎵 Schedule any audio file (MP3, WAV, FLAC, M4A, OGG)
- ⏰ Flexible repeat patterns (once, daily, weekdays, weekends, custom)
- 📱 Native macOS menu bar integration
- 🔔 System notifications
- 🎨 Beautiful, minimal interface
- 🚀 Launch at login
- 🔊 Per-schedule volume control
- ⚙️ Light/dark theme support

### Future Features
- 🪟 Windows and Linux support
- 🎙️ In-app audio recording
- 🤖 AI-powered scheduling
- 📝 Expanded actions (open apps, run scripts)

---

## 🏗️ Technology Stack

**Frontend:**
- React 18+ with TypeScript
- Vite (build tool)
- Tailwind CSS + shadcn/ui
- Zustand (state management)

**Backend:**
- Rust with Tauri 2.0
- Tokio (async runtime)
- SQLite with sqlx
- Rodio (audio playback)

**Why Tauri?**
- 🚀 Smaller bundle size (~3MB vs Electron's 100MB+)
- ⚡ Faster startup (~0.4s vs 1.5s)
- 🔒 Better security (Rust + minimal attack surface)
- 🌍 Cross-platform ready

---

## 📚 Documentation

Comprehensive documentation is available in the [`docs/`](./docs) directory:

### Planning & Design
- [**App Plan Details**](./docs/app-plan-details.md) - Complete development plan
- [**Competitive Analysis**](./docs/competitive-analysis.md) - Market research findings
- [**User Personas**](./docs/user-personas.md) - Target user profiles
- [**User Journey Maps**](./docs/user-journey-maps.md) - User experience flows
- [**Wireframes**](./docs/wireframes.md) - UI/UX wireframes
- [**Architecture Design**](./docs/architecture-design.md) - Technical architecture

### Development
- [**CLAUDE.md**](./CLAUDE.md) - Quick reference for AI assistants
- [**BRANDING.md**](./BRANDING.md) - Brand identity guidelines

---

## 🚀 Getting Started

> **Note**: Project is in planning phase. Setup instructions will be added when development begins.

### Prerequisites (Coming Soon)
- Rust (latest stable)
- Node.js 18+
- macOS 12+ (for initial development)

### Development Setup (Coming Soon)
```bash
# Clone the repository
git clone https://github.com/flup-repo/resonator.git
cd resonator

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

---

## 🗺️ Roadmap

### Phase 1: Research & Planning ✅ (Complete)
- Market research and competitive analysis
- User experience design
- Architecture design

### Phase 2: Project Setup ⏳ (In Progress)
- Development environment setup
- Initialize Tauri project
- UI foundation (Tailwind + shadcn/ui)

### Phase 3: Core Development 📅 (Weeks 3-10)
- Data persistence layer
- Audio playback system
- Scheduling engine
- Frontend UI components

### Phase 4: macOS Implementation 📅 (Week 10-11)
- Launch at login
- Notifications
- Code signing & notarization

### Phase 5: Testing 📅 (Weeks 11-13)
- Unit testing
- Integration testing
- Manual QA

### Phase 6: Deployment 📅 (Week 13-14)
- Build pipeline
- Distribution setup
- Auto-update system

---

## 🤝 Contributing

> **Note**: Contribution guidelines will be added once development begins.

Resonator is open source and community-driven. We welcome contributions from:
- 🐛 Bug reports and feature requests
- 💻 Code contributions
- 📖 Documentation improvements
- 🎨 Design suggestions

---

## 💖 Support

Resonator is free and open source. If you find it useful, consider supporting development:

- ☕ [Buy Me a Coffee](https://buymeacoffee.com/resonator) (coming soon)
- 💙 [PayPal Donation](https://paypal.me/resonator) (coming soon)
- ⭐ Star this repository
- 🐦 Share on social media

---

## 📄 License

MIT License - see [LICENSE](./LICENSE) file for details.

---

## 🙏 Acknowledgments

Built with:
- [Tauri](https://tauri.app/) - Build smaller, faster, and more secure desktop applications
- [React](https://react.dev/) - The library for web and native user interfaces
- [Rust](https://www.rust-lang.org/) - A language empowering everyone to build reliable and efficient software
- [Rodio](https://github.com/RustAudio/rodio) - Rust audio playback library

Inspired by the need for simple, reliable audio scheduling.

---

## 📬 Contact

- **GitHub**: [@flup-repo](https://github.com/flup-repo)
- **Issues**: [Report a bug or request a feature](https://github.com/flup-repo/resonator/issues)

---

<p align="center">
  <b>Resonator</b> - Audio that resonates with your routine
</p>

<p align="center">
  Made with ❤️ for the macOS community
</p>
