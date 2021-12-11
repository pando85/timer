use timer_core::beep::beep;
use timer_core::timer;
use timer_core::ui;

use std::io;
use std::process::exit;
use std::thread;
use std::time::Duration;

use clap::{crate_authors, crate_description, crate_version, AppSettings, IntoApp, Parser};
use signal_hook::{consts::signal::*, iterator::Signals};
use time::OffsetDateTime;

#[derive(Parser)]
#[clap(
    name="timer",
    about = crate_description!(),
    version = crate_version!(),
    author = crate_authors!("\n"),
    setting = AppSettings::ArgRequiredElseHelp,
)]
struct Opts {
    /// Remaining time until the alarm sounds. Format: `%Hh %Mm %Ss`.
    /// It also supports `min` for minutes or empty for seconds.
    /// In addition, it supports a target time `%H:%M`. E.g.: 10s, 12:00, 3h10m, 15min, 10.
    #[clap(multiple_occurrences = true, takes_value = true, number_of_values = 1)]
    time: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_time = opts.time.join(" ");
    let end = match timer::parse_counter_time(input_time.as_str()) {
        Some(counter) => {
            let now = OffsetDateTime::now_utc();
            now + counter
        }
        None => match timer::parse_end_time(input_time.as_str()) {
            Some(x) => x,
            None => {
                Opts::into_app().print_help().unwrap();
                eprintln!(
                    "Error: Invalid value: Unable to parse TIME value '{}'",
                    input_time.as_str()
                );
                exit(1);
            }
        },
    };

    let mut stdout = io::stdout();
    ui::set_up_terminal(&mut stdout).unwrap();

    let thread_join_handle = thread::spawn(move || {
        match timer::countdown(&mut stdout, end) {
            Ok(_) => {
                ui::restore_terminal(&mut stdout).unwrap();
            }
            Err(e) => {
                ui::restore_terminal(&mut stdout).unwrap();
                eprintln!("Error: {:?}", e);
            }
        };
        exit(0)
    });

    let mut signals = Signals::new(&[SIGWINCH, SIGTERM, SIGINT, SIGQUIT]).unwrap();
    let mut stdout_signals_thread = io::stdout();

    for signal in &mut signals {
        match signal {
            SIGWINCH => {
                timer::resize_term(&mut stdout_signals_thread, end).unwrap();
            }

            SIGTERM | SIGINT | SIGQUIT => {
                // ensure beep stops
                let _ = beep(0, Duration::from_secs(0));
                ui::restore_terminal(&mut stdout_signals_thread).unwrap();
                exit(1);
            }
            _ => unreachable!(),
        }
    }

    thread_join_handle.join().unwrap();
}
