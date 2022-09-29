use clap::{crate_authors, crate_description, crate_version, Parser};

#[derive(Parser)]
#[clap(
    name="timer",
    about = crate_description!(),
    version = crate_version!(),
    author = crate_authors!("\n"),
)]
pub struct Opts {
    /// Disable playing sounds
    #[clap(short, long)]
    pub silence: bool,
    /// Enable terminal bell (useful for visual bell)
    #[clap(short, long)]
    pub terminal_bell: bool,
    /// Remaining time until the alarm sounds. Format: `%Hh %Mm %Ss`.
    /// It also supports `min` for minutes or empty for seconds.
    /// In addition, it supports a target time `%H:%M`. E.g.: 10s, 12:00, 3h10m, 15min, 10.
    #[clap(multiple_occurrences = true, takes_value = true, number_of_values = 1)]
    pub time: Vec<String>,
}
