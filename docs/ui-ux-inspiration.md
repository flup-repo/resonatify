# UI/UX Inspiration & Design Patterns

**Date**: 2025-11-13
**Phase**: 1.1 Market Research & Competitive Analysis
**Status**: Complete

---

## Overview

This document captures UI/UX patterns, design inspiration, and interface guidelines discovered during competitive research. These insights will inform the wireframing and design phases.

---

## 1. Menu Bar Integration

### 1.1 Icon Design Patterns

**Observed Best Practices:**
- **Style**: Monochrome template images (SF Symbols style)
- **Size**: 16x16pt @1x, with @2x and @3x variants
- **States**:
  - Inactive/Idle: Simple icon
  - Active/Running: Slightly different (filled, or with indicator dot)
  - Error: Red tint or warning indicator
- **Examples**:
  - Timer apps: Clock or stopwatch icon
  - Alarm apps: Bell icon
  - Scheduler apps: Calendar icon with clock

**Recommendation for Resonatify:**
- Primary icon: Waveform + clock combination
- Active state: Filled waveform
- Idle state: Outline waveform
- Template mode for macOS native appearance

### 1.2 Menu Structure

**Typical Menu Organization:**
```
┌─────────────────────────────┐
│ 🎵 Resonatify          │
├─────────────────────────────┤
│ Next: Meditation in 15m     │ ← Dynamic next schedule
├─────────────────────────────┤
│ 📱 Show Window              │
│ ⏸️  Pause All Schedules      │
│ ⏯️  Resume All Schedules     │
├─────────────────────────────┤
│ ⚙️  Settings...              │
│ ℹ️  About                    │
├─────────────────────────────┤
│ ❌ Quit Resonatify      │
└─────────────────────────────┘
```

**Key Patterns:**
1. App name at top
2. Dynamic status/next schedule (changes in real-time)
3. Primary action (Show Window)
4. Quick actions (Pause/Resume)
5. Separator
6. Settings/About
7. Separator
8. Quit (with keyboard shortcut Cmd+Q)

**Interactions:**
- Left-click icon: Show main window (or toggle)
- Right-click icon: Show menu
- Tooltip on hover: Next scheduled item

---

## 2. Main Window Design

### 2.1 Layout Structure

**Popular Layout Pattern:**
```
┌────────────────────────────────────────────┐
│  Resonatify              [- □ ×]     │ ← Header
├────────────────────────────────────────────┤
│  [+ New Schedule]  [Settings]  [Refresh]  │ ← Action Bar
├────────────────────────────────────────────┤
│  ┌──────────────────────────────────────┐ │
│  │  🎵 Morning Meditation               │ │
│  │  ⏰ 07:00 AM · Every Day             │ │ ← Schedule Card
│  │  🔊 meditation.mp3 · Volume: 80%     │ │
│  │  [▶ Test] [✏️ Edit] [🗑️ Delete] [✓]  │ │
│  └──────────────────────────────────────┘ │
│  ┌──────────────────────────────────────┐ │
│  │  💼 Work Break                       │ │
│  │  ⏰ 03:00 PM · Weekdays              │ │
│  │  🔊 break-bell.mp3 · Volume: 100%    │ │
│  │  [▶ Test] [✏️ Edit] [🗑️ Delete] [ ]  │ │
│  └──────────────────────────────────────┘ │
│                                            │
│  [Empty state if no schedules]            │
│                                            │
└────────────────────────────────────────────┘
```

### 2.2 Header Design

**Components:**
- App name/logo (left)
- Window controls (right, native macOS style)
- Optional: Current status indicator

**Style:**
- Clean, minimal
- Native macOS traffic lights
- Height: ~52px (standard header height)

### 2.3 Action Bar

**Primary Actions:**
- **New Schedule** (prominent button, primary color)
- **Settings** (icon button or text button)
- **Refresh** (optional, for manual sync)

**Secondary Actions:**
- Search/filter (if many schedules)
- Sort options (by time, name, etc.)
- Bulk actions (enable/disable all)

**Layout:**
- Left-aligned: Primary action (New Schedule)
- Right-aligned: Secondary actions
- Spacing: 8-12px between buttons

### 2.4 Schedule Card Design

**Information Hierarchy:**

**Priority 1 (Most Prominent):**
- Schedule name (H3, 18-20px, bold)
- Scheduled time (16-18px, medium weight)

