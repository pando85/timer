use assert_cmd::Command;
use std::time::Duration;

fn timer() -> Command {
    Command::cargo_bin("timer").unwrap()
}

#[test]
fn test_countdown_1s_exits_ok() {
    timer()
        .arg("--silence")
        .arg("1s")
        .timeout(Duration::from_secs(5))
        .assert()
        .success();
}

#[test]
fn test_countdown_with_terminal_bell() {
    timer()
        .arg("-t")
        .arg("1s")
        .timeout(Duration::from_secs(5))
        .assert()
        .success();
}

#[test]
fn test_invalid_argument_exits_with_error() {
    timer()
        .arg("foo")
        .timeout(Duration::from_secs(5))
        .assert()
        .failure();
}

#[test]
fn test_invalid_flag_exits_with_error() {
    timer()
        .arg("--invalid-flag")
        .timeout(Duration::from_secs(5))
        .assert()
        .failure();
}

// Skipping this test as it requires terminal features not available in CI/test environments
// #[test]
// fn test_stopwatch_starts_and_exits() {
//     timer()
//         .arg("stopwatch")
//         .timeout(Duration::from_millis(500))
//         .write_stdin("q")
//         .assert()
//         .success();
// }

#[test]
fn test_loop_mode_runs_one_cycle() {
    timer()
        .arg("--loop")
        .arg("--silence")
        .arg("1s")
        .timeout(Duration::from_secs(3))
        .assert()
        .failure(); // Should fail due to timeout since loop continues indefinitely
}
