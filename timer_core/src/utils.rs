use crate::Result;

use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

pub struct JoinWithTimeout<T> {
    handle: thread::JoinHandle<T>,
    signal: Receiver<()>,
}

impl<T> JoinWithTimeout<T> {
    pub fn join(self, timeout: Duration) -> Result<T> {
        self.signal.recv_timeout(timeout)?;
        Ok(self.handle.join().unwrap())
    }
}

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
