use std::fs::{File, OpenOptions};
use std::mem::{size_of, MaybeUninit};
use std::os::unix::io::AsRawFd;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
use libc::{c_void, input_event, write};

const FILE: &str = "/dev/input/by-path/platform-pcspkr-event-spkr";
const EV_SND: u16 = 0x12;
const SND_TONE: u16 = 0x02;

lazy_static! {
    static ref DEVICE: File = OpenOptions::new()
        .append(true)
        .open(FILE)
        .unwrap_or_else(|_| panic!("Could not open {}", FILE));
}

pub fn beep(freq: i32, time: Duration) -> nix::Result<()> {
    send_beep(freq)?;
    sleep(time);
    send_beep(0)
}

fn send_beep(freq: i32) -> nix::Result<()> {
    unsafe {
        let mut e = MaybeUninit::<input_event>::zeroed().assume_init();
        e.type_ = EV_SND;
        e.code = SND_TONE;
        e.value = freq;

        let e_ptr: *mut c_void = &mut e as *mut _ as *mut c_void;
        write(DEVICE.as_raw_fd(), e_ptr, size_of::<input_event>());
    }
    Ok(())
}
