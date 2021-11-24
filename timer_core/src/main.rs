//! Demonstrates how to read events asynchronously with tokio.
//!
//! cargo run --features="event-stream" --example event-stream-tokio

use std::io;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

use crossterm::cursor;
use crossterm::execute;
use crossterm::style;
use crossterm::terminal;
use crossterm::Result;


fn print_events<W>(w: &mut W) -> Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    let now = SystemTime::now();
    let total = Duration::from_secs(10);

    loop {
        sleep(Duration::from_secs(1));

        let elapsed_time = now.elapsed().unwrap();
        let counter = total.saturating_sub(elapsed_time);
        draw(w, counter)?;
        if counter.is_zero() {
            sleep(Duration::from_secs(5));
            return Ok(());
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

fn main() -> Result<()> {
    let mut stdout = io::stdout();

    let res = print_events(&mut stdout);

    execute!(
        stdout,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
