# User Journey Maps: Resonator

**Date**: 2025-11-13
**Phase**: 1.2 User Experience Design
**Status**: Complete

---

## Overview

This document maps the key user journeys through Resonator, identifying touchpoints, user thoughts, emotions, pain points, and opportunities for improvement. Each journey is mapped for our primary persona (Sarah - The Mindful Professional) with notes for other personas where they differ significantly.

---

## Journey Map Legend

**Emotional States:**
- 😊 Positive / Satisfied
- 😐 Neutral / Focused
- 😟 Confused / Uncertain
- 😤 Frustrated / Annoyed
- 🎉 Delighted / Excited

**Opportunity Types:**
- 💡 Feature opportunity
- 🎨 Design opportunity
- 📝 Content opportunity
- ⚡ Performance opportunity

---

## Journey 1: First-Time Onboarding

**Persona**: Sarah (Mindful Professional)
**Goal**: Install app and understand what it does
**Starting Point**: Just downloaded DMG file
**Success Criteria**: App installed, understand purpose, feel confident to create first schedule

### Journey Steps

#### Step 1: Installation
**Touchpoint**: Disk image mount, drag-to-Applications

**User Actions:**
- Downloads Resonator DMG from website
- Opens DMG file
- Drags app to Applications folder
- Ejects disk image
- Opens Applications folder
- Double-clicks Resonator

**User Thoughts:**
- "Okay, standard Mac installation"
- "Hope this is safe to install"

**Emotions**: 😐 Neutral, slightly cautious

**Pain Points:**
- Concern about app safety (unsigned apps show warning)
- Not sure what will happen when opened

**Opportunities:**
- 💡 Clear installation instructions on website
- 📝 Screenshot showing drag-to-Applications
- ⚡ Fast first launch (<1 second)

---

#### Step 2: First Launch & Permissions
**Touchpoint**: macOS security dialog, permission requests

**User Actions:**
- Double-clicks app
- [If unsigned] Sees Gatekeeper warning → Goes to System Settings → Security → Opens Anyway
- Sees "Resonator would like to send notifications" → Clicks Allow
- App icon appears in menu bar
- Main window opens

**User Thoughts:**
- "Is this app safe?" (if unsigned)
- "Why does it need notifications?" (expects explanation)
- "Okay, it's running"

**Emotions**: 😟 Uncertain about security → 😐 Neutral once running

**Pain Points:**
- Gatekeeper warning is scary for non-technical users
- Notification permission seems random without context
- Not sure if app is running (window might be hidden)

**Opportunities:**
- 📝 Welcome screen explaining why notifications are needed
- 💡 Show menu bar icon first, then open window
- 🎨 Clear visual that app is ready to use
- 📝 Code signing and notarization (eliminate Gatekeeper warning)

**Persona Variations:**
- **Mark**: Expects permission requests, clicks through quickly
- **Linda**: Confused by Gatekeeper, might need help

---

#### Step 3: Welcome Screen / Empty State
**Touchpoint**: Main application window (empty state)

**User Actions:**
- Sees empty state with welcome message
- Reads: "No Schedules Yet. Create your first audio schedule to get started."
- Sees large "+ Create Your First Schedule" button
- Notices menu bar icon (speaker symbol)

**User Thoughts:**
- "Oh, I can schedule audio files - that's what I wanted!"
- "This looks clean and simple"
- "I need to add a schedule to get started"

**Emotions**: 😊 Positive, understands purpose

**Pain Points:**
- None if empty state is well-designed
- Could be unclear what "audio schedule" means

**Opportunities:**
- 🎨 Beautiful empty state with clear icon
- 📝 One-sentence description: "Play audio files at specific times"
- 💡 Optional: Quick tour button (skippable)
- 📝 Show example: "Example: Play meditation.mp3 every day at 7 AM"

**Persona Variations:**
- **Mark**: Skips any tours, goes straight to creation
- **Linda**: Appreciates examples and explanations

---

#### Step 4: Decision Point
**Touchpoint**: Empty state → Create schedule button

**User Actions:**
- Hovers over "Create Your First Schedule" button
- Clicks button

**User Thoughts:**
- "Let me try creating a schedule"
- "Hope this is easy"

