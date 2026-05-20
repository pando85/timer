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

#[derive(Subcommand, PartialEq, Debug)]
pub enum Command {
    /// Start a stopwatch (counts up from zero)
    Stopwatch,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Opts::command().debug_assert()
}

#[test]
fn test_opts_valid_duration() {
    let args = ["timer", "5m"];
    let opts = Opts::try_parse_from(args).unwrap();
    assert_eq!(opts.time, vec!["5m"]);
}

#[test]
fn test_opts_valid_hms() {
    let args = ["timer", "1h30m45s"];
    let opts = Opts::try_parse_from(args).unwrap();
    assert_eq!(opts.time, vec!["1h30m45s"]);
}

#[test]
fn test_opts_stopwatch() {
    let args = ["timer", "stopwatch"];
    let opts = Opts::try_parse_from(args).unwrap();
    assert_eq!(opts.command, Some(Command::Stopwatch));
}

#[test]
fn test_opts_silence_flag() {
    let args = ["timer", "--silence", "10s"];
    let opts = Opts::try_parse_from(args).unwrap();
    assert!(opts.silence);
}

#[test]
fn test_opts_short_flags() {
    let args = ["timer", "-l", "-s", "5m"];
    let opts = Opts::try_parse_from(args).unwrap();
    assert!(opts.r#loop);
    assert!(opts.silence);
}

#[test]
fn test_opts_invalid_flag() {
    let args = ["timer", "--invalid"];
    let result = Opts::try_parse_from(args);
    assert!(result.is_err());
}

#[test]
fn test_opts_defaults() {
    let args = ["timer", "10s"];
    let opts = Opts::try_parse_from(args).unwrap();
    assert!(!opts.silence);
    assert!(!opts.r#loop);
    assert!(!opts.terminal_bell);
}
