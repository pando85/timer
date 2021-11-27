use crate::ui;

use std::io;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use crossterm::Result;

use regex::{Regex, RegexSet};

#[derive(Debug, PartialEq)]
pub struct Time(Duration);

impl Time {
    pub fn new(s: &str) -> Result<Self> {
        let re = Regex::new(
            r"(?x)
            (?:(?P<hours>\d+)h ?)?
            (?:(?P<minutes>\d+)m ?)?
            (?:(?P<seconds>\d+)s ?)?$",
        )
        .unwrap();
        // without `^(?!$)` match anything

        //let caps = re.captures(s).ok_or(err: E)?;
        let caps = re.captures(s).unwrap();
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
        Ok(Time(Duration::from_secs(total)))
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
        assert_eq!(Time(Duration::from_secs(7800)), Time::new("2h10m").unwrap());
        assert_eq!(Time(Duration::from_secs(70)), Time::new("1m10s").unwrap());
        assert_eq!(Time(Duration::from_secs(420)), Time::new("5m120s").unwrap());
        assert_eq!(
            Time(Duration::from_secs(35996400)),
            Time::new("9999h").unwrap()
        );
    }
}
