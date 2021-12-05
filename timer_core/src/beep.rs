use std::fs::{File, OpenOptions};
use std::mem::{size_of, MaybeUninit};
use std::os::unix::io::AsRawFd;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
use libc::{c_void, input_event, write};
use nix::{ioctl_write_int_bad, Result};

const DEVICE_PATHS: [&str; 2] = [
    "/dev/input/by-path/platform-pcspkr-event-spkr",
    "/dev/console",
];
const EV_SND: u16 = 0x12;
const SND_TONE: u16 = 0x02;
const KIOCSOUND: u64 = 0x4B2F;
const TIMER_FREQUENCY: u32 = 1193182;

lazy_static! {
    static ref DEVICE: File = DEVICE_PATHS
        .into_iter()
        .find_map(|d| OpenOptions::new().append(true).open(d).ok())
        .unwrap_or_else(|| panic!("No device found in: {:?}", DEVICE_PATHS));
}

ioctl_write_int_bad!(kiocsound, KIOCSOUND);

pub fn beep(freq: i32, time: Duration) -> Result<()> {
    let driver = match unsafe { kiocsound(DEVICE.as_raw_fd(), 0) } {
        Ok(_) => Driver {
            beep: driver_console,
        },
        Err(_) => Driver { beep: driver_evdev },
    };

    driver.beep(freq)?;
    sleep(time);
    driver.beep(0)
}

struct Driver {
    beep: fn(freq: i32) -> Result<()>,
}

impl Driver {
    pub fn beep(&self, freq: i32) -> Result<()> {
        (self.beep)(freq)
    }
}

fn driver_evdev(freq: i32) -> Result<()> {
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

fn driver_console(freq: i32) -> nix::Result<()> {
    let period_in_clock_cycles = TIMER_FREQUENCY.checked_div(freq as u32).unwrap_or(0);

    unsafe {
        kiocsound(DEVICE.as_raw_fd(), period_in_clock_cycles as i32)?;
    }
    Ok(())
}
