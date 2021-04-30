use std::os::raw::{c_int, c_char};
use std::io::Result;

use crate::errorhere;

#[repr(C)]
#[derive(Copy, Clone)]
struct utsname {
    pub sysname: [c_char; 65usize],
    pub nodename: [c_char; 65usize],
    pub release: [c_char; 65usize],
    pub version: [c_char; 65usize],
    pub machine: [c_char; 65usize],
    pub _domainname: [c_char; 65usize],
}

extern "C" {
    fn uname(__name: *mut utsname) -> c_int;
}

pub struct Uname {
	pub sysname: String,
	pub nodename: String,
	pub release: String,
	pub version: String,
	pub machine: String,
	pub domainname: String
}

impl Uname {
	pub fn get()-> Result<Self> {

		let mut raw: utsname = utsname {
		    sysname: [c_char::default(); 65usize],
		    nodename: [c_char::default(); 65usize],
		    release: [c_char::default(); 65usize],
		    version: [c_char::default(); 65usize],
		    machine: [c_char::default(); 65usize],
		    _domainname: [c_char::default(); 65usize],
		};

		unsafe { uname(&mut raw); }

		let info: Uname = Uname {
		    sysname: fromraw(&raw.sysname)?,
		    nodename: fromraw(&raw.nodename)?,
		    release: fromraw(&raw.release)?,
		    version: fromraw(&raw.version)?,
		    machine: fromraw(&raw.machine)?,
		    domainname: fromraw(&raw._domainname)?,
		};

		Ok(info)
	}
}

fn fromraw(s: &[c_char; 65usize]) -> Result<String> {

	match String::from_utf8(
		s.iter().map(|x| *x as u8).collect()
	) {
		Ok(res) => Ok(res),
		Err(e) => errorhere(&e.to_string())?,
	}

}