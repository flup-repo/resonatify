# User Personas: Resonatify

**Date**: 2025-11-13
**Phase**: 1.2 User Experience Design
**Status**: Complete

---

## Overview

This document defines the target user personas for Resonatify. These personas guide design decisions, feature prioritization, and user experience considerations throughout development.

---

## Primary Personas

### Persona 1: Sarah - The Mindful Professional

**Demographics:**
- **Age**: 32
- **Occupation**: Product Manager at tech company
- **Location**: San Francisco, CA
- **Tech Savvy**: High
- **macOS Experience**: 5+ years

**Background:**
Sarah works long hours in a high-stress environment. She's discovered that scheduled meditation breaks significantly improve her focus and reduce stress. She uses various wellness apps but hasn't found a good solution for playing her custom meditation audio files at specific times throughout the day.

**Goals:**
- Schedule meditation audio to play at 10 AM, 3 PM, and 7 PM daily
- Use her own curated meditation recordings (not generic app sounds)
- Set up once and forget - reliable background execution
- Minimal disruption - wants subtle notifications
- Quick access to pause/skip if in a meeting

**Pain Points:**
- Current timer apps only play system sounds
- Calendar reminders require too many clicks to start audio
- Music apps don't have reliable scheduling
- Doesn't want another heavy, battery-draining app
- Frustrated by apps that don't work when laptop is closed/sleeping

**Technical Comfort:**
- Comfortable with file management
- Uses keyboard shortcuts regularly
- Appreciates native macOS design
- Values privacy (prefers local-only apps)

**Usage Patterns:**
- Sets up schedules once, rarely changes them
- Runs app in background all day
- Occasionally adjusts volume per schedule
- Checks menu bar for next scheduled session
- Uses test/preview feature before committing

**User Needs:**
1. **Reliability**: Schedules must execute even if app is minimized
2. **Simplicity**: Quick setup, minimal configuration
3. **Flexibility**: Different audio files for different times
4. **Control**: Easy to pause all schedules temporarily (before meetings)
5. **Feedback**: Clear confirmation that schedule is active

**Quote:**
> "I just want my meditation audio to play automatically at specific times. Why is this so hard to find?"

---

### Persona 2: Mark - The Remote Worker with ADHD

**Demographics:**
- **Age**: 28
- **Occupation**: Software Developer (remote)
- **Location**: Austin, TX
- **Tech Savvy**: Very High
- **macOS Experience**: 3+ years

**Background:**
Mark has ADHD and works from home. He uses audio cues to maintain structure in his day - break reminders, focus session starts, and end-of-workday signals. He's built custom audio files (music + voice reminders) that work best for him. He needs these to play automatically because he hyperfocuses and loses track of time.

