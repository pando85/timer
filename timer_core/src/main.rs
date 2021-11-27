use timer_core::timer;
use timer_core::ui;

use std::io;
use std::time::{Duration, SystemTime};

fn main() {
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
