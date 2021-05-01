use std::fs::File;
use std::io::Result;
use std::{env::var, io::Read};

use crate::errorhere;

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
}

impl Ecos {
    /// Collects all information, unavailable infromation is `None`
    pub fn get() -> Result<Self> {
        let ecos = Self {
            name: Self::getuser(),
            home: Self::gethome(),
            shell: Self::getshell(),
            desktop: Self::getdesktop(),
            session: Self::getsession(),
            distro: Self::getdistro(),
        };

        Ok(ecos)
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
}

/// This function will read the `/etc/lsb-release` file on a Linux system and
/// parse to find the `DISTRIB_ID` item. Returns `Ok(DISTRIB_ID)` on success.
fn read_distro() -> Result<String> {
    let mut file: File = File::open("/etc/lsb-release")?;

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    let lsb: String = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(e) => errorhere(&e.to_string())?,
    };

    let v: Vec<&str> = lsb.split('\n').collect();
    for l in v {
        if l.contains("DISTRIB_ID") {
            let n: Vec<&str> = l.split('=').collect();
            return Ok(n[1].into());
        }
    }

    errorhere("failed to read distro")
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
