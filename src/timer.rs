use crate::Result;
use crate::beep::beep;
#[cfg(not(feature = "test-beep"))]
use crate::constants::PLAY_TIMEOUT;
use crate::constants::{BEEP_DELAY, BEEP_DURATION, BEEP_FREQ, BEEP_REPETITIONS, SOUND_START_DELAY};
use crate::opts::Opts;
#[cfg(not(feature = "test-beep"))]
use crate::sound::Sound;
use crate::ui;
#[cfg(not(feature = "test-beep"))]
use crate::utils::spawn_thread;

use std::io;
use std::thread::sleep;
use std::time::Duration as stdDuration;

use regex::{Regex, RegexSet};
use tailcall::tailcall;
use time::{Duration, OffsetDateTime, Time, format_description};

pub const BELL_CHART: char = '';

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

fn play_beep() -> Result<()> {
    for _ in 0..BEEP_REPETITIONS {
        // order in the delay is because sounds start beeping ~100ms later than beep
        sleep(stdDuration::from_millis(SOUND_START_DELAY));
        // ignore beep device is not writeable
        if beep(BEEP_FREQ, stdDuration::from_millis(BEEP_DURATION)).is_err() {
            sleep(stdDuration::from_millis(BEEP_DURATION));
        }

        // let this indicated for possible changes in constant values
        #[allow(clippy::absurd_extreme_comparisons)]
        if BEEP_DELAY - SOUND_START_DELAY > 0 {
            sleep(stdDuration::from_millis(BEEP_DELAY - SOUND_START_DELAY));
        }
    }
    Ok(())
}

#[cfg(not(feature = "test-beep"))]
fn play_sound() -> Result<()> {
    let sound = Sound::new()?;

    for _ in 0..BEEP_REPETITIONS {
        sound.play()?;
        sleep(stdDuration::from_millis(BEEP_DELAY));
    }
    Ok(())
}

#[tailcall]
pub fn countdown<W: io::Write>(w: &mut W, end: OffsetDateTime, opts: &Opts) -> Result<()> {
    match end - OffsetDateTime::now_utc() {
        counter if counter > Duration::ZERO => {
            ui::draw(w, counter)?;
            sleep(stdDuration::from_secs(1));
            countdown(w, end, opts)
        }
        counter if counter <= Duration::ZERO => {
            ui::draw(w, Duration::ZERO)?;
            if opts.terminal_bell {
                println!("{BELL_CHART}");
            }

            if !opts.silence {
                // Under normal builds we play a sound in parallel and join with timeout.
                // Under the test-beep feature we skip spawning the audio thread (CI environments
                // may lack audio devices and return WouldBlock) but still exercise the beep path
                // to populate the in-memory log.
                #[cfg(not(feature = "test-beep"))]
                {
                    // error cannot be printed because we restore the terminal after this
                    let handler_with_timeout = spawn_thread(|| play_sound().unwrap());
                    play_beep()?;
                    return handler_with_timeout.join(stdDuration::from_millis(PLAY_TIMEOUT));
                }
                #[cfg(feature = "test-beep")]
                {
                    play_beep()?;
                }
            }
            Ok(())
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "test-beep")]
    use crate::beep::{__beep_log_clear, __beep_log_snapshot};
    use time::macros::time;

    #[cfg(feature = "test-beep")]
    #[test]
    fn countdown_triggers_expected_beep_cycles() {
        __beep_log_clear();
        // Construct end time already passed so countdown enters alarm branch immediately.
        let end = OffsetDateTime::now_utc() - Duration::seconds(1);
        let opts = Opts {
            r#loop: false,
            silence: false,
            terminal_bell: false,
            time: vec!["0".to_string()],
        };
        // Use a sink writer; we only care about side effects.
        let mut sink = Vec::<u8>::new();
        // This will invoke play_beep() which calls our feature-gated logger.
        countdown(&mut sink, end, &opts).unwrap();
        let log = __beep_log_snapshot();
        // Each repetition => (freq, 0)
        assert_eq!(
            log.len(),
            BEEP_REPETITIONS * 2,
            "Unexpected beep log length: {:?}",
            log
        );
        for pair in log.chunks(2) {
            assert_eq!(
                pair[1], 0,
                "Second element of each beep pair must be 0 (stop)"
            );
        }
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
}
