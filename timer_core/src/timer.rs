use crate::beep::beep;
use crate::constants::{BEEP_DELAY, BEEP_DURATION, BEEP_FREQ, BEEP_REPETITIONS};
use crate::ui;

use std::io;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use time::{format_description, OffsetDateTime, Time};
use crossterm::Result;

use regex::{Regex, RegexSet};

pub fn parse_counter_time(s: &str) -> Option<Duration> {
    let re = Regex::new(r"(?:(?P<hours>\d+)h ?)?(?:(?P<minutes>\d+)m ?)?(?:(?P<seconds>\d+)s ?)?$")
        .unwrap();

    let caps = re.captures(s)?;

    // regex without `^(?!$)` must ensure of matching something
    if caps[0].is_empty() {
        return None
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

    let total: u64 = (hours * 3600 + minutes * 60 + seconds).into();
    Some(Duration::from_secs(total))
}

pub fn parse_end_time(s: &str) -> Option<SystemTime> {
    let format = format_description::parse("[hour]:[minute]").ok()?;
    let now = OffsetDateTime::now_utc();
    let end_time = Time::parse(s, &format).ok()?;
    let end = now.replace_time(end_time);
    let duration = end - now;
    let system = SystemTime::now();
    Some(system + duration)
}

pub fn resize_term<W>(w: &mut W, end: SystemTime) -> Result<()>
where
    W: io::Write,
{
    match end.duration_since(SystemTime::now()) {
        Ok(counter) => ui::draw(w, counter),
        Err(_) => ui::draw(w, Duration::ZERO),
    }
}

pub fn countdown<W>(w: &mut W, end: SystemTime) -> Result<()>
where
    W: io::Write,
{
    match end.duration_since(SystemTime::now()) {
        Ok(counter) => {
            ui::draw(w, counter)?;
            sleep(Duration::from_secs(1));
            countdown(w, end)
        }
        Err(_) => {
            ui::draw(w, Duration::ZERO)?;
            for _ in 0..BEEP_REPETITIONS {
                beep(BEEP_FREQ, Duration::from_millis(BEEP_DURATION)).unwrap();
                sleep(Duration::from_millis(BEEP_DELAY));
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use time::macros::time;

    #[test]
    fn test_parse_counter_time() {
        assert_eq!(Duration::from_secs(7800), parse_counter_time("2h10m").unwrap());
        assert_eq!(Duration::from_secs(70), parse_counter_time("1m10s").unwrap());
        assert_eq!(Duration::from_secs(420), parse_counter_time("5m120s").unwrap());
        assert_eq!(Duration::from_secs(603), parse_counter_time("10m3s").unwrap());
        assert_eq!(Duration::from_secs(35996400), parse_counter_time("9999h").unwrap());
    }

    #[test]
    fn test_parse_end_time() {
        let now = OffsetDateTime::now_utc();
        let date = OffsetDateTime::from(parse_end_time("12:00").unwrap());
        let expected_date = now.replace_time(time!(12:00));
        assert_eq!(date.to_ordinal_date(), expected_date.to_ordinal_date());
        assert_eq!(date.to_hms(), expected_date.to_hms());
    }
}
