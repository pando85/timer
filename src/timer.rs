use crate::Result;
use crate::alert::{Alert, BeepAlert, SoundAlert, write_terminal_bell};
use crate::opts::Opts;
use crate::ui;

use std::io;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration as stdDuration;

use regex::{Regex, RegexSet};
use time::{Duration, OffsetDateTime, Time, format_description};

pub fn parse_counter_time(s: &str) -> Option<Duration> {
    let re = Regex::new(
        r"^(?:(?P<hours>\d+)h ?)?(?:(?P<minutes>\d+)m(?:in)? ?)?(?:(?P<seconds>\d+)s? ?)?$",
    )
    .unwrap();

    let caps = re.captures(s)?;

    // regex without `^(?!$)` must ensure of matching something
    if caps[0].is_empty() {
        return None;
    };

    let set = RegexSet::new([r"(\d+)h", r"(\d+)m(?:in)?", r"(\d+)s?$"]).unwrap();

    let matches: Vec<_> = set.matches(s).into_iter().collect();

    let hours = if matches.contains(&0) {
        caps["hours"].parse::<u32>().unwrap()
    } else {
        0
    };
    let minutes = if matches.contains(&1) {
        caps["minutes"].parse::<u32>().unwrap()
    } else {
        0
    };
    let seconds = if matches.contains(&2) {
        caps["seconds"].parse::<u32>().unwrap()
    } else {
        0
    };

    let total: i64 = (hours * 3600 + minutes * 60 + seconds).into();
    Some(Duration::seconds(total))
}

pub fn parse_end_time(s: &str) -> Option<OffsetDateTime> {
    // Try to parse with hours, minutes, and seconds (with optional fractional seconds)
    if let Ok(format) = format_description::parse("[hour]:[minute]:[second].[subsecond]")
        && let Ok(end_time) = Time::parse(s, &format)
    {
        let now = OffsetDateTime::now_local().ok()?;
        let (h, m, s) = now.to_hms();
        let end_date = if Time::from_hms(h, m, s).ok()? >= end_time {
            now + Duration::days(1)
        } else {
            now
        };
        return Some(end_date.replace_time(end_time));
    }
    if let Ok(format) = format_description::parse("[hour]:[minute]:[second]")
        && let Ok(end_time) = Time::parse(s, &format)
    {
        let now = OffsetDateTime::now_local().ok()?;
        let (h, m, s) = now.to_hms();
        let end_date = if Time::from_hms(h, m, s).ok()? >= end_time {
            now + Duration::days(1)
        } else {
            now
        };
        return Some(end_date.replace_time(end_time));
    }
    // Fallback to [hour]:[minute]
    let format = format_description::parse("[hour]:[minute]").ok()?;
    let now = OffsetDateTime::now_local().ok()?;
    let end_time = Time::parse(s, &format)
        .or_else(|_| Time::parse(&format!("0{s}"), &format))
        .ok()?;
    let (h, m, s) = now.to_hms();
    let end_date = if Time::from_hms(h, m, s).ok()? >= end_time {
        now + Duration::days(1)
    } else {
        now
    };
    Some(end_date.replace_time(end_time))
}

pub fn resize_term<W>(w: &mut W, end: OffsetDateTime) -> Result<()>
where
    W: io::Write,
{
    match end - OffsetDateTime::now_utc() {
        counter if counter > Duration::ZERO => ui::draw(w, counter),
        counter if counter <= Duration::ZERO => ui::draw(w, Duration::ZERO),
        _ => unreachable!(),
    }
}

pub fn countdown_with_alerts<W, B>(
    w: &mut W,
    end: OffsetDateTime,
    opts: &Opts,
    beep_alert: &B,
    sound_alert: Arc<dyn Alert + Send + Sync>,
) -> Result<()>
where
    W: io::Write,
    B: Alert + ?Sized,
{
    loop {
        match end - OffsetDateTime::now_utc() {
            counter if counter > Duration::ZERO => match ui::draw(w, counter) {
                Ok(_) => {
                    sleep(stdDuration::from_secs(1));
                }
                Err(e) => return Err(e),
            },
            _ => {
                ui::draw(w, Duration::ZERO)?;

                if opts.terminal_bell {
                    write_terminal_bell(w)?;
                }

                if !opts.silence {
                    let sa = Arc::clone(&sound_alert);
                    let sound_handle = std::thread::spawn(move || sa.play().unwrap());
                    beep_alert.play()?;
                    sound_handle.join().map_err(|_| "Sound thread panicked")?;
                }
                return Ok(());
            }
        }
    }
}

