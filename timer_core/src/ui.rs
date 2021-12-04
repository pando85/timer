use std::io;
use std::time::Duration;

use crossterm::cursor;
use crossterm::execute;
use crossterm::style;
use crossterm::terminal;
use crossterm::Result;

pub fn draw<W>(w: &mut W, counter: Duration) -> Result<()>
where
    W: io::Write,
{
    execute!(
        w,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;
    let counter_string = time_to_string(counter);
    // get terminal size
    // get centered output
    println!("{}", counter_string);
    Ok(())
}

pub fn set_up_terminal<W>(w: &mut W) -> Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen)
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
    let hours = counter.as_secs() / 3600;
    let minutes = if hours == 0 {
        counter.as_secs() / 60
    } else {
        (counter.as_secs() % (hours * 3600)) / 60
    };

    let seconds = if hours == 0 && minutes == 0 {
        (counter.as_millis() as f64 / 1000.0).round() as u8
    } else {
        (counter.as_millis() as f64 / 1000.0 % (hours * 3600 + minutes * 60) as f64).round() as u8
    };

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_to_string() {
        assert_eq!(
            time_to_string(Duration::from_secs(7800)),
            "2h 10m 0s".to_string()
        );
        assert_eq!(
            time_to_string(Duration::from_secs(15323)),
            "4h 15m 23s".to_string()
        );
        assert_eq!(
            time_to_string(Duration::from_secs(150)),
            "2m 30s".to_string()
        );
        assert_eq!(time_to_string(Duration::from_secs(3)), "3s".to_string());
        assert_eq!(
            time_to_string(Duration::from_millis(2955)),
            "3s".to_string()
        );
        assert_eq!(
            time_to_string(Duration::from_millis(4355)),
            "4s".to_string()
        );
        assert_eq!(time_to_string(Duration::from_secs(0)), "0s".to_string());
    }
}
