// Not a shebang: leading comment prevents pre-commit hook from misclassifying inner attribute.
#![allow(clippy::assertions_on_constants)]

#[cfg(not(feature = "test-beep"))]
use crate::constants::BEEP_DURATION;
#[cfg(not(feature = "test-beep"))]
use crate::constants::PLAY_TIMEOUT;
use crate::constants::{BEEP_DELAY, BEEP_REPETITIONS, SOUND_START_DELAY};

#[test]
fn sound_start_delay_not_exceed_beep_delay() {
    assert!(SOUND_START_DELAY <= BEEP_DELAY);
}

#[cfg(not(feature = "test-beep"))]
#[test]
fn play_timeout_is_sufficient() {
    let total_one_cycle = BEEP_DURATION + BEEP_DELAY; // ms for one repetition (approx)
    let expected_min = (BEEP_REPETITIONS as u64) * total_one_cycle;
    assert!(
        PLAY_TIMEOUT >= expected_min,
        "PLAY_TIMEOUT insufficient: {} < {}",
        PLAY_TIMEOUT,
        expected_min
    );
}

#[test]
fn repetitions_positive() {
    assert!(BEEP_REPETITIONS > 0);
}
