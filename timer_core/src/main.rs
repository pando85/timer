use std::io;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use crossterm::cursor;
use crossterm::execute;
use crossterm::style;
use crossterm::terminal;
use crossterm::Result;

fn countdown<W>(w: &mut W, end: SystemTime) -> Result<()>
where
    W: io::Write,
{
    match end.duration_since(SystemTime::now()) {
        Ok(counter) => {
            draw(w, counter)?;
            sleep(Duration::from_secs(1));
            countdown(w, end)
        }
        Err(_) => {
            draw(w, Duration::ZERO)?;
            sleep(Duration::from_secs(5));
            Ok(())
        }
    }
}

fn draw<W>(w: &mut W, counter: Duration) -> Result<()>
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

fn set_up_terminal<W>(w: &mut W) -> Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen)
}

fn restore_terminal<W>(w: &mut W) -> Result<()>
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

fn main() {
    let mut stdout = io::stdout();

    set_up_terminal(&mut stdout).unwrap();

    let now = SystemTime::now();
    let counter = Duration::from_secs(10);
    let end = now + counter;
    let res = countdown(&mut stdout, end);

    restore_terminal(&mut stdout).unwrap();

    if let Err(err) = res {
        println!("{:?}", err)
    }
}
