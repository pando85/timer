use timer_core::beep::beep;
use timer_core::opts::Opts;
use timer_core::timer;
use timer_core::ui;

use std::io;
use std::process::exit;
use std::thread;
use std::time::Duration;

use clap::{IntoApp, Parser};
use signal_hook::{consts::signal::*, iterator::Signals};
use time::OffsetDateTime;

fn parse_time(input_time: &str) -> Option<OffsetDateTime> {
    match timer::parse_counter_time(input_time) {
        Some(counter) => {
            let now = OffsetDateTime::now_utc();
            Some(now + counter)
        }
        None => timer::parse_end_time(input_time),
    }
}

fn handle_countdown<W: io::Write>(w: &mut W, end: OffsetDateTime, opts: &Opts) {
    match timer::countdown(w, end, opts) {
        Ok(_) => {
            ui::restore_terminal(w).unwrap();
        }
        Err(e) => {
            ui::restore_terminal(w).unwrap();
            eprintln!("Error: {:?}", e);
        }
    };
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_time = opts.time.join(" ");
    let end = match parse_time(input_time.as_str()) {
        Some(x) => x,
        None => {
            Opts::command().print_help().unwrap();
            eprintln!(
                "Error: Invalid value: Unable to parse TIME value '{}'",
                input_time.as_str()
            );
            exit(1);
        }
    };

    let mut stdout = io::stdout();
    ui::set_up_terminal(&mut stdout).unwrap();

    let thread_join_handle = thread::spawn(move || {
        handle_countdown(&mut stdout, end, &opts);
        if opts.r#loop {
            loop {
                // safe unwrap: previously checked
                let new_end = parse_time(input_time.as_str()).unwrap();
                handle_countdown(&mut stdout, new_end, &opts);
            }
        }
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
