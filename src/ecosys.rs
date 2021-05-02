use std::fs::File;
use std::io::{ErrorKind, Result};
use std::{env::var, io::Read};

use crate::error;

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
    pub board: Option<String>,
    pub mem: Option<String>,
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
            board: Self::getproduct(),
            mem: Self::getmem(),
        }
    }

    fn getuser() -> Option<String> {
        var("USER").ok()
    }

    fn gethome() -> Option<String> {
        var("HOME").ok()
    }

    fn getshell() -> Option<String> {
        let path = var("SHELL").ok()?;
        Some(path.split('/').last()?.to_string().to_title())
    }

    fn getdesktop() -> Option<String> {
        Some(var("DESKTOP_SESSION").ok()?.to_title())
    }

    fn getsession() -> Option<String> {
        Some(var("XDG_SESSION_TYPE").ok()?.to_title())
    }

    fn getdistro() -> Option<String> {
        read_distro().ok()
    }

    fn getcpu() -> Option<String> {
        read_cpu().ok()
    }

    fn getproduct() -> Option<String> {
        read_product().ok()
    }

    fn getmem() -> Option<String> {
        let totalstr = read_memory("MemTotal").ok()?;
        let freestr = read_memory("MemFree").ok()?;
        let bufferstr = read_memory("Buffers").ok()?;
        let cachedstr = read_memory("Cached").ok()?;

        let total: u32 = totalstr.parse().ok()?;
        let free: u32 = freestr.parse().ok()?;
        let buffers: u32 = bufferstr.parse().ok()?;
        let cached: u32 = cachedstr.parse().ok()?;
        // let avail: u32 = availstr.parse().ok()?;

        Some(format!("{}/{}", (total-free-buffers-cached)/1024, (total)/1024))
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
            return Ok(get_special(l, '=', 1))
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
            return Ok(get_special(l, ':', 1))
        }
    }

    error!("failed to read CPU")
}

fn read_product() -> Result<String> {
    let mut famfile: File = File::open("/sys/devices/virtual/dmi/id/product_family")?;

    let mut buf: Vec<u8> = Vec::new();
    famfile.read_to_end(&mut buf)?;

    let family = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(e) => error!(&e.to_string())?,
    };

    Ok(family.trim().to_string())
}

fn read_memory(p: &str) -> Result<String> {
    let mut file = File::open("/proc/meminfo")?;

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    let meminfo: String = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(e) => error!(&e.to_string())?,
    };

    let v: Vec<&str> = meminfo.split('\n').collect();
    for l in v {
        if l.contains(p) {
            let tmp = get_special(l, ':', 1);
            return Ok(get_special(&tmp, ' ', 0));
        }
    }

    error!("failed to read meminfo")
}

fn get_special(s: &str, split: char, v: usize) -> String {
    let n: Vec<&str> = s.split(split).collect();
    return n[v].to_string().trim().into()
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
