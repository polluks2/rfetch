use std::io::{ErrorKind, Result};
use std::os::raw::{c_char, c_int};

use crate::error;

/// Raw `utsname` struct from `/usr/include/sys/utsname.h` C header.
#[repr(C)]
#[derive(Copy, Clone)]
struct utsname {
    pub sysname: [c_char; 65_usize],
    pub nodename: [c_char; 65_usize],
    pub release: [c_char; 65_usize],
    pub version: [c_char; 65_usize],
    pub machine: [c_char; 65_usize],
    pub _domainname: [c_char; 65_usize],
}

extern "C" {
    fn uname(__name: *mut utsname) -> c_int;
}

/// Safe implementation of `/usr/include/sys/utsname.h` header.
#[derive(Debug)]
pub struct Uname {
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
}

impl Uname {
    /// Collects and converts all available information from the utsname
    /// struct from raw C to the safety of Rust.
    pub fn new() -> Result<Self> {
        let mut raw: utsname = unsafe { std::mem::zeroed() };

        let ret = unsafe { uname(&mut raw) };

        if ret != 0 {
            error!("Failed to fill uname")?;
        }

        let info: Uname = Uname {
            sysname: fromraw(&raw.sysname)?.trim().to_string(),
            nodename: fromraw(&raw.nodename)?.trim().to_string(),
            release: fromraw(&raw.release)?.trim().to_string(),
            version: fromraw(&raw.version)?.trim().to_string(),
            machine: fromraw(&raw.machine)?.trim().to_string(),
        };

        Ok(info)
    }
}

/// The actual function which converts C char arrays into Rust `String`.
fn fromraw(s: &[c_char; 65usize]) -> Result<String> {
    let mut v = s.iter().map(|x| *x as u8).collect::<Vec<u8>>();
    v.retain(|x| *x != 0);
    match String::from_utf8(v) {
        Ok(res) => Ok(res),
        Err(e) => error!(&e.to_string())?,
    }
}