**Emotions**: 😊 Curious, ready to start

**Pain Points:**
- Anticipation anxiety (will this be complicated?)

**Opportunities:**
- 🎨 Inviting button design
- ⚡ Instant modal open (<100ms)
- 💡 If user hesitates, show tooltip with encouragement

---

### Onboarding Journey Summary

**Timeline**: 2-3 minutes
**Key Touchpoints**: 4 (Installation, Permissions, Welcome, Decision)
**Emotional Arc**: Cautious → Neutral → Positive → Curious
**Success Factors**:
- ✅ Fast installation and launch
- ✅ Clear explanation of app purpose
- ✅ Inviting empty state
- ✅ Obvious next step

**Critical Improvements:**
1. Code signing to eliminate Gatekeeper warning
2. Contextual permission explanation
3. Beautiful, clear empty state
4. Fast, responsive interactions

---

## Journey 2: Creating First Schedule

**Persona**: Sarah (Mindful Professional)
**Goal**: Create a schedule to play meditation audio at 7 AM daily
**Starting Point**: Clicked "Create Your First Schedule"
**Success Criteria**: Schedule created, tested, and saved successfully

### Journey Steps

#### Step 1: Modal Opens
**Touchpoint**: Add Schedule modal

**User Actions:**
- Modal fades in with "Add New Schedule" title
- Sees form with labeled fields
- Cursor auto-focuses on "Schedule Name" field

**User Thoughts:**
- "Okay, a form to fill out"
- "What should I name this?"

**Emotions**: 😐 Focused

**Pain Points:**
- Blank fields can be intimidating
- Not sure what to name it

**Opportunities:**
- 📝 Placeholder text: "e.g., Morning Meditation"
- 🎨 Clean, spacious form layout
- 💡 Progressive disclosure (show fields one at a time? No - too slow)
- 📝 Field labels are clear and self-explanatory

---

#### Step 2: Fill Schedule Name
**Touchpoint**: Schedule Name input field

**User Actions:**
- Types "Morning Meditation"
- Presses Tab to move to next field

**User Thoughts:**
- "That was easy"
- "What's next?"

**Emotions**: 😊 Confident

**Pain Points:**
- None if keyboard navigation works well

**Opportunities:**
- ⚡ Smooth tab navigation
- 🎨 Subtle visual feedback (field highlight)

**Persona Variations:**
- **Linda**: Might click instead of using Tab

---

#### Step 3: Select Audio File
**Touchpoint**: Audio File picker

**User Actions:**
- Clicks "Browse" button next to audio file field
- Native macOS file dialog opens
- Navigates to ~/Music/Meditation/
- Selects "morning-meditation.mp3"
- Clicks Open

**User Thoughts:**
- "Good, it uses the standard file picker"
- "Found my file easily"

**Emotions**: 😊 Satisfied

**Pain Points:**
- If file is in iCloud Drive, might be slow to load
- If file format is unsupported, needs clear error

**Opportunities:**
- 💡 Remember last-used directory
- 📝 Show supported formats in file dialog
- ⚡ Validate file format immediately
- 🎨 Show filename with icon after selection
- 💡 "Test Audio" button appears after file selected

**Persona Variations:**
- **Robert**: Browses to FLAC files, needs format validation
- **Linda**: Might get lost in file system, needs clear feedback

---

#### Step 4: Test Audio (Optional)
**Touchpoint**: Test Audio button

**User Actions:**
- Clicks "▶ Test Audio" button
- Audio plays for 3-5 seconds
- Button changes to "⏹️ Stop" during playback
- Audio stops

**User Thoughts:**
- "Perfect, that's the right file"
- "Good to test before saving"

**Emotions**: 😊 Reassured

**Pain Points:**
- If audio is very long, needs to be skippable
- Volume might be too loud/quiet

**Opportunities:**
- 💡 Test at the volume setting specified in form
- ⚡ Fast audio loading and playback
- 🎨 Visual feedback during playback (waveform animation?)
- 💡 Show audio duration

---

#### Step 5: Set Time
**Touchpoint**: Time picker

**User Actions:**
- Clicks hour dropdown → Selects "07"
- Clicks minute dropdown → Selects "00"
- Clicks AM/PM selector → Already set to "AM"

