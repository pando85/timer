use std::io;

use crossterm::cursor;
use crossterm::execute;
use crossterm::style;
use crossterm::terminal;
use crossterm::Result;
use time::Duration;

use crate::figlet::Figlet;

pub fn draw<W>(w: &mut W, counter: Duration) -> Result<()>
where
    W: io::Write,
{
    let counter_string = time_to_string(counter);

    let figlet = Figlet::default();
    let figlet_string = figlet.convert(&counter_string);

    let size = terminal::size()?;
    let s = match center(size, &figlet_string) {
        Some(s) => s,
        None => center(size, &counter_string).unwrap(),
    };

    execute!(
        w,
        terminal::SetTitle(&counter_string),
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All),
    )?;
    println!("{}", s);
    Ok(())
}

pub fn set_up_terminal<W>(w: &mut W) -> Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen, cursor::Hide)
}

pub fn restore_terminal<W>(w: &mut W) -> Result<()>
where
    W: io::Write,
{
    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )
}

fn time_to_string(counter: Duration) -> String {
    let total_s = (counter.whole_milliseconds() as f64 / 1000.).round() as u64;
    let hours = total_s / 3600;
    let minutes = if hours == 0 {
        total_s / 60
    } else {
        (total_s % (hours * 3600)) / 60
    };

    let seconds = if hours == 0 && minutes == 0 {
        total_s as u8
    } else {
        (total_s % (hours * 3600 + minutes * 60)) as u8
    };

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
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
    fn test_time_to_string() {
        assert_eq!(
            time_to_string(Duration::seconds(7800)),
            "2h 10m 0s".to_string()
        );
        assert_eq!(
            time_to_string(Duration::seconds(15323)),
            "4h 15m 23s".to_string()
        );
        assert_eq!(time_to_string(Duration::seconds(150)), "2m 30s".to_string());
        assert_eq!(time_to_string(Duration::seconds(3)), "3s".to_string());
        assert_eq!(
            time_to_string(Duration::milliseconds(2955)),
            "3s".to_string()
        );
        assert_eq!(
            time_to_string(Duration::milliseconds(4355)),
            "4s".to_string()
        );
        assert_eq!(
            time_to_string(Duration::milliseconds(59999)),
            "1m 0s".to_string()
        );
        assert_eq!(
            time_to_string(Duration::milliseconds(7199999)),
            "2h 0m 0s".to_string()
        );
        assert_eq!(time_to_string(Duration::seconds(0)), "0s".to_string());
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
