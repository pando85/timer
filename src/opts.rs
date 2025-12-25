use clap::{ArgAction, Parser, Subcommand, crate_authors, crate_description, crate_version};

#[derive(Parser)]
#[command(
    name="timer",
    about = crate_description!(),
    version = crate_version!(),
    author = crate_authors!("\n"),
)]
pub struct Opts {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Repeat countdown infinitely
    #[arg(short, long)]
    pub r#loop: bool,
    /// Disable playing sounds
    #[arg(short, long)]
    pub silence: bool,
    /// Enable terminal bell (useful for visual bell)
    #[arg(short, long)]
    pub terminal_bell: bool,
    /// Remaining time until the alarm sounds. Format: `%Hh %Mm %Ss`.
    /// It also supports `min` for minutes or empty for seconds.
    /// In addition, you can set a target time `%H:%M`. E.g.: 10s, 08:25, 12:00, 3h10m, 15min, 10.
    #[arg(action = ArgAction::Append, num_args = 1)]
    pub time: Vec<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start a stopwatch (counts up from zero)
    Stopwatch,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Opts::command().debug_assert()
}