**User Thoughts:**
- "Simple time selection"
- "7 AM is perfect"

**Emotions**: 😊 Easy and clear

**Pain Points:**
- Dropdown can be slower than typing
- Scrolling through 60 minutes is tedious

**Opportunities:**
- 💡 Allow keyboard typing (HH:mm format)
- 💡 15-minute increments for faster selection
- 🎨 Large, easy-to-click dropdowns
- 💡 Remember common times

**Persona Variations:**
- **Mark**: Would type "07:00" directly if possible
- **Linda**: Prefers clicking dropdowns

---

#### Step 6: Set Repeat Pattern
**Touchpoint**: Repeat button group

**User Actions:**
- Sees button group: [Once] [Daily] [Weekdays] [Weekends] [Custom]
- Clicks [Daily]
- Button highlights with accent color

**User Thoughts:**
- "Daily is what I want"
- "That was simple"

**Emotions**: 😊 Satisfied

**Pain Points:**
- None if buttons are well-labeled

**Opportunities:**
- 🎨 Clear active state (filled button)
- 💡 Show icons: 📅 (Once), 🔄 (Daily), 💼 (Weekdays), 🏖️ (Weekends), ⚙️ (Custom)
- 📝 Tooltip on hover explaining each option

**Persona Variations:**
- **Mark**: Clicks [Weekdays] for work-only schedules
- **Linda**: Uses [Once] for specific class times

---

#### Step 7: Set Volume
**Touchpoint**: Volume slider

**User Actions:**
- Sees slider at 80% (default)
- Drags slightly to 75%

**User Thoughts:**
- "80% is probably good, maybe a bit lower"

**Emotions**: 😐 Neutral

**Pain Points:**
- Hard to hit exact value with slider
- Not sure what volume level is appropriate

**Opportunities:**
- 💡 Show percentage number next to slider
- 💡 Click slider track to jump to position
- 💡 Keyboard arrows for fine adjustment
- 📝 Suggested volumes: "Default (80%)", "Quiet (50%)", "Loud (100%)"

---

#### Step 8: Enable Schedule
**Touchpoint**: Enable checkbox

**User Actions:**
- Sees "✓ Enable this schedule" (checked by default)
- Leaves it checked

**User Thoughts:**
- "Yes, I want this enabled"
- "Good that it's default enabled"

**Emotions**: 😊 Confident

**Pain Points:**
- None

**Opportunities:**
- 📝 Clear label
- 🎨 Prominent checkbox

---

#### Step 9: Save Schedule
**Touchpoint**: Save button

**User Actions:**
- Reviews form (quick mental check)
- Clicks "Save Schedule" button
- Modal closes with fade-out animation
- Main window shows new schedule card
- Success notification: "Schedule created successfully"

**User Thoughts:**
- "Done! That was easy"
- "I see my schedule in the list"

**Emotions**: 🎉 Delighted

**Pain Points:**
- None if save is instant

**Opportunities:**
- ⚡ Instant save (<200ms)
- 🎨 Smooth modal close animation
- 💡 Brief success message (toast notification)
- 🎨 New schedule card animates into view
- 💡 Highlight new schedule briefly

**Persona Variations:**
- **Linda**: Wants clear confirmation that save worked
- **Mark**: Doesn't need confirmation, expects it to work

---

#### Step 10: Confirmation & Next Steps
**Touchpoint**: Main window with first schedule

**User Actions:**
- Sees schedule card with all details
- Reads: "Morning Meditation | 07:00 AM · Every Day | meditation.mp3 · Volume: 75%"
- Sees toggle is enabled (green)
- Notices menu bar icon

**User Thoughts:**
- "Perfect, it's all there"
- "It will play tomorrow morning"
- "I can add more if I want"

**Emotions**: 🎉 Accomplished

**Pain Points:**
- Not clear when it will execute next
- Unsure if app needs to stay open

**Opportunities:**
- 💡 Show "Next: Tomorrow at 7:00 AM" in schedule card
- 📝 Small info tooltip: "App runs in background - you can close this window"
- 💡 Suggest adding more schedules if appropriate

---

### First Schedule Creation Summary

