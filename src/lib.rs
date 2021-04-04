// lib.rs
// rfch
use std::env::var;
use uname::Info as Uname;

const NONE: &str = "NONE";

pub struct Rfetch {
    pub user: User,
    pub uname: Uname,
}

impl Rfetch {
    /// Creates a new Rfetch struct with the two other structs
    /// for ultimate structs.
    pub fn new(uts: Uname, uss: User) -> Self {
        Self {
            user: uss,
            uname: uts,
        }
    }

    /// Prints the user's name and host name:
    /// foo@bar
    pub fn print_user_stuffs(&self) {
        if self.user.name != NONE {
            println!("         {}@{}", self.user.name, self.uname.nodename);
        }
    }

    /// Prints the user's name
    pub fn print_user(&self) {
        if self.user.name != NONE {
            println!("User:    {}", self.user.name);
        }
    }

    /// Prints OS name
    pub fn print_os(&self) {
        println!("OS:      {}", self.uname.sysname);
    }

    /// Prints kernel version
    pub fn print_kernel(&self) {
        println!("Kernel:  {}", self.uname.release);
    }

    /// Prints hostname
    pub fn print_host(&self) {
        println!("Host:    {}", self.uname.nodename);
    }

    /// Prints system architecture
    pub fn print_arch(&self) {
        println!("Arch:    {}", self.uname.machine);
    }

    /// Prints user shell
    pub fn print_shell(&self) {
        if self.user.shell != NONE {
            println!("Shell:   {}", self.user.shell);
        }
    }

    /// Prints user desktop env
    pub fn print_desktop(&self) {
        if self.user.desktop != NONE {
            println!("Desktop: {}", self.user.desktop);
        }
    }

    /// Prints the current window system
    pub fn print_session(&self) {
        if self.user.session != NONE {
            println!("Session: {}", self.user.session);
        }
    }

    /// Prints user home directory
    pub fn print_home(&self) {
        if self.user.home != NONE {
            println!("Home:    {}", self.user.home);
        }
    }

    /// Prints system uptime
    pub fn print_uptime(&self) {}

    ///         avery@caline
    /// OS:      Linux
    /// Kernel:  5.11.11-arch1-1
    /// Shell:   fish
    ///
    ///
    pub fn print_default(self) {
        self.print_user_stuffs();
        self.print_os();
        self.print_kernel();
        self.print_uptime(); // not implemented
        self.print_shell();

        drop(self);
    }

    /// Prints all available information
    pub fn print_all(self) {
        self.print_user_stuffs();
        self.print_user();
        self.print_os();
        self.print_kernel();
        self.print_host();
        self.print_arch();
        self.print_uptime(); // not implemented
        self.print_shell();
        self.print_desktop();
        self.print_session();
        self.print_host();

        drop(self);
    }
}

pub struct User {
    pub name: String,
    pub home: String,
    pub shell: String,
    pub desktop: String,
    pub session: String,
}

impl User {
    fn var_handler(variable: &str) -> String {
        var(variable).unwrap_or_else(|_| NONE.to_string())
    }

    pub fn new() -> Self {
        Self {
            name: Self::var_handler("LOGNAME"),
            home: Self::var_handler("HOME"),
            shell: Self::var_handler("SHELL")
                .split('/')
                .last()
                .unwrap_or(NONE)
                .to_string(),
            desktop: Self::var_handler("XDG_SESSION_DESKTOP"),
            session: Self::var_handler("XDG_SESSION_TYPE"),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self::new()
    }
}

// use uname::Info as Uname
