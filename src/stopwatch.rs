use crate::Result;
use crate::ui;

use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;
use time::Duration as TimeDuration;

/// Stopwatch state machine
enum State {
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

    fn toggle_pause(self) -> Self {
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

    fn reset() -> Self {
        State::Running {
            start: Instant::now(),
            accumulated: Duration::ZERO,
        }
    }

    fn is_running(&self) -> bool {
        matches!(self, State::Running { .. })
    }
}

/// Run the stopwatch loop
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

        // Only redraw if seconds changed (reduces flickering)
        if current_secs != last_drawn_secs {
            let time_duration = TimeDuration::new(elapsed.as_secs() as i64, 0);
            ui::draw_with_laps(w, time_duration, &laps, state.is_running())?;
            last_drawn_secs = current_secs;
        }

        // Poll for events with a short timeout
        if event::poll(Duration::from_millis(50))?
            && let Event::Key(key_event) = event::read()?
        {
            match handle_key(key_event) {
                Action::Quit => break,
                Action::TogglePause => {
                    state = state.toggle_pause();
                    // Force redraw on state change
                    last_drawn_secs = u64::MAX;
                }
                Action::Lap => {
                    if state.is_running() {
                        laps.push(state.elapsed());
                        // Force redraw on lap
                        last_drawn_secs = u64::MAX;
                    }
                }
                Action::Reset => {
                    laps.clear();
                    state = State::reset();
                    // Force redraw on reset
                    last_drawn_secs = u64::MAX;
                }
                Action::None => {}
            }
        }

        // Small sleep to prevent busy-waiting
        if state.is_running() {
            sleep(Duration::from_millis(10));
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

enum Action {
    Quit,
    TogglePause,
    Lap,
    Reset,
    None,
}

fn handle_key(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Char(' ') | KeyCode::Char('p') => Action::TogglePause,
        KeyCode::Char('l') | KeyCode::Enter => Action::Lap,
        KeyCode::Char('r') => Action::Reset,
        _ => Action::None,
    }
}
