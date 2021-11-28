use crate::ui;

use std::io;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use crossterm::Result;

use regex::{Regex, RegexSet};

#[derive(Debug, PartialEq)]
pub struct Time(Duration);

pub fn parse_time(s: &str) -> Result<Duration> {
    let re = Regex::new(r"(?:(?P<hours>\d+)h ?)?(?:(?P<minutes>\d+)m ?)?(?:(?P<seconds>\d+)s ?)?$")
        .unwrap();

    // let caps = re.captures(s).ok_or(err: E)?;
    let caps = re.captures(s).unwrap();

    // regex without `^(?!$)` must ensure of matching something
    // add error
    if caps[0].is_empty() {
        panic!("No valid time found.")
    };

    // if exists
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
    Ok(Duration::from_secs(total))
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
            sleep(Duration::from_secs(5));
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_from_string() {
        assert_eq!(Duration::from_secs(7800), parse_time("2h10m").unwrap());
        assert_eq!(Duration::from_secs(70), parse_time("1m10s").unwrap());
        assert_eq!(Duration::from_secs(420), parse_time("5m120s").unwrap());
        assert_eq!(Duration::from_secs(603), parse_time("10m3s").unwrap());
        assert_eq!(Duration::from_secs(35996400), parse_time("9999h").unwrap());
    }
}
