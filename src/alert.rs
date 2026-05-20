use crate::Result;

use crate::beep::beep;
use crate::constants::{BEEP_DELAY, BEEP_DURATION, BEEP_FREQ, BEEP_REPETITIONS, SOUND_START_DELAY};
use crate::sound::Sound;

use std::io::Write;
#[cfg(test)]
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration as stdDuration;

pub trait Alert {
    fn play(&self) -> Result<()>;
}

pub struct BeepAlert;

impl Alert for BeepAlert {
    fn play(&self) -> Result<()> {
        for _ in 0..BEEP_REPETITIONS {
            sleep(stdDuration::from_millis(SOUND_START_DELAY));
            if beep(BEEP_FREQ, stdDuration::from_millis(BEEP_DURATION)).is_err() {
                sleep(stdDuration::from_millis(BEEP_DURATION));
            }
            let remaining_delay = BEEP_DELAY.saturating_sub(SOUND_START_DELAY);
            if remaining_delay > 0 {
                sleep(stdDuration::from_millis(remaining_delay));
            }
        }
        Ok(())
    }
}

pub struct SoundAlert;

impl Alert for SoundAlert {
    fn play(&self) -> Result<()> {
        let sound = Sound::new()?;
        for _ in 0..BEEP_REPETITIONS {
            sound.play()?;
            sleep(stdDuration::from_millis(BEEP_DELAY));
        }
        Ok(())
    }
}

#[cfg(test)]
pub struct SilentAlert;

#[cfg(test)]
impl Alert for SilentAlert {
    fn play(&self) -> Result<()> {
        Ok(())
    }
}

pub fn write_terminal_bell<W: Write>(w: &mut W) -> Result<()> {
    w.write_all(b"\x07")?;
    w.flush()?;
    Ok(())
}

#[cfg(test)]
#[derive(Clone)]
pub struct AlertCallLog {
    pub calls: Vec<&'static str>,
}

#[cfg(test)]
impl AlertCallLog {
    pub fn new() -> Self {
        Self { calls: Vec::new() }
    }
}

#[cfg(test)]
pub struct MockAlert {
    log: Arc<Mutex<AlertCallLog>>,
    name: &'static str,
}

#[cfg(test)]
impl MockAlert {
    pub fn new(log: Arc<Mutex<AlertCallLog>>, name: &'static str) -> Self {
        Self { log, name }
    }
}

#[cfg(test)]
impl Alert for MockAlert {
    fn play(&self) -> Result<()> {
        self.log.lock().unwrap().calls.push(self.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silent_alert_returns_ok() {
        assert!(SilentAlert.play().is_ok());
    }

    #[test]
    fn write_terminal_bell_writes_bell_char() {
        let mut buf = Vec::new();
        write_terminal_bell(&mut buf).unwrap();
        assert_eq!(buf, b"\x07");
    }

    #[test]
    fn mock_alert_records_calls() {
        let log = Arc::new(Mutex::new(AlertCallLog::new()));
        let alert = MockAlert::new(Arc::clone(&log), "beep");
        alert.play().unwrap();
        alert.play().unwrap();
        assert_eq!(log.lock().unwrap().calls, vec!["beep", "beep"]);
    }

    #[test]
    fn mock_alert_records_different_names() {
        let log = Arc::new(Mutex::new(AlertCallLog::new()));
        let a = MockAlert::new(Arc::clone(&log), "beep");
        let b = MockAlert::new(Arc::clone(&log), "sound");
        a.play().unwrap();
        b.play().unwrap();
        a.play().unwrap();
        assert_eq!(log.lock().unwrap().calls, vec!["beep", "sound", "beep"]);
    }
}
