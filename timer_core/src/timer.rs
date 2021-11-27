use crate::ui;

use std::io;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use crossterm::Result;

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
