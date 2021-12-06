use crate::beep::beep;
use crate::constants::{BEEP_DELAY, BEEP_DURATION, BEEP_FREQ, BEEP_REPETITIONS, SOUND_START_DELAY};
use crate::sound::Sound;
use crate::ui;
use crate::Result;

use std::io;
use std::thread::sleep;
use std::time::Duration as stdDuration;

use regex::{Regex, RegexSet};
use time::{format_description, Duration, OffsetDateTime, Time};

pub fn parse_counter_time(s: &str) -> Option<Duration> {
    let re = Regex::new(r"(?:(?P<hours>\d+)h ?)?(?:(?P<minutes>\d+)m ?)?(?:(?P<seconds>\d+)s ?)?$")
        .unwrap();

    let caps = re.captures(s)?;

    // regex without `^(?!$)` must ensure of matching something
    if caps[0].is_empty() {
        return None;
    };

    let set = RegexSet::new(&[r"(\d+)h", r"(\d+)m", r"(\d+)s"]).unwrap();

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
    let format = format_description::parse("[hour]:[minute]").ok()?;
    let now = OffsetDateTime::now_local().ok()?;
    let end_time = Time::parse(s, &format).ok()?;
    let (h, m, s) = now.to_hms();
    let end_date = if Time::from_hms(h, m, s).ok()? >= end_time {
        now + Duration::days(1)
    } else {
        now
    };
    Some(end_date.replace_time(end_time))
}

pub fn resize_term<W>(w: &mut W, end: OffsetDateTime) -> crossterm::Result<()>
where
    W: io::Write,
{
    match end - OffsetDateTime::now_utc() {
        counter if counter > Duration::ZERO => ui::draw(w, counter),
        counter if counter <= Duration::ZERO => ui::draw(w, Duration::ZERO),
        _ => unreachable!(),
    }
}

pub fn countdown<W>(w: &mut W, end: OffsetDateTime) -> Result<()>
where
    W: io::Write,
{
    match end - OffsetDateTime::now_utc() {
        counter if counter > Duration::ZERO => {
            ui::draw(w, counter)?;
            sleep(stdDuration::from_secs(1));
            countdown(w, end)
        }
        counter if counter <= Duration::ZERO => {
            ui::draw(w, Duration::ZERO)?;
            let sound = Sound::new()?;
            for _ in 0..BEEP_REPETITIONS {
                sound.play()?;
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
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use time::macros::time;

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
    }

    // macos is not able to build with `unsound_local_offset` feature:
    // https://github.com/time-rs/time/issues/408
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_parse_end_time() {
        let now = OffsetDateTime::now_local().ok().unwrap();
        let date = OffsetDateTime::from(parse_end_time("12:00").unwrap());
        let expected_date = now.replace_time(time!(12:00));
        assert_eq!(date.to_hms(), expected_date.to_hms());
    }
}
