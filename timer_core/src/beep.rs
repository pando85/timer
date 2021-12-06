use crate::Result;

use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::mem::{size_of, MaybeUninit};
use std::os::unix::io::AsRawFd;
use std::thread::sleep;
use std::time::Duration;

use glob::glob;
use lazy_static::lazy_static;
use libc::{c_void, input_event, write};
use nix::ioctl_write_int_bad;

const DEVICE_PATHS: [&str; 2] = [
    "/dev/input/by-path/platform-pcspkr-event-spkr",
    "/dev/console",
];
const EV_SND: u16 = 0x12;
const SND_TONE: u16 = 0x02;
const KIOCSOUND: u64 = 0x4B2F;
const TIMER_FREQUENCY: u32 = 1193182;

lazy_static! {
    static ref DEVICE: Option<File> = get_device();
}

fn get_device() -> Option<File> {
    let strings_from_glob = |x| {
        glob(x)
            .unwrap()
            .map(|x| x.unwrap().to_str().unwrap().to_string())
            .collect::<Vec<String>>()
    };
    let all_ttys = strings_from_glob("/dev/tty[0-9]*");
    let all_vcs = strings_from_glob("/dev/vc/[0-9]*");
    DEVICE_PATHS
        .into_iter()
        .map(|s| s.to_string())
        .chain(all_ttys)
        .chain(all_vcs)
        .find_map(|d| OpenOptions::new().append(true).open(d).ok())
}

ioctl_write_int_bad!(kiocsound, KIOCSOUND);

#[derive(Debug, Clone)]
struct DeviceError;

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No device found in: {:?}", DEVICE_PATHS)
    }
}

impl Error for DeviceError {}

pub fn beep(freq: i32, time: Duration) -> Result<()> {
    let device = DEVICE.as_ref().ok_or(DeviceError)?;
    let driver = match unsafe { kiocsound(device.as_raw_fd(), 0) } {
        Ok(_) => Driver {
            device,
            beep: driver_console,
        },
        Err(_) => Driver {
            device,
            beep: driver_evdev,
        },
    };

    driver.beep(freq)?;
    sleep(time);
    driver.beep(0)?;
    Ok(())
}

struct Driver {
    device: &'static File,
    beep: fn(dev: &File, freq: i32) -> nix::Result<()>,
}

impl Driver {
    pub fn beep(&self, freq: i32) -> nix::Result<()> {
        (self.beep)(self.device, freq)
    }
}

fn driver_evdev(dev: &File, freq: i32) -> nix::Result<()> {
    unsafe {
        let mut e = MaybeUninit::<input_event>::zeroed().assume_init();
        e.type_ = EV_SND;
        e.code = SND_TONE;
        e.value = freq;

        let e_ptr: *mut c_void = &mut e as *mut _ as *mut c_void;
        write(dev.as_raw_fd(), e_ptr, size_of::<input_event>());
    }
    Ok(())
}

fn driver_console(dev: &File, freq: i32) -> nix::Result<()> {
    let period_in_clock_cycles = TIMER_FREQUENCY.checked_div(freq as u32).unwrap_or(0);

    unsafe {
        kiocsound(dev.as_raw_fd(), period_in_clock_cycles as i32)?;
    }
    Ok(())
}
