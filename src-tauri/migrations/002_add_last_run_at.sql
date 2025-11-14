-- Add last_run_at column to track the last successful execution timestamp for each schedule
ALTER TABLE schedules
    ADD COLUMN last_run_at TEXT;
