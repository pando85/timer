use crate::figlet::Figlet;

use lazy_static::lazy_static;
use time::Duration;

lazy_static! {
    static ref FIGLET: Figlet = Figlet::default();
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct Time {
    hours: u64,
    minutes: u8,
    seconds: u8,
}

impl From<&Duration> for Time {
    fn from(duration: &Duration) -> Self {
        let total_s = (duration.whole_milliseconds() as f64 / 1000.).round() as u64;
        let hours = total_s / 3600;
        let minutes = if hours == 0 {
            (total_s / 60) as u8
        } else {
            ((total_s % (hours * 3600)) / 60) as u8
        };

        let seconds = if hours == 0 && minutes == 0 {
            total_s as u8
        } else {
            (total_s % (hours * 3600 + minutes as u64 * 60)) as u8
        };
        Time {
            hours,
            minutes,
            seconds,
        }
    }
}

impl Time {
    fn format_ruled(&self, omit_m: bool, omit_s: bool) -> String {
        if self.hours > 0 {
            match (omit_m, omit_s) {
                (true, _) => format!("{}h", self.hours),
                (false, true) => format!("{}h {}m", self.hours, self.minutes),
                (false, false) => format!("{}h {}m {}s", self.hours, self.minutes, self.seconds),
            }
        } else if self.minutes > 0 {
            match omit_s {
                true => format!("{}m", self.minutes),
                false => format!("{}m {}s", self.minutes, self.seconds),
            }
        } else {
            format!("{}s", self.seconds)
        }
    }

    pub fn format(&self) -> String {
        self.format_ruled(false, false)
    }

    fn try_render(
        &self,
        size: (u16, u16),
        omit_m: bool,
        omit_s: bool,
        is_centered: bool,
    ) -> Option<String> {
        let s = self.format_ruled(omit_m, omit_s);
        let figlet_string = FIGLET.convert(&s);

        if is_centered {
            center(size, &figlet_string)
        } else {
            Some(s)
        }
    }

    pub fn render(&self, size: (u16, u16)) -> String {
        match self.try_render(size, false, false, true) {
            Some(s) => s,
            None => match self.try_render(size, false, true, true) {
                Some(s) => s,
                None => match self.try_render(size, true, true, true) {
                    Some(s) => s,
                    // safe unwrap: if is not centered return string without figlet
                    None => self.try_render(size, false, false, false).unwrap(),
                },
            },
        }
    }
}

fn center(size: (u16, u16), s: &str) -> Option<String> {
    let s_size = get_size(s);
    let distance = get_distance_from_top_left(size, s_size)?;
    let horizontal_space = vec![" "; distance.0 as usize].join("");
    let vertical_space = vec!["\n"; distance.1 as usize].join("");
    let horizontal_centered_s = s
        .split('\n')
        .map(|s| horizontal_space.clone() + s)
        .collect::<Vec<String>>()
        .join("\n");
    Some(vertical_space + &horizontal_centered_s)
}

/// Return text size (columns, rows)
fn get_size(s: &str) -> (u16, u16) {
    let v: Vec<&str> = s.split('\n').collect();
    let columns = v
        .clone()
        .into_iter()
        .max_by(|a, b| a.chars().count().cmp(&b.chars().count()))
        .unwrap()
        .chars()
        .count();
    let rows = v.len();
    (columns as u16, rows as u16)
}

/// Return distance from top left corner (columns, rows)
fn get_distance_from_top_left(
    terminal_size: (u16, u16),
    text_size: (u16, u16),
) -> Option<(u16, u16)> {
    let get_distance = |x: u16, y: u16| -> Option<u16> { Some(x.checked_sub(y)? / 2) };
    Some((
        get_distance(terminal_size.0, text_size.0)?,
        get_distance(terminal_size.1, text_size.1)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_from() {
        assert_eq!(
            Time::from(&Duration::seconds(7800)),
            Time {
                hours: 2,
                minutes: 10,
                seconds: 0
            }
        );
        assert_eq!(
            Time::from(&Duration::seconds(15323)),
            Time {
                hours: 4,
                minutes: 15,
                seconds: 23
            }
        );
        assert_eq!(
            Time::from(&Duration::seconds(150)),
            Time {
                hours: 0,
                minutes: 2,
                seconds: 30
            }
        );
        assert_eq!(
            Time::from(&Duration::seconds(3)),
            Time {
                hours: 0,
                minutes: 0,
                seconds: 3
            }
        );
        assert_eq!(
            Time::from(&Duration::milliseconds(2955)),
            Time {
                hours: 0,
                minutes: 0,
                seconds: 3
            }
        );
        assert_eq!(
            Time::from(&Duration::milliseconds(4355)),
            Time {
                hours: 0,
                minutes: 0,
                seconds: 4
            }
        );
        assert_eq!(
            Time::from(&Duration::milliseconds(59999)),
            Time {
                hours: 0,
                minutes: 1,
                seconds: 0
            }
        );
        assert_eq!(
            Time::from(&Duration::milliseconds(7199999)),
            Time {
                hours: 2,
                minutes: 0,
                seconds: 0
            }
        );
        assert_eq!(
            Time::from(&Duration::seconds(0)),
            Time {
                hours: 0,
                minutes: 0,
                seconds: 0
            }
        );
    }

    #[test]
    fn test_get_size() {
        assert_eq!(get_size("\n\n\n\n\n"), (0, 6));
        assert_eq!(get_size("123456789"), (9, 1));
        assert_eq!(get_size("123\n1234\n1234\n"), (4, 4));
        assert_eq!(get_size("1\n1\n1234567"), (7, 3));
        assert_eq!(get_size(""), (0, 1));
    }

    #[test]
    fn test_get_distance_from_top_left() {
        assert_eq!(
            get_distance_from_top_left((10, 10), (5, 5)).unwrap(),
            (2, 2)
        );
        assert_eq!(
            get_distance_from_top_left((100, 100), (20, 20)).unwrap(),
            (40, 40)
        );
        assert_eq!(get_distance_from_top_left((100, 100), (2000, 2000)), None);
    }
}