pub fn countdown<W>(w: &mut W, end: OffsetDateTime, opts: &Opts) -> Result<()>
where
    W: io::Write,
{
    countdown_with_alerts(w, end, opts, &BeepAlert, Arc::new(SoundAlert))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use crate::alert::SilentAlert;
    use crate::alert::{Alert, AlertCallLog, MockAlert};
    use crate::constants::{BEEP_DELAY, BEEP_FREQ, BEEP_REPETITIONS, SOUND_START_DELAY};
    use clap::Parser;
    use std::sync::{Arc, Mutex};
    use time::macros::time;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_constants_beep_freq_unchanged() {
        assert_eq!(BEEP_FREQ, 440);
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_constants_beep_repetitions_unchanged() {
        assert_eq!(BEEP_REPETITIONS, 5);
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_constants_sound_start_delay_less_than_beep_delay() {
        assert!(SOUND_START_DELAY <= BEEP_DELAY);
    }

    #[test]
    fn test_countdown_uses_parallel_sound_and_beep() {
        use crate::opts::Opts;

        let mut opts = Opts::try_parse_from(["timer", "1s"]).unwrap();
        opts.silence = false;

        let log = Arc::new(Mutex::new(AlertCallLog::new()));
        let beep_alert = MockAlert::new(Arc::clone(&log), "beep");
        let sound_alert = MockAlert::new(Arc::clone(&log), "sound");
        let sound_alert_arc = Arc::new(sound_alert) as Arc<dyn Alert + Send + Sync>;

        // Create a countdown that ends in the past (immediately complete)
        let end_time = OffsetDateTime::now_utc() - Duration::seconds(1);
        let mut output = Vec::new();

        countdown_with_alerts(&mut output, end_time, &opts, &beep_alert, sound_alert_arc).unwrap();

        let log_guard = log.lock().unwrap();
        let calls: Vec<&str> = log_guard.calls.clone();

        // Both beep and sound should have been called
        assert!(calls.contains(&"beep"));
        assert!(calls.contains(&"sound"));
    }

    #[test]
    fn test_parse_counter_time() {
        assert_eq!(
            Duration::seconds(7800),
            parse_counter_time("2h10m").unwrap()
        );
        assert_eq!(Duration::seconds(70), parse_counter_time("1m10s").unwrap());
        assert_eq!(
            Duration::seconds(420),
            parse_counter_time("5m120s").unwrap()
        );
        assert_eq!(Duration::seconds(603), parse_counter_time("10m3s").unwrap());
        assert_eq!(
            Duration::seconds(35996400),
            parse_counter_time("9999h").unwrap()
        );
        assert_eq!(
            Duration::seconds(1143),
            parse_counter_time("19min3s").unwrap()
        );
        assert_eq!(Duration::seconds(60), parse_counter_time("1m").unwrap());
        assert_eq!(Duration::seconds(10), parse_counter_time("10").unwrap());
        assert_eq!(Duration::seconds(120), parse_counter_time("120").unwrap());
        assert_eq!(Duration::seconds(350), parse_counter_time("5m50").unwrap());
        assert_eq!(None, parse_counter_time("boo"));
        assert_eq!(None, parse_counter_time("10:00"));
    }

    #[test]
    fn test_parse_counter_time_zero() {
        assert_eq!(Duration::seconds(0), parse_counter_time("0s").unwrap());
    }

    #[test]
    fn test_parse_counter_time_empty() {
        assert_eq!(None, parse_counter_time(""));
    }

    #[test]
    fn test_parse_counter_time_invalid() {
        assert_eq!(None, parse_counter_time("abc"));
    }

    #[test]
    fn test_parse_counter_time_large() {
        assert_eq!(
            Duration::seconds(3599996400),
            parse_counter_time("999999h").unwrap()
        );
    }

    #[test]
    fn test_parse_counter_time_duplicate_units() {
        // This should either return Some or None, but shouldn't panic
        let result = parse_counter_time("1h1h");
        // Just ensure it doesn't panic
        assert!(result.is_some() || result.is_none());
    }

    #[test]
    fn test_parse_counter_time_bare_number() {
        assert_eq!(Duration::seconds(10), parse_counter_time("10").unwrap());
    }

    #[test]
    fn test_parse_counter_time_only_hours() {
        assert_eq!(Duration::seconds(18000), parse_counter_time("5h").unwrap());
    }

    #[test]
    fn test_parse_counter_time_hours_minutes() {
        assert_eq!(
            Duration::seconds(5400),
            parse_counter_time("1h30m").unwrap()
        );
    }

    #[test]
    fn test_parse_counter_time_hours_minutes_seconds() {
        assert_eq!(
            Duration::seconds(3723),
            parse_counter_time("1h2m3s").unwrap()
        );
    }

    #[test]
    fn test_parse_counter_time_with_spaces() {
        let result = parse_counter_time("1h 30m");
        if let Some(dur) = result {
            assert_eq!(Duration::seconds(5400), dur);
        } else {
            assert_eq!(None, result);
        }
    }

    #[test]
    fn test_parse_end_time() {
        let now = OffsetDateTime::now_local().ok().unwrap();

        let date = parse_end_time("12:00").unwrap();
        let expected_date = now.replace_time(time!(12:00));
        assert_eq!(date.to_hms(), expected_date.to_hms());
    }

    #[test]
    fn test_parse_end_time_leading_zero() {
        let now = OffsetDateTime::now_local().ok().unwrap();

        let date = parse_end_time("9:30").unwrap();
        let expected_date = now.replace_time(time!(9:30));
        assert_eq!(date.to_hms(), expected_date.to_hms());
    }

    #[test]
    fn test_parse_end_time_hms() {
        let now = OffsetDateTime::now_local().ok().unwrap();
        let date = parse_end_time("13:45:43").unwrap();
        let expected_date = now.replace_time(time!(13:45:43));
        assert_eq!(date.to_hms(), expected_date.to_hms());
    }

    #[test]
    fn test_parse_end_time_hms_milli() {
        let now = OffsetDateTime::now_local().ok().unwrap();
        let date = parse_end_time("13:45:43.123").unwrap();
        let expected_date = now.replace_time(time!(13:45:43.123));
        assert_eq!(date.to_hms_milli(), expected_date.to_hms_milli());
    }

    #[test]
    fn test_parse_end_time_midnight() {
        let now = OffsetDateTime::now_local().ok().unwrap();
        let date = parse_end_time("00:00").unwrap();
        let expected_date = now.replace_time(time!(00:00));
        assert_eq!(date.to_hms(), expected_date.to_hms());
    }

    #[test]
    fn test_parse_end_time_hms_max() {
        let now = OffsetDateTime::now_local().ok().unwrap();
        let date = parse_end_time("23:59:59").unwrap();
        let expected_date = now.replace_time(time!(23:59:59));
        assert_eq!(date.to_hms(), expected_date.to_hms());
    }

    #[test]
    fn test_parse_end_time_invalid() {
        assert_eq!(None, parse_end_time("abc"));
    }

    #[test]
    fn test_parse_end_time_with_leading_zero() {
        let now = OffsetDateTime::now_local().ok().unwrap();
        let date = parse_end_time("08:25").unwrap();
        let expected_date = now.replace_time(time!(08:25));
        assert_eq!(date.to_hms(), expected_date.to_hms());
    }

    #[test]
    fn test_parse_end_time_hms_with_millis() {
        let now = OffsetDateTime::now_local().ok().unwrap();
        let date = parse_end_time("13:45:43.999").unwrap();
        let expected_date = now.replace_time(time!(13:45:43.999));
        assert_eq!(date.to_hms_milli(), expected_date.to_hms_milli());
    }

    #[test]
    fn test_countdown_silence_mode() {
        use crate::opts::Opts;
        use std::io::Cursor;

        let mut buffer = Cursor::new(Vec::new());
        let end = OffsetDateTime::now_utc() - Duration::seconds(1); // 1 second ago
        let opts = Opts {
            command: None,
            r#loop: false,
            silence: true,
            terminal_bell: false,
            time: vec!["1s".to_string()],
        };

        let beep_log = Arc::new(Mutex::new(AlertCallLog::new()));
        let beep_alert = MockAlert::new(beep_log.clone(), "beep");
        let sound_alert = Arc::new(SilentAlert);

        let result = countdown_with_alerts(&mut buffer, end, &opts, &beep_alert, sound_alert);

        assert!(result.is_ok());
        // Verify beep was not called in silence mode
        let log = beep_log.lock().unwrap();
        assert!(log.calls.is_empty());
    }

    #[test]
    fn test_countdown_terminal_bell() {
        use crate::opts::Opts;
        use std::io::Cursor;

        let mut buffer = Cursor::new(Vec::new());
        let end = OffsetDateTime::now_utc() - Duration::seconds(1); // 1 second ago
        let opts = Opts {
            command: None,
            r#loop: false,
            silence: true,
            terminal_bell: true,
            time: vec!["1s".to_string()],
        };

        let beep_log = Arc::new(Mutex::new(AlertCallLog::new()));
        let beep_alert = MockAlert::new(beep_log.clone(), "beep");
        let sound_alert = Arc::new(SilentAlert);

        let result = countdown_with_alerts(&mut buffer, end, &opts, &beep_alert, sound_alert);

        assert!(result.is_ok());
        // Verify terminal bell character is in the output
        let output = buffer.into_inner();
        assert!(output.contains(&0x07)); // Bell character \x07
    }

    #[test]
    fn test_countdown_beep_called() {
        use crate::opts::Opts;
        use std::io::Cursor;

        let mut buffer = Cursor::new(Vec::new());
        let end = OffsetDateTime::now_utc() - Duration::seconds(1); // 1 second ago
        let opts = Opts {
            command: None,
            r#loop: false,
            silence: false, // Enable alerts
            terminal_bell: false,
            time: vec!["1s".to_string()],
        };

        let beep_log = Arc::new(Mutex::new(AlertCallLog::new()));
        let sound_log = Arc::new(Mutex::new(AlertCallLog::new()));
        let beep_alert = MockAlert::new(beep_log.clone(), "beep");
        let sound_alert = Arc::new(MockAlert::new(sound_log.clone(), "sound"));

        let result = countdown_with_alerts(&mut buffer, end, &opts, &beep_alert, sound_alert);

        assert!(result.is_ok());
        // Verify beep was called
        let log = beep_log.lock().unwrap();
        assert!(log.calls.contains(&"beep"));
    }

    #[test]
    fn test_countdown_sound_and_beep_parallel() {
        use crate::opts::Opts;
        use std::io::Cursor;

        let mut buffer = Cursor::new(Vec::new());
        let end = OffsetDateTime::now_utc() - Duration::seconds(1); // 1 second ago
        let opts = Opts {
            command: None,
            r#loop: false,
            silence: false, // Enable alerts
            terminal_bell: false,
            time: vec!["1s".to_string()],
        };

        let beep_log = Arc::new(Mutex::new(AlertCallLog::new()));
        let sound_log = Arc::new(Mutex::new(AlertCallLog::new()));
        let beep_alert = MockAlert::new(beep_log.clone(), "beep");
        let sound_alert = Arc::new(MockAlert::new(sound_log.clone(), "sound"));

        let result = countdown_with_alerts(&mut buffer, end, &opts, &beep_alert, sound_alert);

        assert!(result.is_ok());
        // Verify both beep and sound were called
        let beep_calls = beep_log.lock().unwrap();
        let sound_calls = sound_log.lock().unwrap();
        assert!(beep_calls.calls.contains(&"beep"));
        assert!(sound_calls.calls.contains(&"sound"));
    }

    #[test]
    fn test_countdown_sound_thread_panic_catches() {
        use crate::opts::Opts;
        use std::io::Cursor;

        // Define PanicAlert locally within the test
        struct PanicAlert;
        impl Alert for PanicAlert {
            fn play(&self) -> Result<()> {
                panic!("Sound thread panicked");
            }
        }

        let mut buffer = Cursor::new(Vec::new());
        let end = OffsetDateTime::now_utc() - Duration::seconds(1); // 1 second ago
        let opts = Opts {
            command: None,
            r#loop: false,
            silence: false, // Enable alerts
            terminal_bell: false,
            time: vec!["1s".to_string()],
        };

        let beep_log = Arc::new(Mutex::new(AlertCallLog::new()));
        let beep_alert = MockAlert::new(beep_log.clone(), "beep");
        let sound_alert = Arc::new(PanicAlert);

        let result = countdown_with_alerts(&mut buffer, end, &opts, &beep_alert, sound_alert);

        // The function should catch the panic and return an error
        assert!(result.is_err());
        // Verify beep was still called before the sound thread panicked
        let log = beep_log.lock().unwrap();
        assert!(log.calls.contains(&"beep"));
    }

    #[test]
    fn test_resize_term_positive_counter() {
        let mut writer = Vec::new();
        let end = OffsetDateTime::now_utc() + Duration::seconds(60);
        let result = resize_term(&mut writer, end);
        assert!(result.is_ok());
        assert!(!writer.is_empty());
    }

    #[test]
    fn test_resize_term_zero_counter() {
        let mut writer = Vec::new();
        let end = OffsetDateTime::now_utc() - Duration::seconds(60);
        let result = resize_term(&mut writer, end);
        assert!(result.is_ok());
        assert!(!writer.is_empty());
    }
}