**Timeline**: 2-3 minutes
**Key Touchpoints**: 10 (Modal → Name → Audio → Test → Time → Repeat → Volume → Enable → Save → Confirm)
**Emotional Arc**: Focused → Confident → Satisfied → Accomplished
**Success Factors**:
- ✅ Clear, self-explanatory form fields
- ✅ Test audio feature builds confidence
- ✅ Simple repeat patterns (not overcomplex)
- ✅ Instant save with clear confirmation

**Critical Improvements:**
1. Auto-focus first field
2. Test audio button
3. Fast save with visual feedback
4. Show "Next execution" time
5. Clear background execution message

---

## Journey 3: Managing Multiple Schedules

**Persona**: Sarah (after 1 week of use)
**Goal**: Review schedules, adjust one, temporarily disable another
**Starting Point**: Has 3 schedules running (7 AM, 10 AM, 3 PM)
**Success Criteria**: Successfully edits one, disables one temporarily, closes app with confidence

### Journey Steps

#### Step 1: Opening App
**Touchpoint**: Menu bar icon or Dock

**User Actions:**
- Clicks menu bar icon → Sees menu with "Show Window"
- Clicks "Show Window"
- Main window opens showing 3 schedule cards

**User Thoughts:**
- "Let me check my schedules"
- "All three are there and enabled"

**Emotions**: 😊 Confident

**Pain Points:**
- Forgot if app window was already open or not
- Not sure how to open window (menu bar vs. Dock)

**Opportunities:**
- 💡 Keyboard shortcut to toggle window (Cmd+0)
- 🎨 Menu shows "Show Window" or "Hide Window" dynamically
- 💡 Window reopens to previous position
- 📝 Menu bar tooltip shows next scheduled item

---

#### Step 2: Reviewing Schedules
**Touchpoint**: Main window schedule list

**User Actions:**
- Scans three schedule cards
- Sees: Morning Meditation (7 AM), Work Break (10 AM), Afternoon Meditation (3 PM)
- Checks toggle states (all enabled)
- Notices "Next: Tomorrow at 7:00 AM" on top card

**User Thoughts:**
- "Everything looks good"
- "Wait, I have a meeting at 3 PM tomorrow, need to disable that one"

**Emotions**: 😐 Focused, slight concern

**Pain Points:**
- Lots of information to scan
- Need to quickly disable one without deleting

**Opportunities:**
- 🎨 Color-coded schedule cards (optional)
- 💡 Sort by next execution time (earliest first)
- 💡 Show upcoming executions for next 7 days (optional calendar view)
- 🎨 Clear visual hierarchy (name biggest, details smaller)

---

#### Step 3: Temporarily Disabling Schedule
**Touchpoint**: Schedule card toggle

**User Actions:**
- Finds "Afternoon Meditation" schedule
- Clicks toggle switch (right side of card)
- Toggle animates from green (on) to gray (off)
- Card appearance dims slightly
- Sees brief confirmation: "Schedule disabled"

**User Thoughts:**
- "Perfect, that's disabled for tomorrow"
- "I can re-enable it easily later"

**Emotions**: 😊 Satisfied

**Pain Points:**
- None if toggle is obvious and instant

**Opportunities:**
- ⚡ Instant toggle (no loading delay)
- 🎨 Clear visual difference (enabled vs. disabled)
- 💡 Update "Next:" text to next enabled schedule
- 💡 Quick "Disable All" button for meetings (menu bar menu)

**Persona Variations:**
- **Mark**: Uses "Pause All Schedules" from menu bar frequently
- **Linda**: Nervous about accidentally toggling, appreciates confirmation

---

#### Step 4: Editing Schedule
**Touchpoint**: Schedule card edit button

**User Actions:**
- Decides to change Work Break volume (too loud)
- Finds "Work Break" schedule card
- Clicks "✏️ Edit" button
- Edit modal opens (same as Add, but pre-filled)
- Changes volume from 100% to 70%
- Clicks "Save Schedule"
- Modal closes, card updates

**User Thoughts:**
- "That was easy to edit"
- "Volume will be better now"

**Emotions**: 😊 Confident

**Pain Points:**
- None if edit is smooth

