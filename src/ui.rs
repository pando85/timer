use crate::Result;
use crate::time::Time;

use std::io;
use std::time::Duration as StdDuration;

use crossterm::cursor;
use crossterm::execute;
use crossterm::style;
use crossterm::terminal;
use time::Duration;

pub fn draw<W>(w: &mut W, counter: Duration) -> Result<()>
where
    W: io::Write,
{
    let counter_time = Time::from(&counter);
    let size = terminal::size()?;

    let s = counter_time.render(size);
    execute!(
        w,
        terminal::SetTitle(&counter_time.format()),
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All),
    )?;
    println!("{s}");
    Ok(())
}

pub fn draw_with_laps<W>(
    w: &mut W,
    elapsed: Duration,
    laps: &[StdDuration],
    is_running: bool,
) -> Result<()>
where
    W: io::Write,
{
    let elapsed_time = Time::from(&elapsed);
    let size = terminal::size()?;

    // Reserve space for laps and status at the bottom
    let laps_lines = if laps.is_empty() { 0 } else { 2 };
    let status_lines = 2;
    let reserved_lines = laps_lines + status_lines;
    let adjusted_size = (size.0, size.1.saturating_sub(reserved_lines));

    let status = if is_running { "RUNNING" } else { "PAUSED" };
    let title = format!("Stopwatch - {} - {}", elapsed_time.format(), status);

    execute!(
        w,
        terminal::SetTitle(&title),
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All),
    )?;

    // Draw the main time display (dimmed when paused)
    let s = elapsed_time.render(adjusted_size);
    if !is_running {
        execute!(w, style::SetForegroundColor(style::Color::DarkGrey))?;
    }
    // In raw mode, we need \r\n for proper line breaks
    for line in s.split('\n') {
        execute!(
            w,
            crossterm::style::Print(line),
            crossterm::style::Print("\r\n")
        )?;
    }
    if !is_running {
        execute!(w, style::SetForegroundColor(style::Color::Reset))?;
    }

    // Draw laps at the bottom
    if !laps.is_empty() {
        let laps_str = format_laps(laps);
        let laps_y = size.1.saturating_sub(reserved_lines);
        execute!(
            w,
            cursor::MoveTo(0, laps_y),
            crossterm::style::Print(format!("Laps: {}\r\n", laps_str))
        )?;
    }

    // Draw status/controls at the very bottom
    let status_y = size.1.saturating_sub(1);
    execute!(w, cursor::MoveTo(0, status_y))?;

    // Draw legend with bold key letters
    // Show Play/Pause based on current state
    let action = if is_running { "ause" } else { "lay" };

    execute!(
        w,
        style::SetAttribute(style::Attribute::Bold),
        style::Print("P"),
        style::SetAttribute(style::Attribute::Reset),
        style::Print(action),
        style::SetForegroundColor(style::Color::DarkGrey),
        style::Print("(space)"),
        style::SetForegroundColor(style::Color::Reset),
        style::Print("  "),
        style::SetAttribute(style::Attribute::Bold),
        style::Print("L"),
        style::SetAttribute(style::Attribute::Reset),
        style::Print("ap  "),
        style::SetAttribute(style::Attribute::Bold),
        style::Print("R"),
        style::SetAttribute(style::Attribute::Reset),
        style::Print("eset  "),
        style::SetAttribute(style::Attribute::Bold),
        style::Print("Q"),
        style::SetAttribute(style::Attribute::Reset),
        style::Print("uit"),
    )?;

    w.flush()?;
    Ok(())
}

fn format_laps(laps: &[StdDuration]) -> String {
    laps.iter()
        .enumerate()
        .map(|(i, lap)| {
            let secs = lap.as_secs();
            let hours = secs / 3600;
            let minutes = (secs % 3600) / 60;
            let seconds = secs % 60;

            let time_str = if hours > 0 {
                format!("{}h {}m {}s", hours, minutes, seconds)
            } else if minutes > 0 {
                format!("{}m {}s", minutes, seconds)
            } else {
                format!("{}s", seconds)
            };

            format!("[{}] {}", i + 1, time_str)
        })
        .collect::<Vec<_>>()
        .join("  ")
}

pub fn set_up_terminal<W>(w: &mut W) -> std::io::Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen, cursor::Hide)
}

pub fn restore_terminal<W>(w: &mut W) -> std::io::Result<()>
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
