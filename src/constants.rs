pub const BEEP_DELAY: u64 = 100;
pub const BEEP_DURATION: u64 = 400;
pub const BEEP_REPETITIONS: usize = 5;
pub const BEEP_FREQ: i32 = 440;
#[cfg(not(feature = "test-beep"))]
pub const PLAY_TIMEOUT: u64 = 2 * BEEP_REPETITIONS as u64 * (BEEP_DURATION + BEEP_DELAY);
pub const SOUND_START_DELAY: u64 = 100;