**Opportunities:**
- ⚡ Fast modal open with pre-filled data
- 💡 Highlight changed fields after save
- 🎨 Clear visual that card updated
- 💡 Show last edited timestamp (optional)

---

#### Step 5: Closing App
**Touchpoint**: Window close button

**User Actions:**
- Finished reviewing/editing
- Clicks red window close button (top left)
- Window closes
- Menu bar icon remains

**User Thoughts:**
- "Is the app still running?"
- "Will my schedules still work?"

**Emotions**: 😟 Slightly uncertain

**Pain Points:**
- Not clear if app is still running
- Uncertainty about background execution

**Opportunities:**
- 💡 First time closing: Show tooltip "Resonator is still running in the menu bar"
- 📝 Settings option: "Minimize to tray on close" (macOS: hide window)
- 🎨 Menu bar icon stays visible as confirmation
- 💡 Notification: "Schedules will continue in the background"

**Persona Variations:**
- **Mark**: Expects this behavior, no uncertainty
- **Linda**: Needs reassurance app is still running

---

#### Step 6: Background Confirmation
**Touchpoint**: Menu bar icon (after closing window)

**User Actions:**
- Hovers over menu bar icon
- Sees tooltip: "Next: Tomorrow at 7:00 AM"
- Continues with other work

**User Thoughts:**
- "Okay, it's still running"
- "I'll get my meditation tomorrow morning"

**Emotions**: 😊 Reassured

**Pain Points:**
- Initial uncertainty resolved

**Opportunities:**
- 🎨 Informative tooltip
- 💡 Menu shows upcoming schedules
- 💡 Icon changes slightly when next schedule is soon (<15 min)

---

### Managing Multiple Schedules Summary

**Timeline**: 1-2 minutes
**Key Touchpoints**: 6 (Open → Review → Disable → Edit → Close → Confirm)
**Emotional Arc**: Confident → Focused → Satisfied → Uncertain → Reassured
**Success Factors**:
- ✅ Easy to scan multiple schedules
- ✅ Instant toggle for enable/disable
- ✅ Edit is as easy as create
- ✅ Clear background running confirmation

**Critical Improvements:**
1. Clear visual states (enabled vs. disabled)
2. Instant toggle with visual feedback
3. Background running reassurance
4. Menu bar tooltip with next schedule
5. Quick "Pause All" option

---

## Journey 4: Adjusting Settings

**Persona**: Sarah (after 2 weeks of use)
**Goal**: Enable launch at login, switch to dark theme, adjust default volume
**Starting Point**: Opens app from menu bar
**Success Criteria**: Settings changed and persisted

### Journey Steps

#### Step 1: Opening Settings
**Touchpoint**: Main window header or menu

**User Actions:**
- Opens main window
- Looks for settings
- Clicks "Settings" button (gear icon, top right) OR
- Menu: Resonator → Settings (Cmd+,)

**User Thoughts:**
- "Where are settings?"
- "Ah, gear icon makes sense"

**Emotions**: 😐 Neutral

**Pain Points:**
- Settings location not immediately obvious
- Multiple entry points can be confusing

**Opportunities:**
- 🎨 Clear gear icon in header
- 💡 Standard keyboard shortcut (Cmd+,)
- 📝 "Settings" label next to icon
- 💡 Menu bar menu also has "Settings..." option

---

#### Step 2: Settings Panel Opens
**Touchpoint**: Settings window/panel

**User Actions:**
- Settings opens in new window OR sidebar panel
- Sees sidebar navigation: General, Notifications, Audio, About, Support
- "General" is selected by default

**User Thoughts:**
- "Clean layout"
- "I see sections for different settings"

**Emotions**: 😊 Clear and organized

**Pain Points:**
- Too many sections could be overwhelming
- Not sure which section has what

**Opportunities:**
- 🎨 Sidebar navigation (macOS System Settings style)
- 💡 Icons for each section
- 📝 Search bar (future) to find settings
- 🎨 Clean, spacious layout

---

#### Step 3: Enable Launch at Login
**Touchpoint**: General settings

**User Actions:**
- Already in "General" section
- Sees "Launch at login" toggle (off)
- Clicks toggle
- Toggle animates to "on"
- [macOS] Sees permission dialog: "Resonator would like to add a login item"
- Clicks "Allow"
- Toggle stays on

