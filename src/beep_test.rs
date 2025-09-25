// Test overlay for beep module - provides in-memory logging for deterministic testing
use super::{Duration, Result, sleep};
use std::sync::{LazyLock, Mutex};

static BEEP_LOG: LazyLock<Mutex<Vec<i32>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub fn beep(freq: i32, time: Duration) -> Result<()> {
    let mut log = BEEP_LOG.lock().unwrap();
    log.push(freq);
    sleep(time);
    log.push(0);
    Ok(())
}

pub fn __beep_log_snapshot() -> Vec<i32> {
    BEEP_LOG.lock().unwrap().clone()
}

pub fn __beep_log_clear() {
    BEEP_LOG.lock().unwrap().clear()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_single_beep_cycle() {
        __beep_log_clear();
        beep(440, Duration::from_millis(1)).unwrap();
        let log = __beep_log_snapshot();
        assert_eq!(log, vec![440, 0]);
    }
}
