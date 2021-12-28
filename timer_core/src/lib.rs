pub mod beep;
pub mod constants;
pub mod figlet;
pub mod sound;
mod time;
pub mod timer;
pub mod ui;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
