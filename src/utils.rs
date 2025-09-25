#[cfg(not(feature = "test-beep"))]
use crate::Result;
#[cfg(not(feature = "test-beep"))]
use std::sync::mpsc::{Receiver, channel};
#[cfg(not(feature = "test-beep"))]
use std::thread;
#[cfg(not(feature = "test-beep"))]
use std::time::Duration;

#[cfg(not(feature = "test-beep"))]
pub struct JoinWithTimeout<T> {
    handle: thread::JoinHandle<T>,
    signal: Receiver<()>,
}

#[cfg(not(feature = "test-beep"))]
impl<T> JoinWithTimeout<T> {
    pub fn join(self, timeout: Duration) -> Result<T> {
        self.signal.recv_timeout(timeout)?;
        Ok(self.handle.join().unwrap())
    }
}

#[cfg(not(feature = "test-beep"))]
pub fn spawn_thread<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(
    f: F,
) -> JoinWithTimeout<T> {
    let (send, recv) = channel();
    let t = thread::spawn(move || {
        let x = f();
        send.send(()).unwrap();
        x
    });
    JoinWithTimeout {
        handle: t,
        signal: recv,
    }
}
