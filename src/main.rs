mod ecosys;
mod uname;

use std::io::{Result, Error, ErrorKind};
use std::env::args;

use ecosys::Ecos;
use uname::Uname;

macro_rules! printo {
	($fmt:expr, $o:expr) => {
		if let Some(v) = &$o {
			println!($fmt, v);
		}
	};
}

// Main struct to contain the ecosys and uname structs to safely access both.
struct Rfetch {
	user: Ecos,
	uname: Uname,
}

impl Rfetch {
	pub fn create(user: Ecos, uname: Uname) -> Rfetch {
		Self { user, uname }
	}

	/// This is effectively the main function.
	/// This will parse the arguments and executes what the user requets.
	pub fn run(self, args: &[String]) -> Result<()> {
		let argc: usize = args.len();

		// If there are no arguments, print default and exit.
		if argc == 1 {
			self.default();
			return Ok(());
		}

		// If the user wants help.
		if args.contains(&"--help".into()) || args.contains(&"-h".into()) {
			self.help();
			return Ok(())
		}

		// Iter through each argument, and the characters of every argument.
		for arg in args.iter().skip(1) {
			// check if it even is an argument.
			if !arg.contains('-') {
				errorhere("missing arguments")?;
			}
			
			// Collect each char of the argument.
			let chargs: Vec<char> = arg.chars().collect();
			for c in chargs {
				match c {
					'A' => self.print_all(), // Each method name explains.
					'a' => self.print_arch(),
					'd' => self.print_desktop(),
					'D' => self.print_distro(),
					'H' => self.print_home(),
					'k' => self.print_kernel(),
					'o' => self.print_os(),
					's' => self.print_shell(),
					'S' => self.print_session(),
					'u' => self.print_name(),

					'-' => (),
					_ => errorhere(&format!("'{}' not a valid argument", c))?,
				}
			}
		}
		Ok(())
	}

	///
	/// ```
	/// Distro:		Arch Linux
	/// User:		avery
	/// Kernel:		5.11.16-arch1-1
	/// Shell:		Fish
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
	/// Distro:		Arch Linux
	/// User:		avery
	/// Home:		/home/avery
	/// Kernel:		5.11.16-arch1-1
	/// Shell:		Fish
	/// Arch:		x86_64
	/// Desktop:	Gnome
	/// Session:	Wayland
	/// OS:		Linux
	/// ```
	///
	fn print_all(&self) {
		self.print_distro();
		self.print_name();
		self.print_home();
		self.print_kernel();
		self.print_shell();
		self.print_arch();
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

	fn print_desktop(&self) {
		printo!("Desktop:\t{}", self.user.desktop)
	}

	fn print_home(&self) {
		printo!("Home:\t\t{}", self.user.home)
	}

	fn print_kernel(&self) {
		println!("Kernel:\t\t{}", self.uname.release)
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

fn main() -> Result<()> {
	let args: Vec<String> = args().collect();

	let rfetch = Rfetch::create(
		Ecos::get()?,
		Uname::get()?,
	);

	rfetch.run(&args)?;

	Ok(())
}

fn errorhere<T>(s: &str) -> Result<T> {
	Err(Error::new(ErrorKind::Other, s))
}
