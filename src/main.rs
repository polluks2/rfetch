mod ecosys;
mod uname;

use std::io::{Result, Error, ErrorKind};
use std::env::args;

use ecosys::Ecos;
use uname::Uname;

struct Rfetch {
	user: Ecos,
	uname: Uname,
}

macro_rules! printo {
	($o:expr) => {
		if let Some(v) = &$o {
			println!("{}", v)
		}
	};

	($fmt:expr, $o:expr) => {
		if let Some(v) = &$o {
			println!($fmt, v);
		}
	};
}

impl Rfetch {
	pub fn create(user: Ecos, uname: Uname) -> Rfetch {
		Self { user, uname }
	}


	pub fn run(self, args: &[String]) -> Result<()> {
		let argc: usize = args.len();

		if argc == 1 {
			self.default();
			return Ok(());
		}

		if args.contains(&"--help".into()) || args.contains(&"-h".into()) {
			self.help();
			return Ok(())
		}

		for arg in args.iter().skip(1) {
			if !arg.contains('-') {
				errorhere("missing arguments")?;
			}
			
			let chargs: Vec<char> = arg.chars().collect();

			for c in chargs {
				match c {
					'A' => self.print_all(),
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

	fn default(&self) {
		self.print_distro();
		self.print_name();
		self.print_kernel();
		self.print_shell();
	}

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
