-- Schedules table stores user-defined audio schedules
CREATE TABLE IF NOT EXISTS schedules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    audio_file_path TEXT NOT NULL,
    scheduled_time TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    repeat_type TEXT NOT NULL,
    repeat_days TEXT,
    volume INTEGER NOT NULL DEFAULT 100 CHECK (volume BETWEEN 0 AND 100),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_schedules_enabled ON schedules(enabled);
CREATE INDEX IF NOT EXISTS idx_schedules_scheduled_time ON schedules(scheduled_time);

-- Settings table stores key/value configuration
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- Playback history stores execution log entries
CREATE TABLE IF NOT EXISTS audio_playback_history (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    played_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    status TEXT NOT NULL CHECK (status IN ('success', 'failed', 'skipped')),
    error_message TEXT,
    FOREIGN KEY (schedule_id) REFERENCES schedules(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_history_schedule_id ON audio_playback_history(schedule_id);
CREATE INDEX IF NOT EXISTS idx_history_played_at ON audio_playback_history(played_at DESC);

-- Keep only the most recent 1000 history entries to prevent unbounded growth
CREATE TRIGGER IF NOT EXISTS trg_history_trim
AFTER INSERT ON audio_playback_history
BEGIN
    DELETE FROM audio_playback_history
    WHERE id NOT IN (
        SELECT id FROM audio_playback_history
        ORDER BY played_at DESC
        LIMIT 1000
    );
END;