**Priority 2 (Secondary):**
- Repeat pattern (14-16px, regular)
- Audio file name (14-16px, regular, truncated if long)
- Volume level (14-16px, regular)

**Priority 3 (Tertiary):**
- Action buttons (icons, 32x32px touch targets)
- Enable/disable toggle

**Visual Design:**
- Border: 1px subtle (light gray / dark gray depending on theme)
- Border radius: 8-12px (modern, rounded)
- Padding: 16-20px
- Shadow: Subtle (0 1px 3px rgba(0,0,0,0.1))
- Hover state: Slight shadow increase or border color change
- Spacing between cards: 12-16px

**Card States:**
- **Enabled**: Normal appearance
- **Disabled**: Reduced opacity (60%), strikethrough text
- **Active (playing)**: Accent border (blue/green), pulse animation
- **Error**: Red border, error icon

### 2.5 Empty State

**When No Schedules Exist:**
```
┌────────────────────────────────────────┐
│                                        │
│         🎵                             │
│                                        │
│    No Schedules Yet                    │
│    Create your first audio schedule    │
│    to get started                      │
│                                        │
│    [+ Create Your First Schedule]      │
│                                        │
└────────────────────────────────────────┘
```

**Components:**
- Large icon (64x64px or larger)
- Heading (H2, bold)
- Description (body text, gray)
- CTA button (prominent)

---

## 3. Add/Edit Schedule Modal

### 3.1 Modal Layout

**Structure:**
```
┌──────────────────────────────────────────────┐
│  Add New Schedule                       [×]  │ ← Header
├──────────────────────────────────────────────┤
│                                              │
│  Schedule Name                               │
│  ┌──────────────────────────────────────┐   │
│  │ Morning Meditation                   │   │
│  └──────────────────────────────────────┘   │
│                                              │
│  Audio File                                  │
│  ┌──────────────────────────────────────┐   │
│  │ meditation.mp3                       │ 🔍 │
│  └──────────────────────────────────────┘   │
│  [▶ Test Audio]                              │
│                                              │
│  Time                                        │
│  ┌──────┐  ┌──────┐  ┌────┐                │
│  │  07  │ : │  00  │  │ AM │                │
│  └──────┘  └──────┘  └────┘                │
│                                              │
│  Repeat                                      │
│  [Once] [Daily] [Weekdays] [Weekends] [Custom] │
│                                              │
│  Volume: ████████░░ 80%                      │
│                                              │
│  ┌──────────────────────────────────────┐   │
│  │  ✓ Enable this schedule              │   │
│  └──────────────────────────────────────┘   │
│                                              │
├──────────────────────────────────────────────┤
│                    [Cancel]  [Save Schedule] │ ← Footer
└──────────────────────────────────────────────┘
```

### 3.2 Input Components

**Schedule Name:**
- Type: Text input
- Placeholder: "e.g., Morning Meditation"
- Required: Yes
- Validation: 1-100 characters

**Audio File:**
- Type: File picker button
- Display: Filename (truncated if long)
- Actions: Browse, Test (play preview)
- Validation: Supported formats only (MP3, WAV, FLAC, M4A, OGG)

**Time Picker:**
- Type: Hour/Minute dropdowns or numeric inputs
- Format: 12-hour (AM/PM) or 24-hour based on system preferences
- Default: Current time + 1 hour

**Repeat Pattern:**
- Type: Button group (pill-style)
- Options: Once, Daily, Weekdays, Weekends, Custom
- Custom: Opens day selector (checkboxes for Mon-Sun)

