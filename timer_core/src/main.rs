use timer_core::timer;
use timer_core::ui;

use std::io;
use std::time::{Duration, SystemTime};

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

fn main() {
    let opts: Opts = Opts::parse();

    let _counter = timer::Time::new(opts.time.as_str());
    let mut stdout = io::stdout();
    ui::set_up_terminal(&mut stdout).unwrap();

    let now = SystemTime::now();
    let counter = Duration::from_secs(10);
    let end = now + counter;
    let res = timer::countdown(&mut stdout, end);

    ui::restore_terminal(&mut stdout).unwrap();

    if let Err(err) = res {
        println!("{:?}", err)
    }
}
