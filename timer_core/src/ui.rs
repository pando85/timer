use crate::time::Time;

use std::io;

use crossterm::cursor;
use crossterm::execute;
use crossterm::style;
use crossterm::terminal;
use crossterm::Result;
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