**Volume Control:**
- Type: Slider
- Range: 0-100%
- Default: 80% (or user's preference)
- Visual: Filled track with thumb

**Enable Toggle:**
- Type: Checkbox
- Default: Enabled
- Label: "Enable this schedule"

### 3.3 Modal Behavior

**Opening:**
- Fade in + scale animation (150ms)
- Overlay darkens background
- Focus first input (schedule name)

**Validation:**
- Real-time for audio file (format check)
- On submit for required fields
- Error messages below fields (red text, icon)

**Closing:**
- Cancel button
- Click outside modal (optional)
- Escape key
- X button in header
- After successful save (with confirmation)

---

## 4. Settings Panel

### 4.1 Settings Layout

**Structure (Sidebar Navigation):**
```
┌─────────────┬──────────────────────────────┐
│  General    │  Appearance                  │
│  ────────   │  ┌────────────────────────┐  │
│             │  │ Theme                  │  │
│  Notifications│  │ ○ Light  ● System    │  │
│             │  │ ○ Dark                 │  │
│  Audio      │  └────────────────────────┘  │
│             │                              │
│  About      │  Startup                     │
│             │  ┌────────────────────────┐  │
│  ───────    │  │ ✓ Launch at login      │  │
│  Support    │  │ ✓ Minimize to tray     │  │
│             │  └────────────────────────┘  │
└─────────────┴──────────────────────────────┘
```

### 4.2 Settings Sections

**General:**
- Theme: Light / Dark / System (radio buttons)
- Launch at login (toggle)
- Minimize to tray on close (toggle)
- Language (dropdown, future)

**Notifications:**
- Show notifications (toggle)
- Notification sound (toggle)
- Notification style: Banner / Alert (radio)
- Show in Do Not Disturb (toggle)

**Audio:**
- Default volume (slider, 0-100%)
- Audio output device (dropdown, system devices)
- Fade in duration (0-10 seconds, future)
- Fade out duration (0-10 seconds, future)

**About:**
- App version
- GitHub repository link
- Check for updates button
- Credits / licenses

**Support:**
- Donation button (prominent, PayPal/Buy Me a Coffee)
- Feedback link (GitHub issues)
- Documentation link

### 4.3 Settings Interactions

**Auto-save:**
- Settings save immediately on change (no Save button needed)
- Visual feedback: "Saved" checkmark animation

**Dangerous Actions:**
- Confirmation dialogs for data deletion
- Clear warning text

---

## 5. Notification Design

### 5.1 Notification Types

**Schedule Execution:**
```
┌────────────────────────────────────┐
│  🎵 Resonatify                │
│                                    │
│  Playing: Morning Meditation       │
│  meditation.mp3                    │
│                                    │
│  [Stop]  [Dismiss]                 │
└────────────────────────────────────┘
```

**Schedule Complete:**
```
┌────────────────────────────────────┐
│  ✓ Resonatify                 │
│                                    │
│  Finished playing:                 │
│  Morning Meditation                │
│                                    │
│  [Dismiss]                         │
└────────────────────────────────────┘
```

**Error:**
```
┌────────────────────────────────────┐
│  ⚠️ Resonatify                 │
│                                    │
│  Failed to play:                   │
│  Morning Meditation                │
│  Reason: File not found            │
│                                    │
│  [Dismiss]                         │
└────────────────────────────────────┘
```

### 5.2 Notification Behavior

**Timing:**
- Appear: When action starts
- Duration: 5 seconds (banner) or until dismissed (alert)
- Sound: Optional (user preference)

**Actions:**
- Stop: Stop audio playback immediately
- Dismiss: Close notification

**Interaction:**
- Click notification: Show main window
- Hover: Pause auto-dismiss timer

---

## 6. Color Palette & Theming

### 6.1 Light Theme

**Primary Colors:**
- Primary: `#3B82F6` (Blue 500) - Buttons, links, active states
- Primary Hover: `#2563EB` (Blue 600)
- Primary Light: `#DBEAFE` (Blue 100) - Backgrounds, hover states

**Neutral Colors:**
- Background: `#FFFFFF` (White)
- Surface: `#F9FAFB` (Gray 50) - Cards, modals
- Border: `#E5E7EB` (Gray 200)
- Text Primary: `#111827` (Gray 900)
- Text Secondary: `#6B7280` (Gray 500)
- Text Tertiary: `#9CA3AF` (Gray 400)

**Semantic Colors:**
- Success: `#10B981` (Green 500)
- Warning: `#F59E0B` (Amber 500)
- Error: `#EF4444` (Red 500)
- Info: `#3B82F6` (Blue 500)

### 6.2 Dark Theme

**Primary Colors:**
- Primary: `#60A5FA` (Blue 400)
- Primary Hover: `#3B82F6` (Blue 500)
- Primary Light: `#1E3A8A` (Blue 900)

**Neutral Colors:**
- Background: `#111827` (Gray 900)
- Surface: `#1F2937` (Gray 800)
- Border: `#374151` (Gray 700)
- Text Primary: `#F9FAFB` (Gray 50)
- Text Secondary: `#D1D5DB` (Gray 300)
- Text Tertiary: `#9CA3AF` (Gray 400)

**Semantic Colors:**
- Success: `#34D399` (Green 400)
- Warning: `#FBBF24` (Amber 400)
- Error: `#F87171` (Red 400)
- Info: `#60A5FA` (Blue 400)

### 6.3 Theme Switching

**Approach:**
- CSS variables for colors
- System preference detection
- Smooth transitions (200ms)
- Remember user preference

---

## 7. Typography

### 7.1 Font Stack

**Primary Font:**
```css
font-family: -apple-system, BlinkMacSystemFont, "Segoe UI",
             Roboto, "Helvetica Neue", Arial, sans-serif;
```

**Rationale:**
- Native system fonts for best performance
- Consistent with macOS design language
- No font loading overhead

### 7.2 Type Scale

**Headings:**
- H1: 30px / 2rem, bold, -0.02em tracking
- H2: 24px / 1.5rem, bold, -0.01em tracking
- H3: 20px / 1.25rem, semibold, normal tracking
- H4: 18px / 1.125rem, semibold, normal tracking

**Body:**
- Large: 16px / 1rem, regular, normal tracking
- Base: 14px / 0.875rem, regular, normal tracking
- Small: 12px / 0.75rem, regular, 0.01em tracking

**Line Heights:**
- Headings: 1.2-1.3
- Body: 1.5-1.6
- UI elements: 1.4

---

## 8. Spacing & Layout

### 8.1 Spacing Scale

**Based on 4px base unit:**
- xs: 4px (0.25rem)
- sm: 8px (0.5rem)
- md: 12px (0.75rem)
- lg: 16px (1rem)
- xl: 24px (1.5rem)
- 2xl: 32px (2rem)
- 3xl: 48px (3rem)

### 8.2 Layout Measurements

**Window:**
- Min width: 600px
- Default width: 800px
- Max width: 1200px (for wide screens)
- Min height: 400px
- Default height: 600px

**Content:**
- Max content width: 1000px (centered)
- Padding: 20-24px (lg-xl)
- Card padding: 16-20px (lg)

**Touch Targets:**
- Minimum: 44x44px (Apple HIG)
- Buttons: 32-40px height
- Icons: 16-24px (with 44x44px touch area)

---

## 9. Animation & Transitions

### 9.1 Animation Principles

**Durations:**
- Micro: 100-150ms (hover, active states)
- Small: 150-250ms (dropdowns, tooltips)
- Medium: 250-350ms (modals, drawers)
- Large: 350-500ms (page transitions)

**Easing:**
- Standard: cubic-bezier(0.4, 0.0, 0.2, 1) - Most interactions
- Decelerate: cubic-bezier(0.0, 0.0, 0.2, 1) - Enter animations
- Accelerate: cubic-bezier(0.4, 0.0, 1, 1) - Exit animations

### 9.2 Common Animations

**Modal Open:**
```
opacity: 0 → 1
scale: 0.95 → 1
duration: 200ms
easing: decelerate
```

**Card Hover:**
```
shadow: small → medium
duration: 150ms
easing: standard
```

**Button Press:**
```
scale: 1 → 0.98
duration: 100ms
easing: standard
```

**Toggle Switch:**
```
translate: 0 → 100%
duration: 200ms
easing: standard
```

---

## 10. Iconography

### 10.1 Icon Style

**Approach:**
- Use Lucide Icons or Heroicons (React-friendly)
- Consistent stroke width (1.5-2px)
- Rounded corners
- 24x24px default size (adjust for context)

### 10.2 Icon Usage

**Primary Icons:**
- Play: ▶️ (play audio)
- Stop: ⏹️ (stop audio)
- Edit: ✏️ (edit schedule)
- Delete: 🗑️ (delete schedule)
- Add: ➕ (new schedule)
- Settings: ⚙️ (settings)
- Bell: 🔔 (notifications)
- Clock: ⏰ (time/schedule)
- Volume: 🔊 (audio/volume)
- Waveform: 📊 (audio file)
- Check: ✓ (enabled/success)
- X: ✕ (close/disabled)

**State Icons:**
- Info: ℹ️
- Warning: ⚠️
- Error: ⛔
- Success: ✅

---

## 11. Accessibility Considerations

### 11.1 Visual Accessibility

**Contrast:**
- WCAG AA minimum: 4.5:1 for body text
- WCAG AAA recommended: 7:1 for body text
- Large text (18px+): 3:1 minimum

**Focus States:**
- Visible focus indicator (2px outline)
- Skip to content link
- Logical tab order

### 11.2 Screen Reader Support

**Labels:**
- All interactive elements have labels
- Icon buttons have aria-labels
- Form inputs have associated labels
- Status messages use aria-live

**Semantic HTML:**
- Proper heading hierarchy
- List elements for lists
- Button vs. link usage
- Form semantics

### 11.3 Keyboard Navigation

**Shortcuts:**
- Cmd+N: New schedule
- Cmd+S: Save (in modal)
- Cmd+W: Close window
- Cmd+Q: Quit app
- Escape: Close modal/dialog
- Space: Toggle checkboxes/switches
- Enter: Activate focused button

**Focus Management:**
- Auto-focus first input in modals
- Return focus on modal close
- Focus visible at all times
- Skip navigation links

---

## 12. Responsive Considerations

### 12.1 Window Resizing

**Breakpoints:**
- Small: 600-799px (min width)
- Medium: 800-1000px (default)
- Large: 1000px+ (max content width)

**Behavior:**
- Layout remains consistent (desktop-only app)
- Content reflows for narrow windows
- Minimum width enforced (600px)

### 12.2 Future: Cross-Platform

**Windows:**
- Native window controls (right side)
- Different menu patterns (app menu bar vs. system menu bar)
- Adjust spacing for different DPI

**Linux:**
- Various window manager styles
- Consistent internal UI
- Theme adaptation

---

## 13. Design Inspiration Sources

### 13.1 Apps to Study

**Timer Apps:**
- Apimac Timer (feature-rich actions)
- Be Focused (clean Pomodoro UI)
- Timeless (beautiful alarm design)
- macOS Clock app (simplicity)

**Menu Bar Apps:**
- Horo (natural language input)
- Bartender (menu organization)
- Amphetamine (status states)

**Audio Apps:**
- VOX (player controls)
- Spotify (volume sliders, playback UI)
- Apple Music (file browsing)

**macOS Native Apps:**
- Reminders (list management, drag-drop)
- Calendar (event creation)
- System Settings (sidebar navigation)

### 13.2 Design Systems

**Reference:**
- Apple Human Interface Guidelines (macOS)
- shadcn/ui component library
- Tailwind CSS design tokens
- Radix UI primitives (accessible components)

---

## 14. Wireframe Notes (For Next Phase)

### 14.1 Screens to Wireframe

**Priority 1 (MVP):**
1. Main window with schedule list
2. Add/edit schedule modal
3. Settings panel (all tabs)
4. Menu bar menu
5. Empty states

**Priority 2 (Nice-to-have):**
6. Notification designs
7. System tray icon states
8. Confirmation dialogs
9. Error states

### 14.2 Wireframe Fidelity

**Low-Fidelity:**
- Grayscale boxes
- Placeholder text (Lorem ipsum)
- Basic shapes
- Focus on layout and hierarchy

**Tools:**
- Figma (preferred, collaborative)
- Sketch (macOS native)
- Excalidraw (quick, simple)
- Paper/whiteboard (fastest iteration)

---

## 15. Key Takeaways

### 15.1 Design Principles

1. **Simplicity First**: Minimal UI, maximum clarity
2. **Native Feel**: macOS design language, system fonts
3. **Reliable Feedback**: Clear states, confirmations, errors
4. **Keyboard Friendly**: Shortcuts for power users
5. **Accessible**: WCAG compliance, screen reader support
6. **Performant**: Smooth animations, fast interactions

### 15.2 Must-Have UI Elements

- ✅ Clean menu bar icon with states
- ✅ Organized, scannable schedule list
- ✅ Quick, intuitive schedule creation modal
- ✅ Non-intrusive notification design
- ✅ Simple, well-organized settings
- ✅ Empty states with clear CTAs
- ✅ Error states with helpful messages

### 15.3 Next Steps

**Immediate (Phase 1.2):**
- Create user personas
- Map user journeys
- Build low-fidelity wireframes based on these patterns

**Following (Phase 2):**
- High-fidelity mockups (optional, or go straight to code)
- Component library setup (shadcn/ui)
- Implement design system in Tailwind config

---

**Document Status**: ✅ Complete
**Next Phase**: 1.2 User Experience Design
**Dependencies**: Wireframes will reference these patterns
