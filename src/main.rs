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

		for arg in args.iter().skip(1) {
			
			let chargs: Vec<char> = arg.chars().collect();

			for c in chargs {
				match c {
					'-' => (),
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

	fn print_arch(&self) {
		println!("Arch:\t\t{}", self.uname.machine)
	}

	fn print_desktop(&self) {
		if let Some(d) = &self.user.desktop {
			println!("Desktop:\t{}", d)
		}
	}

	fn print_home(&self) {
		if let Some(h) = &self.user.home {
			println!("Home:\t\t{}", h);
		}
	}

	fn print_kernel(&self) {
		println!("Kernel:\t\t{}", self.uname.release)
	}

	fn print_os(&self) {
		println!("OS:\t\t{}", self.uname.sysname)
	}

	fn print_shell(&self) {
		if let Some(s) = &self.user.shell {
			println!("Shell:\t\t{}", s)
		}
	}

	fn print_session(&self) {
		if let Some(s) = &self.user.session {
			println!("Session:\t{}", s)
		}
	}

	fn print_name(&self) {
		if let Some(n) = &self.user.name {
			println!("User:\t\t{}", n);
		}
	}

	fn print_distro(&self) {
		if let Some(d) = &self.user.distro {
			println!("Distro:\t\t{} {}", d, self.uname.sysname)
		}
	}
}

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
