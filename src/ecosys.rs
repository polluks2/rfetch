use std::fs::File;
use std::io::{ErrorKind, Result};
use std::{env::var, io::Read};

use crate::error;

macro_rules! handle {
    ($f:expr) => {
        match $f {
            Ok(a) => Some(a),
            _ => None,
        }
    };
}

/// Ecos system 'Ecosystem' struct contains user's account and desktop
/// information.
/// All information is contained in `Option`, if information is unavailable
/// the item is `None`.
#[derive(Debug)]
pub struct Ecos {
    pub name: Option<String>,
    pub home: Option<String>,
    pub shell: Option<String>,
    pub desktop: Option<String>,
    pub session: Option<String>,
    pub distro: Option<String>,
    pub cpu: Option<String>,
}

impl Ecos {
    /// Collects all information, unavailable infromation is `None`
    pub fn get() -> Self {
        Self {
            name: Self::getuser(),
            home: Self::gethome(),
            shell: Self::getshell(),
            desktop: Self::getdesktop(),
            session: Self::getsession(),
            distro: Self::getdistro(),
            cpu: Self::getcpu(),
        }
    }

    fn getuser() -> Option<String> {
        handle!(var("USER"))
    }

    fn gethome() -> Option<String> {
        handle!(var("HOME"))
    }

    fn getshell() -> Option<String> {
        let path = handle!(var("SHELL"))?;
        Some(path.split('/').last()?.to_string().to_title())
    }

    fn getdesktop() -> Option<String> {
        Some(handle!(var("DESKTOP_SESSION"))?.to_title())
    }

    fn getsession() -> Option<String> {
        Some(handle!(var("XDG_SESSION_TYPE"))?.to_title())
    }

    fn getdistro() -> Option<String> {
        handle!(read_distro())
    }

    fn getcpu() -> Option<String> {
        handle!(read_cpu())
    }
}

/// This function will read the `/etc/lsb-release` file on a Linux system and
/// parse to find the `DISTRIB_ID` item. Returns `Ok(DISTRIB_ID)` on success.
fn read_distro() -> Result<String> {
    let mut file: File = File::open("/etc/lsb-release")?;

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    let lsb: String = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(e) => error!(&e.to_string())?,
    };

    let v: Vec<&str> = lsb.split('\n').collect();
    for l in v {
        if l.contains("DISTRIB_ID") {
            return Ok(get_special(l, '='))
        }
    }

    error!("failed to read distro")
}

fn read_cpu() -> Result<String> {
    let mut file: File = File::open("/proc/cpuinfo")?;

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    let cpu: String = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(e) => error!(&e.to_string())?,
    };

    let v: Vec<&str> = cpu.split('\n').collect();
    for l in v {
        if l.contains("model name") {
            return Ok(get_special(l, ':'))
        }
    }

    error!("failed to read CPU")
}

fn get_special(s: &str, split: char) -> String {
    let n: Vec<&str> = s.split(split).collect();
    return n[1].to_string().trim().into()
}

/// This trait exists purely to change a String to have a capital first
/// letter using a method.
trait Title {
    fn to_title(&self) -> Self;
}

impl Title for String {
    fn to_title(&self) -> Self {
        let mut c = self.chars();

        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