**User Thoughts:**
- "Easy to find"
- "Good, it asks for permission"

**Emotions**: 😊 Confident

**Pain Points:**
- Permission dialog might be unexpected
- Not sure what login item means (Linda)

**Opportunities:**
- 📝 Explain what login item means (tooltip or helper text)
- 💡 Auto-save setting immediately
- 🎨 Visual confirmation (brief checkmark)
- 📝 If permission denied, show helpful message

**Persona Variations:**
- **Linda**: Needs explanation of "login item"
- **Mark**: Expects this, no explanation needed

---

#### Step 4: Switch Theme
**Touchpoint**: General settings (Appearance section)

**User Actions:**
- Scrolls down slightly to "Appearance" section
- Sees theme radio buttons: ○ Light ● System ○ Dark
- Clicks "○ Dark"
- App theme instantly switches to dark
- Schedule cards, background, text all change

**User Thoughts:**
- "Ooh, dark mode looks nice"
- "Instant change, I like that"

**Emotions**: 🎉 Delighted

**Pain Points:**
- None if theme switching is smooth

**Opportunities:**
- ⚡ Instant theme change (no reload)
- 🎨 Smooth color transition (200ms)
- 💡 Remember preference across restarts
- 🎨 Beautiful dark theme implementation

---

#### Step 5: Adjust Default Volume
**Touchpoint**: Audio settings section

**User Actions:**
- Clicks "Audio" in sidebar
- Sees "Default volume for new schedules"
- Sees slider at 80%
- Drags to 70%
- Sees "70%" text update

**User Thoughts:**
- "That's better for my new schedules"
- "Existing schedules keep their volumes, right?"

**Emotions**: 😊 Satisfied, slight uncertainty

**Pain Points:**
- Not clear if this affects existing schedules
- Might want to apply to all schedules

**Opportunities:**
- 📝 Helper text: "This only affects new schedules"
- 💡 Button: "Apply to all schedules" (with confirmation)
- 🎨 Clear labeling
- 💡 Show test button to hear volume level

**Persona Variations:**
- **Robert**: Cares a lot about volume control
- **Mark**: Sets once and forgets

---

#### Step 6: Explore Support Section
**Touchpoint**: Support settings section

**User Actions:**
- Clicks "Support" in sidebar
- Sees donation button: "Support Development ☕"
- Sees feedback link: "Report Issue or Request Feature"
- Sees GitHub repository link

**User Thoughts:**
- "Nice, I can support this"
- "Good to know how to report bugs"

**Emotions**: 😊 Positive

**Pain Points:**
- None

**Opportunities:**
- 🎨 Prominent but not pushy donation button
- 📝 Clear, friendly copy
- 💡 Show recent updates/changelog
- 💡 Social proof: "Join XX users supporting Resonator"

---

#### Step 7: Close Settings
**Touchpoint**: Settings window close button

**User Actions:**
- Clicks close button or presses Escape
- Settings window closes
- Returns to main window (still open)

**User Thoughts:**
- "All my changes are saved"
- "That was easy"

**Emotions**: 😊 Satisfied

**Pain Points:**
- None

**Opportunities:**
- ⚡ Auto-save all settings (no Save button needed)
- 💡 Brief confirmation: "Settings saved"
- 🎨 Smooth window close

---

### Settings Adjustment Summary

**Timeline**: 1-2 minutes
**Key Touchpoints**: 7 (Open → Navigate → Launch at Login → Theme → Volume → Support → Close)
**Emotional Arc**: Neutral → Clear → Confident → Delighted → Satisfied
**Success Factors**:
- ✅ Organized sidebar navigation
- ✅ Auto-save (no manual save needed)
- ✅ Instant theme switching
- ✅ Clear explanations where needed

**Critical Improvements:**
1. Sidebar navigation (System Settings style)
2. Auto-save with visual feedback
3. Helper text for complex settings
4. Instant theme switching
5. Clear section organization

---

## Journey 5: Schedule Execution (Passive Experience)

**Persona**: Sarah
**Goal**: Experience automated schedule execution
**Starting Point**: Normal work day, app running in background
**Success Criteria**: Audio plays at scheduled time, user is aware but not disrupted

