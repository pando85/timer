use std::time::Duration;
use timer_core::beep::beep;
use timer_core::timer;
use timer_core::ui;

use signal_hook::{consts::signal::*, iterator::Signals};
use std::io;
use std::process::exit;
use std::thread;
use std::time::SystemTime;

use clap::{crate_authors, crate_description, crate_version, Parser};

#[derive(Parser)]
#[clap(
    name="timer",
    about = crate_description!(),
    version = crate_version!(),
    author = crate_authors!("\n"),
)]
struct Opts {
    /// Remaining time until the alarm sounds
    time: String,
}

fn main() -> Result<(), io::Error> {
    let opts: Opts = Opts::parse();

    let counter = timer::parse_time(opts.time.as_str()).unwrap();
    let mut stdout = io::stdout();
    ui::set_up_terminal(&mut stdout).unwrap();

    let mut signals = Signals::new(&[SIGWINCH, SIGTERM, SIGINT, SIGQUIT]).unwrap();

    let now = SystemTime::now();
    let end = now + counter;

    thread::spawn(move || {
        match timer::countdown(&mut stdout, end) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        };
        ui::restore_terminal(&mut stdout).unwrap();
    });

    let mut stdout = io::stdout();

    for signal in signals.forever() {
        match signal {
            SIGWINCH => {
                timer::resize_term(&mut stdout, end)?;
            }

            SIGTERM | SIGINT | SIGQUIT => {
                // ensure beep stops
                beep(0, Duration::from_secs(0)).unwrap();
                ui::restore_terminal(&mut stdout).unwrap();
                exit(1);
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
