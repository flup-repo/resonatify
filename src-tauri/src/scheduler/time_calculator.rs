use chrono::{
    DateTime, Datelike, Duration, Local, LocalResult, NaiveDate, NaiveDateTime, NaiveTime,
    TimeZone, Weekday,
};

use crate::db::models::{RepeatType, Schedule};

use super::SchedulerError;

const TIME_FORMAT: &str = "%H:%M";
const GRACE_PERIOD: Duration = Duration::minutes(1);

pub fn next_execution_time(
    schedule: &Schedule,
    reference: DateTime<Local>,
    last_run: Option<DateTime<Local>>,
) -> Result<Option<DateTime<Local>>, SchedulerError> {
    if !schedule.enabled {
        return Ok(None);
    }

    let time = parse_time(&schedule.scheduled_time).map_err(|err| {
        SchedulerError::InvalidScheduleTime {
            schedule_id: schedule.id.clone(),
            reason: err,
        }
    })?;

    let next = match &schedule.repeat_type {
        RepeatType::Once | RepeatType::Daily => Some(find_next_matching_day(
            reference,
            time,
            last_run,
            |_, _| true,
        )),
        RepeatType::Weekdays => Some(find_next_matching_day(reference, time, last_run, |date, _| {
            matches!(
                date.weekday(),
                Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri
            )
        })),
        RepeatType::Weekends => Some(find_next_matching_day(reference, time, last_run, |date, _| {
            matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
        })),
        RepeatType::Weekly { days } => {
            if days.is_empty() {
                None
            } else {
                Some(find_next_matching_day(reference, time, last_run, |date, _| {
                    days.iter().any(|d| *d == date.weekday())
                }))
            }
        }
        RepeatType::Custom { interval_minutes } => {
            if *interval_minutes == 0 {
                return Err(SchedulerError::InvalidScheduleTime {
                    schedule_id: schedule.id.clone(),
                    reason: "custom interval must be greater than 0".into(),
                });
            }

            Some(find_next_custom_interval(
                reference,
                time,
                *interval_minutes as i64,
                last_run,
            ))
        }
    };

    Ok(next)
}

fn parse_time(value: &str) -> Result<NaiveTime, String> {
    NaiveTime::parse_from_str(value, TIME_FORMAT)
        .map_err(|err| format!("invalid time '{value}' - {err}"))
}

fn find_next_matching_day<F>(
    reference: DateTime<Local>,
    time: NaiveTime,
    last_run: Option<DateTime<Local>>,
    predicate: F,
) -> DateTime<Local>
where
    F: Fn(NaiveDate, DateTime<Local>) -> bool,
{
    let mut date = reference.date_naive();

    loop {
        let candidate = combine(date, time);

        if predicate(date, candidate) {
            if candidate >= reference {
                return candidate;
            }

            if reference - candidate <= GRACE_PERIOD && should_fire_with_grace(reference, last_run) {
                return reference;
            }
        }

        date = date.succ_opt().unwrap_or_else(|| date + Duration::days(1));
    }
}

fn find_next_custom_interval(
    reference: DateTime<Local>,
    time: NaiveTime,
    interval_minutes: i64,
    last_run: Option<DateTime<Local>>,
) -> DateTime<Local> {
    let interval_minutes = interval_minutes.max(1);
    let interval = Duration::minutes(interval_minutes);
    let mut candidate = combine(reference.date_naive(), time);

    if candidate > reference {
        return candidate;
    }

    if reference - candidate <= GRACE_PERIOD && should_fire_with_grace(reference, last_run) {
        return reference;
    }

    let elapsed = reference - candidate;
    let interval_seconds = interval.num_seconds().max(1);
    let intervals_passed = (elapsed.num_seconds() + interval_seconds - 1) / interval_seconds;
    let advance_seconds = interval_seconds * (intervals_passed + 1);
    candidate += Duration::seconds(advance_seconds);

    candidate
}

fn combine(date: NaiveDate, time: NaiveTime) -> DateTime<Local> {
    let naive = NaiveDateTime::new(date, time);

    match Local.from_local_datetime(&naive) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(dt, _) => dt,
        LocalResult::None => {
            // Handle DST gaps by moving forward minute-by-minute until a valid time exists.
            let mut attempt = naive + Duration::minutes(1);
            loop {
                match Local.from_local_datetime(&attempt) {
                    LocalResult::Single(dt) => break dt,
                    LocalResult::Ambiguous(dt, _) => break dt,
                    LocalResult::None => attempt += Duration::minutes(1),
                }
            }
        }
    }
}

fn should_fire_with_grace(
    reference: DateTime<Local>,
    last_run: Option<DateTime<Local>>,
) -> bool {
    match last_run {
        Some(last) => reference - last >= GRACE_PERIOD,
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Duration, FixedOffset};

    fn schedule_with_repeat(repeat: RepeatType, time: &str) -> Schedule {
        Schedule {
            id: "test".into(),
            name: "Test".into(),
            audio_file_path: "/tmp/test.mp3".into(),
            scheduled_time: time.into(),
            enabled: true,
            repeat_type: repeat,
            volume: 80,
            created_at: "".into(),
            updated_at: "".into(),
            last_run_at: None,
        }
    }

    #[test]
    fn next_execution_daily_same_minute_returns_reference() {
        let reference = Local::now();
        let schedule_time = reference.format("%H:%M").to_string();
        let schedule = schedule_with_repeat(RepeatType::Daily, &schedule_time);

        let next = next_execution_time(&schedule, reference, None).unwrap().unwrap();

        let diff = next - reference;
        assert!(diff >= Duration::seconds(0));
        assert!(diff <= Duration::minutes(1));
    }

    #[test]
    fn next_execution_weekday_skips_to_monday() {
        // Saturday
        let reference = DateTime::<FixedOffset>::parse_from_rfc3339("2025-11-15T10:00:00+00:00")
            .unwrap()
            .with_timezone(&Local);
        let schedule = schedule_with_repeat(RepeatType::Weekdays, "09:30");

        let next = next_execution_time(&schedule, reference, None).unwrap().unwrap();

        assert_eq!(next.weekday(), Weekday::Mon);
    }

    #[test]
    fn next_execution_custom_interval_moves_forward() {
        let reference = DateTime::<FixedOffset>::parse_from_rfc3339("2025-11-13T10:45:00+00:00")
            .unwrap()
            .with_timezone(&Local);
        let schedule = schedule_with_repeat(
            RepeatType::Custom {
                interval_minutes: 45,
            },
            "08:00",
        );

        let next = next_execution_time(&schedule, reference, None).unwrap().unwrap();

        assert!(next > reference);
    }
}