### Journey Steps

#### Step 1: Pre-Execution (2 minutes before)
**Touchpoint**: Menu bar icon (passive awareness)

**User Actions:**
- Working on laptop
- Glances at menu bar
- Notices icon (if she looks)

**User Thoughts:**
- "My meditation is coming up soon"
- (Or doesn't notice, which is fine)

**Emotions**: 😐 Neutral, anticipatory

**Pain Points:**
- Might forget schedule is coming
- No advance warning

**Opportunities:**
- 💡 Optional notification: "Morning Meditation in 2 minutes"
- 🎨 Icon changes slightly when schedule is imminent
- 💡 Menu bar tooltip shows countdown

---

#### Step 2: Execution (Exact Time)
**Touchpoint**: Audio playback, notification

**User Actions:**
- 7:00 AM arrives
- Audio starts playing (morning-meditation.mp3)
- Notification appears: "🎵 Playing: Morning Meditation"
- Notification shows "Stop" button

**User Thoughts:**
- "Perfect timing"
- "That's my meditation track"

**Emotions**: 😊 Satisfied, ready to meditate

**Pain Points:**
- If volume is too loud/quiet, jarring
- If in meeting, disruptive (but she disabled it)
- If file is missing, error is frustrating

**Opportunities:**
- ⚡ Instant playback (no delay)
- 🎨 Clear, non-intrusive notification
- 💡 Quick "Stop" action in notification
- 📝 Log execution for history

**Persona Variations:**
- **Linda**: Critical that this works perfectly (paid classes)
- **Mark**: Expects reliability, notices immediately if it fails

---

#### Step 3: During Playback
**Touchpoint**: Audio continues, menu bar icon shows active state

**User Actions:**
- Listens to meditation audio (10 minutes)
- Menu bar icon shows "active" state (filled/pulsing)
- If she checks menu bar: Sees "Now Playing: Morning Meditation"

**User Thoughts:**
- "This is working perfectly"
- (Focused on meditation, not thinking about app)

**Emotions**: 😊 Calm, satisfied

**Pain Points:**
- If audio stops unexpectedly (device disconnected, file error)
- If computer goes to sleep, audio might stop

**Opportunities:**
- 💡 Handle computer sleep gracefully
- 💡 Show progress in menu bar menu
- 💡 Quick volume adjustment during playback (menu bar)
- ⚡ Reliable playback even with system events

---

#### Step 4: Playback Complete
**Touchpoint**: Audio ends, notification

**User Actions:**
- Audio finishes playing
- Notification: "✓ Finished playing: Morning Meditation"
- Menu bar icon returns to idle state

**User Thoughts:**
- "Done, that was great"
- "Next one is at 10 AM"

**Emotions**: 😊 Accomplished

**Pain Points:**
- None

**Opportunities:**
- 📝 Log completion in history
- 💡 Optional: "Schedule complete" notification (or skip to reduce notifications)
- 🎨 Menu bar tooltip updates to next schedule

---

#### Step 5: Between Schedules (Passive)
**Touchpoint**: Menu bar icon

**User Actions:**
- Continues with work
- App runs silently in background
- Menu bar icon remains visible

**User Thoughts:**
- (Not thinking about app)
- "It'll play again at 10 AM"

**Emotions**: 😐 Neutral, confident

**Pain Points:**
- None

**Opportunities:**
- ⚡ Minimal resource usage (<50MB RAM, <5% CPU)
- 💡 Silent, invisible operation
- 🎨 Only shows when needed (next schedule, if queried)

---

### Schedule Execution Summary

**Timeline**: 10-15 minutes (including playback)
**Key Touchpoints**: 5 (Pre-execution → Execution → During → Complete → Between)
**Emotional Arc**: Neutral → Satisfied → Calm → Accomplished → Confident
**Success Factors**:
- ✅ Exact timing (99.9% accuracy)
- ✅ Instant playback start
- ✅ Clear but non-intrusive notifications
- ✅ Silent background operation between schedules

**Critical Improvements:**
1. Precise execution timing
2. Instant audio playback
3. Handle system sleep/wake
4. Clear active/idle states
5. Minimal resource usage

---

## Cross-Journey Insights

### Common Pain Points Across Journeys:
1. **Uncertainty about background execution** (Journeys 1, 3)
2. **Permission dialogs without context** (Journeys 1, 4)
3. **Not clear when next schedule will execute** (Journeys 2, 3)
4. **Volume levels might be wrong** (Journeys 2, 4, 5)
5. **Fear of accidentally deleting/disabling** (Journey 3)

### Common Delighters Across Journeys:
1. **Instant, smooth interactions** (All journeys)
2. **Test audio before saving** (Journey 2)
3. **Instant theme switching** (Journey 4)
4. **Reliable execution** (Journey 5)
5. **Clear visual feedback** (All journeys)

### Opportunities to Improve Overall Experience:
1. 💡 Onboarding tour (optional, skippable)
2. 💡 Contextual help tooltips
3. 💡 "Next execution" visibility everywhere
4. 💡 Keyboard shortcuts for power users
5. 💡 Execution history log
6. 💡 Quick actions in menu bar
7. 💡 Better handling of system sleep/wake
8. 💡 Volume preview in settings

---

## Persona-Specific Journey Variations

### Sarah (Primary - Mindful Professional):
- **Priorities**: Reliability, simplicity, non-intrusiveness
- **Journey Focus**: Set up once, minimal interaction
- **Key Touchpoints**: Create schedules, passive execution
- **Pain Points**: Background execution uncertainty, volume issues

### Mark (Power User - Remote Worker with ADHD):
- **Priorities**: Speed, keyboard shortcuts, flexibility
- **Journey Focus**: Frequent adjustments, power features
- **Key Touchpoints**: Keyboard shortcuts, quick enable/disable, menu bar
- **Pain Points**: Lack of keyboard shortcuts, slow interactions

### Linda (Non-Technical - Yoga Instructor):
- **Priorities**: Clarity, reliability, helpful guidance
- **Journey Focus**: Careful setup, frequent reviews, professional use
- **Key Touchpoints**: Settings confirmation, clear labels, error messages
- **Pain Points**: Technical jargon, fear of mistakes, unclear states

### Alex (Content Creator):
- **Priorities**: Branding, quick access during work, flexibility
- **Journey Focus**: Multiple schedules, frequent modifications
- **Key Touchpoints**: Menu bar quick actions, schedule list
- **Pain Points**: Slow access during recording, limited schedule slots

### Robert (Musician):
- **Priorities**: Audio quality, precision timing, professional reliability
- **Journey Focus**: High-quality audio setup, volume control
- **Key Touchpoints**: Audio format validation, volume settings, timing precision
- **Pain Points**: Format support, audio quality degradation, timing inaccuracy

---

## Journey Optimization Checklist

For each journey, validate:

**Efficiency:**
- [ ] Minimum steps to complete goal?
- [ ] No unnecessary clicks or interactions?
- [ ] Fast, responsive interactions (<200ms)?
- [ ] Keyboard shortcuts available for key actions?

**Clarity:**
- [ ] Clear next steps at each stage?
- [ ] Helpful error messages if something fails?
- [ ] Visual feedback for all actions?
- [ ] Obvious exit/cancel options?

**Delight:**
- [ ] Smooth animations (not jarring)?
- [ ] Confirmation of successful actions?
- [ ] Pleasant surprises (e.g., auto-focus, smart defaults)?
- [ ] Sense of accomplishment at completion?

**Accessibility:**
- [ ] Screen reader compatible?
- [ ] Keyboard navigable?
- [ ] High contrast mode support?
- [ ] Clear focus indicators?

---

## Next Steps: Wireframing

Based on these journey maps, wireframes should focus on:

1. **Empty state** (Journey 1, Step 3)
2. **Add/Edit schedule modal** (Journey 2, Steps 1-9)
3. **Schedule list with multiple cards** (Journey 3, Step 2)
4. **Settings panel with sidebar** (Journey 4, Steps 2-6)
5. **Menu bar menu** (Journeys 3, 5)
6. **Notification designs** (Journey 5, Steps 2, 4)

---

**Document Status**: ✅ Complete
**Next**: Wireframes
**Owner**: UX Design Team

---

**Last Updated**: 2025-11-13