**Goals:**
- Schedule work breaks every 90 minutes (Pomodoro-style)
- Play different audio for different break types (short break, lunch, end of day)
- Weekday-only schedules (doesn't want them on weekends)
- Consistent routine to build healthy work habits
- Option to quickly add ad-hoc timers when needed

**Pain Points:**
- Forgets to take breaks when hyperfocused
- Generic timer beeps don't break his focus effectively
- Needs personalized audio that he responds to
- Other apps are too complicated or require too much interaction
- Wants "set and forget" automation

**Technical Comfort:**
- Power user, loves keyboard shortcuts
- Comfortable with scripts and automation
- Appreciates open-source software
- Would contribute feedback/bug reports
- Interested in future API/scripting features

**Usage Patterns:**
- Creates multiple schedules for different break types
- Adjusts schedules occasionally based on work patterns
- Uses keyboard shortcut to quickly disable all schedules
- Appreciates upcoming schedule visibility
- Would use advanced features like conditional schedules (future)

**User Needs:**
1. **Automation**: Must work without manual intervention
2. **Customization**: Different audio files, volumes, and patterns
3. **Visibility**: Know what's coming next without opening app
4. **Quick Control**: Fast enable/disable for flexible days
5. **Performance**: Lightweight, doesn't impact development work

**Quote:**
> "I need my break reminders to be automatic and impossible to ignore, but also easy to turn off when I need to."

---

### Persona 3: Linda - The Yoga Instructor

**Demographics:**
- **Age**: 45
- **Occupation**: Yoga instructor and wellness coach
- **Location**: Portland, OR
- **Tech Savvy**: Medium
- **macOS Experience**: 2 years (switched from Windows)

**Background:**
Linda teaches yoga classes from her home studio via Zoom and in-person. She needs to transition between different class segments at specific times, each with its own background music or meditation audio. She also uses scheduled audio reminders for personal daily routines (morning affirmations, evening wind-down).

**Goals:**
- Schedule different meditation/music tracks for class segments
- Personal daily affirmations at 7 AM
- Evening wind-down music at 9 PM
- Easy to adjust if class schedule changes
- Professional reliability - can't fail during paid classes

**Pain Points:**
- Not super technical - needs simple, intuitive interface
- Tried other apps but found them confusing
- Doesn't want to learn complex software
- Needs visual clarity about what's scheduled when
- Worried about accidentally deleting or disabling important schedules

**Technical Comfort:**
- Basic file management
- Comfortable with drag-and-drop interfaces
- Prefers clicking over keyboard shortcuts
- Needs clear labels and confirmations
- Appreciates helpful error messages

**Usage Patterns:**
- Creates 5-10 schedules total
- Reviews schedule list frequently to confirm settings
- Uses descriptive names for each schedule
- Tests audio files before saving schedules
- Adjusts occasionally for class schedule changes

**User Needs:**
1. **Clarity**: Clear visual feedback about what's scheduled
2. **Safety**: Confirmation dialogs for destructive actions
3. **Simplicity**: Obvious buttons and intuitive flows
4. **Reliability**: Professional-grade dependability
5. **Support**: Helpful error messages and documentation

**Quote:**
> "I'm not a tech person, but I need this to just work. My students are paying for professional classes."

---

## Secondary Personas

### Persona 4: Alex - The Content Creator

**Demographics:**
- **Age**: 24
- **Occupation**: YouTuber / Streamer
- **Location**: Los Angeles, CA
- **Tech Savvy**: High
- **macOS Experience**: 4 years

**Background:**
Alex creates content full-time and needs audio cues for various production tasks - recording session reminders, upload deadlines, stream start warnings. Uses custom audio files with personalized messages to keep workflow organized.

**Goals:**
- Schedule recording session reminders with custom audio
- Upload deadline warnings (different audio for different urgency)
- Stream start countdowns
- Custom audio files with personality (branded sounds)

**Pain Points:**
- Juggling multiple projects with different deadlines
- Needs audio reminders because visual ones get missed
- Generic sounds don't match personal brand
- Needs quick access during recording/streaming sessions

**User Needs:**
1. **Branding**: Ability to use custom, branded audio
2. **Flexibility**: Easy to create/modify schedules frequently
3. **Quick Access**: Menu bar controls during streaming
4. **Variety**: Multiple different schedules for different content types
5. **Reliability**: Can't miss deadlines or stream times

**Quote:**
> "I need reminders that match my brand and personality, not boring system beeps."

---

### Persona 5: Robert - The Musician

**Demographics:**
- **Age**: 38
- **Occupation**: Professional musician / composer
- **Location**: Nashville, TN
- **Tech Savvy**: Medium-High
- **macOS Experience**: 10+ years (music production background)

**Background:**
Robert uses scheduled audio for practice routines, composition sessions, and teaching intervals. He has specific music theory exercises on audio files that need to play at practice times, and metronome patterns for different practice segments.

**Goals:**
- Schedule practice session audio reminders
- Play different exercises at different times
- Teaching timer (student session boundaries)
- Composition session time blocks with audio cues

**Pain Points:**
- Needs precise timing for music education
- Audio quality matters (audiophile standards)
- Current solutions lack flexibility for custom audio
- Wants to use his own recorded exercises

**User Needs:**
1. **Audio Quality**: High-quality playback (FLAC support)
2. **Precision**: Exact timing for musical purposes
3. **Customization**: Own recordings and exercises
4. **Professional**: Reliable for teaching purposes
5. **Volume Control**: Different volumes for different purposes

**Quote:**
> "I need high-quality audio playback at precise times for my teaching and practice routines."

---

## Persona Usage Matrix

| Feature | Sarah | Mark | Linda | Alex | Robert |
|---------|-------|------|-------|------|--------|
| Multiple daily schedules | ✓✓✓ | ✓✓✓ | ✓✓ | ✓✓✓ | ✓✓ |
| Custom audio files | ✓✓✓ | ✓✓✓ | ✓✓✓ | ✓✓✓ | ✓✓✓ |
| Weekday-only schedules | ✓ | ✓✓✓ | ✓ | ✓✓ | ✓✓ |
| Background execution | ✓✓✓ | ✓✓✓ | ✓✓✓ | ✓✓✓ | ✓✓✓ |
| Menu bar access | ✓✓ | ✓✓✓ | ✓ | ✓✓✓ | ✓ |
| Quick pause all | ✓✓✓ | ✓✓✓ | ✓ | ✓✓ | ✓ |
| Volume per schedule | ✓✓ | ✓✓ | ✓✓ | ✓ | ✓✓✓ |
| Test/preview audio | ✓✓ | ✓ | ✓✓✓ | ✓✓ | ✓✓✓ |
| Simple UI | ✓✓ | ✓ | ✓✓✓ | ✓ | ✓✓ |
| Keyboard shortcuts | ✓ | ✓✓✓ | - | ✓✓ | ✓ |
| FLAC support | - | - | - | - | ✓✓✓ |
| Clear confirmations | ✓ | - | ✓✓✓ | ✓ | ✓ |

**Legend:** ✓✓✓ = Critical, ✓✓ = Important, ✓ = Nice-to-have, - = Not needed

---

## Design Implications

### From Sarah (Mindful Professional):
- **UI Priority**: Clean, minimal interface
- **Features**: Reliable background execution, subtle notifications
- **Interaction**: Set once, minimal ongoing interaction
- **Testing**: Must handle sleep/wake correctly

### From Mark (Remote Worker):
- **UI Priority**: Keyboard shortcuts, power user features
- **Features**: Quick enable/disable all, upcoming schedule visibility
- **Interaction**: Frequent adjustments, menu bar controls
- **Testing**: Performance impact must be minimal

### From Linda (Yoga Instructor):
- **UI Priority**: Clear visual hierarchy, obvious buttons
- **Features**: Confirmation dialogs, descriptive naming, test audio
- **Interaction**: Mostly clicking, needs visual feedback
- **Testing**: Error handling and helpful messages critical

### From Alex (Content Creator):
- **UI Priority**: Fast access during recording/streaming
- **Features**: Multiple schedules, quick modifications
- **Interaction**: Menu bar quick actions
- **Testing**: Reliability under load

### From Robert (Musician):
- **UI Priority**: Professional appearance, precision controls
- **Features**: High-quality audio formats, volume control
- **Interaction**: Precision timing, audio quality validation
- **Testing**: FLAC support, timing accuracy

---

## Accessibility Considerations

### Visual Impairments:
- **Personas**: All users benefit from high contrast
- **Requirements**:
  - WCAG AA compliance minimum
  - Screen reader support (Linda, older users)
  - Large text options
  - Clear focus indicators

### Motor Impairments:
- **Personas**: Linda (less keyboard-focused), older users
- **Requirements**:
  - Large touch targets (44x44px minimum)
  - Forgiving click areas
  - No double-click requirements
  - Keyboard alternatives for all actions

### Cognitive Considerations:
- **Personas**: Linda (less technical), all users under stress
- **Requirements**:
  - Clear labeling and instructions
  - Consistent patterns
  - Confirmation for destructive actions
  - Helpful error messages
  - Progressive disclosure of complexity

### Auditory (Ironic for audio app):
- **Personas**: Hearing-impaired users might still use for automation
- **Requirements**:
  - Visual feedback for all audio events
  - Notification alternatives
  - Status indicators

---

## Usage Scenarios by Persona

### Sarah's Typical Day:
```
07:00 - Wakes up, sees notification: "Morning meditation in 15m"
07:15 - Meditation audio plays automatically (7 AM schedule)
10:00 - Work break meditation plays (10 AM schedule)
14:45 - In meeting, right-clicks menu bar → "Pause All Schedules"
15:30 - Meeting ends, right-clicks menu bar → "Resume All Schedules"
19:00 - Evening meditation plays (7 PM schedule)
```

### Mark's Typical Workday:
```
09:00 - Starts work, app already running (launch at login)
10:30 - Break reminder audio plays (90-minute schedule)
10:35 - Returns to work
12:00 - Lunch break audio plays (custom schedule)
13:00 - Back to work
14:30 - Break reminder audio plays (90-minute schedule)
16:00 - Break reminder audio plays (90-minute schedule)
17:30 - End of day audio plays (custom schedule)
Weekend - No schedules execute (weekday-only setting)
```

### Linda's Class Day:
```
07:00 - Personal morning affirmation plays
10:00 - Prepares for 10:30 class
10:30 - Class intro music plays (scheduled for class start)
10:45 - Warm-up segment music plays (scheduled transition)
11:00 - Main practice music plays (scheduled transition)
11:20 - Cool-down music plays (scheduled transition)
11:30 - Class end bell plays (scheduled)
21:00 - Evening wind-down music plays (personal schedule)
```

---

## Anti-Personas (Not Our Target Users)

### Enterprise IT Administrator:
- **Why not**: Needs fleet management, user policies, centralized control
- **What they need**: MDM integration, policy deployment, audit logs
- **Our focus**: Individual users, not enterprise

### DJ / Live Performer:
- **Why not**: Needs real-time control, complex mixing, professional DAW features
- **What they need**: Real-time audio manipulation, effects, mixing
- **Our focus**: Simple scheduled playback, not performance tools

### Smart Home Enthusiast:
- **Why not**: Wants extensive home automation integration
- **What they need**: MQTT, HomeKit, Alexa integration, complex rules
- **Our focus**: Personal computer scheduling, not IoT

### Power Gamer:
- **Why not**: Needs gaming-specific features (Discord integration, game detection)
- **What they need**: Game-aware triggers, streaming integration
- **Our focus**: Time-based scheduling, not event-based

---

## Persona Evolution (Future Phases)

### Phase 2 - Expanded Actions:
- **New persona**: Developer (script execution)
- **Enhanced for**: Mark (automation enthusiast)
- **Features**: Run scripts, open apps, send notifications

### Phase 3 - Recording:
- **New persona**: Podcaster (quick voice notes)
- **Enhanced for**: Alex, Robert (record custom audio in-app)
- **Features**: In-app recording, editing, quick capture

### Phase 4 - AI Integration:
- **New persona**: Personal productivity optimizer
- **Enhanced for**: All personas (smart suggestions)
- **Features**: AI-suggested schedules, voice control, natural language

---

## Persona Validation Checklist

Use this checklist to validate design decisions:

**For each feature, ask:**
- [ ] Which persona(s) need this?
- [ ] Is it critical, important, or nice-to-have for them?
- [ ] Does it conflict with any persona's goals?
- [ ] Does it add complexity for Linda (least technical)?
- [ ] Does it provide enough power for Mark (most technical)?
- [ ] Is it accessible for all users?

**For each UI pattern, ask:**
- [ ] Can Linda find and understand this?
- [ ] Can Mark access this via keyboard?
- [ ] Does Sarah need to interact with this often?
- [ ] Is it professional enough for Linda's classes?
- [ ] Is it fast enough for Alex during content creation?

---

## Summary: Core User Needs Across All Personas

**Universal Needs (All Personas):**
1. ✅ Reliable background execution
2. ✅ Custom audio file support
3. ✅ Multiple schedules
4. ✅ Simple setup process
5. ✅ Clear visual feedback

**High-Priority Needs (Most Personas):**
6. ✅ Weekday-only scheduling
7. ✅ Quick pause/resume all schedules
8. ✅ Volume control per schedule
9. ✅ Test/preview audio
10. ✅ Menu bar access

**Technical vs. Non-Technical Split:**
- **Technical users (Sarah, Mark, Alex)**: Value keyboard shortcuts, power features, performance
- **Non-technical users (Linda)**: Value clear UI, confirmations, helpful errors
- **Professional users (Linda, Robert)**: Value reliability above all else

**Design Balance:**
- Default to simple, clear UI (Linda)
- Add power user features that don't clutter (Mark)
- Ensure professional reliability (Linda, Robert)
- Maintain performance (Mark, Alex)

---

**Document Status**: ✅ Complete
**Next**: User Journey Maps
**Owner**: UX Design Team

---

**Last Updated**: 2025-11-13
