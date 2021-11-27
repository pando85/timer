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
    println!("{:?}", counter);
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
