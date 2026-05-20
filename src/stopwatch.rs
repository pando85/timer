use crate::Result;
use crate::ui;

use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;
use time::Duration as TimeDuration;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum State {
    Running {
        start: Instant,
        accumulated: Duration,
    },
    Paused {
        accumulated: Duration,
    },
}

impl State {
    fn elapsed(&self) -> Duration {
        match self {
            State::Running { start, accumulated } => *accumulated + start.elapsed(),
            State::Paused { accumulated } => *accumulated,
        }
    }

    pub(crate) fn toggle_pause(self) -> Self {
        match self {
            State::Running { start, accumulated } => State::Paused {
                accumulated: accumulated + start.elapsed(),
            },
            State::Paused { accumulated } => State::Running {
                start: Instant::now(),
                accumulated,
            },
        }
    }

    pub(crate) fn reset() -> Self {
        State::Running {
            start: Instant::now(),
            accumulated: Duration::ZERO,
        }
    }

    pub(crate) fn is_running(&self) -> bool {
        matches!(self, State::Running { .. })
    }
}

pub fn run<W: io::Write>(w: &mut W) -> Result<()> {
    terminal::enable_raw_mode()?;

    let mut state = State::Running {
        start: Instant::now(),
        accumulated: Duration::ZERO,
    };
    let mut laps: Vec<Duration> = Vec::new();
    let mut last_drawn_secs: u64 = u64::MAX;

    loop {
        let elapsed = state.elapsed();
        let current_secs = elapsed.as_secs();

        if current_secs != last_drawn_secs {
            let time_duration = TimeDuration::new(elapsed.as_secs() as i64, 0);
            ui::draw_with_laps(w, time_duration, &laps, state.is_running())?;
            last_drawn_secs = current_secs;
        }

        if event::poll(Duration::from_millis(50))?
            && let Event::Key(key_event) = event::read()?
        {
            match handle_key(key_event) {
                Action::Quit => break,
                Action::TogglePause => {
                    state = state.toggle_pause();
                    last_drawn_secs = u64::MAX;
                }
                Action::Lap => {
                    if state.is_running() {
                        laps.push(state.elapsed());
                        last_drawn_secs = u64::MAX;
                    }
                }
                Action::Reset => {
                    laps.clear();
                    state = State::reset();
                    last_drawn_secs = u64::MAX;
                }
                Action::None => {}
            }
        }

        if state.is_running() {
            sleep(Duration::from_millis(10));
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum Action {
    Quit,
    TogglePause,
    Lap,
    Reset,
    None,
}

pub(crate) fn handle_key(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Char(' ') | KeyCode::Char('p') => Action::TogglePause,
        KeyCode::Char('l') | KeyCode::Enter => Action::Lap,
        KeyCode::Char('r') => Action::Reset,
        _ => Action::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_handle_key_space() {
        let key = KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE);
        assert_eq!(handle_key(key), Action::TogglePause);
    }

    #[test]
    fn test_handle_key_p() {
        let key = KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE);
        assert_eq!(handle_key(key), Action::TogglePause);
    }

    #[test]
    fn test_handle_key_l() {
        let key = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE);
        assert_eq!(handle_key(key), Action::Lap);
    }

    #[test]
    fn test_handle_key_enter() {
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        assert_eq!(handle_key(key), Action::Lap);
    }

    #[test]
    fn test_handle_key_r() {
        let key = KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE);
        assert_eq!(handle_key(key), Action::Reset);
    }

    #[test]
    fn test_handle_key_q() {
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        assert_eq!(handle_key(key), Action::Quit);
    }

    #[test]
    fn test_handle_key_ctrl_c() {
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(handle_key(key), Action::Quit);
    }

    #[test]
    fn test_handle_key_unknown() {
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
        assert_eq!(handle_key(key), Action::None);
    }

    #[test]
    fn test_state_is_running_running() {
        let state = State::Running {
            start: Instant::now(),
            accumulated: Duration::ZERO,
        };
        assert!(state.is_running());
    }

    #[test]
    fn test_state_is_running_paused() {
        let state = State::Paused {
            accumulated: Duration::ZERO,
        };
        assert!(!state.is_running());
    }

    #[test]
    fn test_state_reset_is_running() {
        let state = State::reset();
        assert!(state.is_running());
    }

    #[test]
    fn test_state_toggle_pause_running_to_paused() {
        let initial_state = State::Running {
            start: Instant::now(),
            accumulated: Duration::ZERO,
        };
        let toggled_state = initial_state.toggle_pause();
        assert!(!toggled_state.is_running());
    }

    #[test]
    fn test_state_toggle_pause_paused_to_running() {
        let initial_state = State::Paused {
            accumulated: Duration::ZERO,
        };
        let toggled_state = initial_state.toggle_pause();
        assert!(toggled_state.is_running());
    }
}
