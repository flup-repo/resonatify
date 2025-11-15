# Resonatify UI Redesign Plan
**Date:** 2025-11-15
**Based on:** OnyX app design patterns
**Status:** Planning

---

## Design Goals

Transform Resonatify from a basic left-aligned layout to a professional, spacious macOS utility design inspired by OnyX.

### Before (Current Issues)
- Text crowded to the left
- Insufficient spacing
- Buttons/actions mixed with content
- No clear visual hierarchy
- Plain white/light design

### After (Target)
- Professional dark theme
- Generous spacing throughout
- Right-aligned actions
- Clear section grouping
- Better use of horizontal space

---

## Color Palette

### Dark Theme (Primary)
```css
Background (darkest):  #1a1d2e
Card/Surface:          #252836
Border:                #2d3142
Text Primary:          #ffffff
Text Secondary:        #8b92ab
Accent (Blue):         #4c9aff (OnyX blue)
Accent Hover:          #6eb1ff
Success:               #00d4aa
Warning:               #ffb84d
Error:                 #ff6b6b
```

### Light Theme (Secondary)
```css
Background:            #f5f7fa
Card/Surface:          #ffffff
Border:                #e1e5eb
Text Primary:          #1a1d2e
Text Secondary:        #6b7280
Accent (Blue):         #4c9aff
```

---

## Layout Structure

### Main App Layout
```
┌─────────────────────────────────────────────────────────────┐
│  [Logo] Resonatify                    [Schedules] [Settings]│ ← Compact header
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌────────────────────────────────────────────────────┐   │
│  │                                                     │   │
│  │         Content Area (max-width: 900px)            │   │ ← Centered, generous padding
│  │                                                     │   │
│  └────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Schedules View Layout
```
┌──────────────────────────────────────────────────────────────────┐
│  Audio Schedules                        [⌘N] [+ New Schedule]   │ ← Header: left-aligned title, right-aligned actions
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  🎵  Morning Meditation                                    │ │ ← Schedule Row
│  │      Daily at 7:00 AM • Volume 80%                         │ │   Icon + Info
│  │                           [Test] [Edit] [Delete] [●─ ON]   │ │   Right-aligned actions
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  💼  Work Break                                            │ │
│  │      Weekdays at 3:00 PM • Volume 100%                     │ │
│  │                           [Test] [Edit] [Delete] [○─ OFF]  │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### Settings View Layout (OnyX-style)
```
┌──────────────────────────────────────────────────────────────────┐
│  [General] [Notifications] [Audio] [About]                       │ ← Pill tabs
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Theme                                          [Light ▼]        │ ← Row: label left, control right
│                                                                  │
│  Launch at login                                [○─────── ON]   │ ← Toggle switch
│                                                                  │
│  Start minimized                                [●─────── OFF]  │
│                                                                  │
│  ─────────────────────────────────────────────────────────────  │ ← Divider
│                                                                  │
│  Default volume                                 [━━━━━●───] 70% │ ← Slider
│                                                                  │
│  Audio device                                   [System ▼]       │ ← Dropdown
│                                                                  │
│  ─────────────────────────────────────────────────────────────  │
│                                                                  │
│                                      [Restore Defaults]      ?  │ ← Bottom actions
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Component Redesigns

### 1. Schedule Card (Row-based, Full-width)

**Current:** Vertical card with cramped layout
**New:** Horizontal row with generous spacing

```tsx
// Visual structure:
┌──────────────────────────────────────────────────────────────────┐
│ 🎵  [Name]                                                       │
│     [Description: repeat + time + volume]                        │
│                               [Test] [Edit] [Delete] [Toggle]   │ ← Actions right-aligned
└──────────────────────────────────────────────────────────────────┘

// Implementation details:
- Height: auto (min 80px with padding)
- Padding: 24px 32px (generous)
- Border: 1px solid #2d3142
- Border-radius: 12px
- Background: #252836
- Hover: subtle border color change + shadow
- Icon: 32x32px, left side
- Actions: gap-3, all right-aligned
- Toggle: iOS-style switch (40x24px)
```

### 2. Settings Rows (OnyX Pattern)

**Pattern:** Label on left, control on right

```tsx
// Visual structure:
Label text                                         [Control]

// Examples:
Theme                                              [Dropdown ▼]
Launch at login                                    [● Toggle ON]
Default volume          [🔇──────●────────🔊] 80%

// Implementation:
- Height: 56px per row
- Padding: 16px 32px
- Border-bottom: 1px solid #2d3142 (except last)
- Layout: justify-between with items-center
- Label: text-base, text-primary
- Control: min-width for consistency
```

### 3. New Schedule Modal (Vertical Form, Spacious)

```tsx
┌──────────────────────────────────────────────────────────┐
│  Create Schedule                                    [×]  │ ← Header (fixed)
├──────────────────────────────────────────────────────────┤
│                                                          │ ← Scrollable content
│  Name                                                    │
│  [Morning Meditation________________________________]    │
│                                                          │
│  Audio File                                              │
│  [meditation.mp3_______________________] [Browse...]    │
│  [▶ Test Audio]                                         │
│                                                          │
│  Time                    Repeat                          │
│  [07:00]                 [Daily ▼]                      │
│                                                          │
│  Volume                                                  │
│  [━━━━━━━━━●──────────────────────────] 80%            │
│                                                          │
│  Enable schedule         [●─────── ON]                  │
│                                                          │
├──────────────────────────────────────────────────────────┤
│                                  [Cancel]  [Create]     │ ← Footer (fixed)
└──────────────────────────────────────────────────────────┘

