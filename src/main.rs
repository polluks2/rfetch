mod ecosys;
mod uname;

use std::env::{Args, args};

use std::io::{Error, ErrorKind, Result};

use ecosys::Ecos;
use uname::Uname;

macro_rules! printo {
    ($fmt:expr, $o:expr) => {
        if let Some(v) = &$o {
            println!($fmt, v);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($e:expr) => {
        crate::_errorhere(ErrorKind::Other, $e)
    };
}

fn _errorhere<T>(kind: ErrorKind, s: &str) -> Result<T> {
    Err(Error::new(kind, s))
}

fn main() -> Result<()> {
    let rfetch = Rfetch::create(Ecos::get(), Uname::get()?);

    rfetch.run(args())?;

    Ok(())
}

/*
static TUX: [&str; 7] = [
    "    .--.",
    "   |o_o |",
    "   |:_/ |",
    "  //   \\ \\",
    " (|     | )",
    "/'|_   _/'\\",
    "\\___)=(___/\\",
];
*/

// Main struct to contain the ecosys and uname structs to safely access both.
struct Rfetch {
    user: Ecos,
    uname: Uname,
    // logo: Vec<&'static str>,
}

impl Rfetch {
    pub fn create(user: Ecos, uname: Uname) -> Rfetch {
        Self { user, uname } // logo: Vec::new() }
    }

    /// This is effectively the main function.
    /// This will parse the arguments and executes what the user requets.
    pub fn run(self, args: Args) -> Result<()> {
        let argc: usize = args.len();

        // If there are no arguments, print default and exit.
        if argc == 1 {
            self.default();
            return Ok(());
        }

        // Iter through each argument, and the characters of every argument.
        args.into_iter().skip(1).try_for_each(|arg| self.parse_args(arg))?;

        Ok(())
    }

    // Iter through each argument happens here
    fn parse_args(&self, arg: String) -> Result<()> {
        if !arg.contains('-') {
            error!("missing arguments")?;
        }

        if arg == "--help" || arg == "-h" {
            self.help();
            return Ok(());
        } else if arg == "--all" || arg == "-A" {
            self.print_all();
            return Ok(());
        }

        arg.chars().try_for_each(|x| self.parse_chars(x))?;

        Ok(())
    }

    // Iter through each char of every argument happens here
    fn parse_chars(&self, c: char) -> Result<()> {
        match c {
            'A' => self.print_all(), // Each method name explains.
            'a' => self.print_arch(),
            'b' => self.print_board(),
            'c' => self.print_cpu(),
            'd' => self.print_desktop(),
            'D' => self.print_distro(),
            'H' => self.print_home(),
            'k' => self.print_kernel(),
            'm' => self.print_mem(),
            'n' => self.print_host(),
            'o' => self.print_os(),
            's' => self.print_shell(),
            'S' => self.print_session(),
            'u' => self.print_name(),

            '-' => (),
            _ => error!(&format!("'{}' not a valid argument", c))?,
        }

        Ok(())
    }

    ///
    /// ```
    /// Distro:     Arch Linux
    /// User:       avery
    /// Kernel:     5.11.16-arch1-1
    /// Shell:     Fish
    /// ```
    ///
    fn default(&self) {
        self.print_distro();
        self.print_name();
        self.print_kernel();
        self.print_shell();
    }

    ///
    /// ```
    /// Distro:     Arch Linux
    /// User:       avery
    /// Home:       /home/avery
    /// Kernel:     5.11.16-arch1-1
    /// Shell:      Fish
    /// Arch:       x86_64
    /// Desktop:    Gnome
    /// Session:    Wayland
    /// OS:         Linux
    /// ```
    ///
    fn print_all(&self) {
        self.print_distro();
        self.print_name();
        self.print_home();
        self.print_kernel();
        self.print_host();
        self.print_shell();
        self.print_arch();
        self.print_cpu();
        self.print_board();
        self.print_mem();
        self.print_desktop();
        self.print_session();
        self.print_os();
    }

    fn help(&self) {
        println!("{}", HELP)
    }

    fn print_arch(&self) {
        println!("Arch:\t\t{}", self.uname.machine)
    }

    fn print_board(&self) {
        printo!("Host:\t\t{}", self.user.board)
    }

    fn print_cpu(&self) {
        printo!("CPU:\t\t{}", self.user.cpu)
    }

    fn print_desktop(&self) {
        printo!("Desktop:\t{}", self.user.desktop)
    }

    fn print_home(&self) {
        printo!("Home:\t\t{}", self.user.home)
    }

    fn print_kernel(&self) {
        println!("Kernel:\t\t{}", self.uname.release)
    }

    fn print_mem(&self) {
        printo!("Memory:\t\t{} MiB", self.user.mem)
    }

    fn print_host(&self) {
        println!("Hostname:\t{}", self.uname.nodename)
    }

    fn print_os(&self) {
        println!("OS:\t\t{}", self.uname.sysname)
    }

    fn print_shell(&self) {
        printo!("Shell:\t\t{}", self.user.shell)
    }

    fn print_session(&self) {
        printo!("Session:\t{}", self.user.session)
    }

    fn print_name(&self) {
        printo!("User:\t\t{}", self.user.name)
    }

    fn print_distro(&self) {
        if let Some(d) = &self.user.distro {
            println!("Distro:\t\t{} {}", d, self.uname.sysname)
        }
    }
}

const HELP: &str = "\
Usage: rfetch [FLAG]

FLAGS:
	-A, --all\tView all
	-a\t\tVies system architecture
	-b\t\tView system board family
	-c\t\tView system CPU
	-d\t\tView desktop environment
	-D\t\tView Linux Distribution
	-h, --help\tView this help information
	-H\t\tView current user home directory
	-k\t\tView system kernel
	-o\t\tView system OS
	-s\t\tView user shell
	-S\t\tView current graphics session
	-u\t\tView user name
";