// Details:
- Width: 600px
- Max-height: 85vh
- Padding: 32px
- Field spacing: 24px between fields
- Labels: text-sm, uppercase, tracking-wide, text-secondary
- Inputs: height 48px, padding 12px 16px
- Buttons: height 40px, padding 12px 24px
```

### 4. Empty State (Centered, Icon-focused)

```tsx
┌──────────────────────────────────────────────────────────┐
│                                                          │
│                         ┌────┐                           │
│                         │ 🎵 │ (96x96px icon)           │
│                         └────┘                           │
│                                                          │
│                  No Schedules Yet                        │ ← text-2xl, bold
│                                                          │
│       Create audio reminders to play at specific times  │ ← text-muted
│                                                          │
│              [+ Create Your First Schedule]             │ ← Large button
│                                                          │
│         Example: meditation.mp3 every day at 7 AM       │ ← Hint text
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## Typography Scale

```css
/* Headers */
h1: 28px, font-weight: 700, letter-spacing: -0.02em
h2: 20px, font-weight: 600
h3: 16px, font-weight: 600

/* Body */
base: 14px, font-weight: 400, line-height: 1.6
large: 16px
small: 12px

/* UI Elements */
button: 14px, font-weight: 500
label: 13px, font-weight: 500, uppercase, letter-spacing: 0.05em
```

---

## Spacing System

```css
/* Based on 8px grid */
xs:  4px
sm:  8px
md:  16px
lg:  24px
xl:  32px
2xl: 48px
3xl: 64px

/* Component-specific */
Card padding:      24px 32px (lg xl)
Row padding:       16px 32px (md xl)
Section spacing:   32px (xl)
Page padding:      48px (2xl)
Modal padding:     32px (xl)
```

---

## Component Spacing Examples

### Schedule Card
```css
.schedule-card {
  padding: 24px 32px;        /* Generous internal padding */
  margin-bottom: 16px;       /* Space between cards */
  gap: 16px;                 /* Gap between icon and content */
}

.schedule-actions {
  gap: 12px;                 /* Space between buttons */
  margin-left: auto;         /* Push to right */
}
```

### Settings Row
```css
.settings-row {
  padding: 16px 32px;
  min-height: 56px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.settings-control {
  min-width: 200px;          /* Consistent control width */
}
```

---

## Interactive States

### Buttons
```css
Default:  bg-accent, text-white
Hover:    bg-accent-hover, shadow-md
Active:   bg-accent-dark, shadow-sm
Disabled: bg-muted, text-muted-foreground, opacity: 0.5
```

### Toggle Switches (iOS-style)
```css
/* ON state */
Background: #4c9aff (blue)
Thumb: white circle, right side
Animation: slide + color transition (200ms)

/* OFF state */
Background: #4a4e5f (gray)
Thumb: white circle, left side
```

### Cards/Rows
```css
Default:  border-color: #2d3142
Hover:    border-color: #4c9aff, shadow-sm
Active:   border-color: #4c9aff, shadow-md
```

---

## Animation Guidelines

```css
/* Transitions */
Color changes:     150ms ease
Shadow changes:    200ms ease
Transform:         200ms cubic-bezier(0.4, 0, 0.2, 1)
Opacity:           150ms ease

/* Modal animations */
Entry: scale(0.95) + opacity 0→1 (200ms)
Exit:  scale(0.95) + opacity 1→0 (150ms)

/* Toggle switch */
Slide: 200ms cubic-bezier(0.4, 0, 0.2, 1)
```

---

## Accessibility

### Focus States
```css
:focus-visible {
  outline: 2px solid #4c9aff;
  outline-offset: 2px;
  border-radius: 6px;
}
```

### Color Contrast
- Text on dark: WCAG AAA (7:1)
- Interactive elements: WCAG AA minimum (4.5:1)
- Use `color-contrast()` when available

### Keyboard Navigation
- All interactive elements accessible via Tab
- Visual focus indicators
- Escape closes modals/dropdowns
- Enter/Space activates buttons/toggles

---

## Implementation Priority

### Phase 1: Core Layout (High Priority) ✅ COMPLETED (2025-11-15)
1. ✅ Dark theme colors in CSS variables (src/index.css)
2. ✅ Update main app layout (centered content, compact header) (src/App.tsx)
3. ✅ Redesign schedule cards as rows (src/components/ScheduleCard.tsx)
4. ✅ Right-align all actions (src/components/ScheduleCard.tsx)
5. ✅ Install and configure @tailwindcss/vite plugin (vite.config.ts, package.json)
6. ✅ Set dark theme as default (src/types/settings.ts)
7. ✅ Implement theme sync with settings (src/App.tsx, src/hooks/useThemeSync.ts)
8. ✅ Change schedule list from grid to vertical stack (src/components/ScheduleList.tsx)

### Phase 2: Settings Redesign
1. ✅ Row-based settings layout
2. ✅ iOS-style toggle switches
3. ✅ Redesigned sliders with icons
4. ✅ Pill-style tab navigation

### Phase 3: Modal & Forms
1. ✅ Spacious modal layout
2. ✅ Better form field styling
3. ✅ Improved buttons and inputs

### Phase 4: Polish
1. ✅ Smooth animations
2. ✅ Hover states
3. ✅ Empty states
4. ✅ Loading states

---

## Figma / Design Mockups

*(To be created based on this plan)*

### Screens to Mock Up
1. Schedules view (with 3-4 schedules)
2. Schedules view (empty state)
3. Settings view (General tab)
4. Settings view (Notifications tab)
5. Create schedule modal
6. Edit schedule modal

---

## References

- **OnyX** app design patterns (Files, Parameters tabs)
- **macOS System Settings** (row-based layout)
- **iOS Settings** (toggle switches, sliders)
- **Tailwind UI** components for implementation

---

**Next Steps:**
1. Review this plan with user
2. Update Tailwind theme with new colors
3. Implement layout changes
4. Test on actual macOS environment
5. Iterate based on feedback
