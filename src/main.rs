///
/// Somewhat of a POC for writing Cimplefetch in Rust, against the whole
/// purpose of Cimplefetch existing: to be written in C.
///
/// Copyright (C) 2021  Avery Murray
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.
///
use std::env;
use std::process::exit;

use rfetch::*;
use uname::uname;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc: usize = args.len();
    let program: &str = args[0].split('/').last().unwrap();

    // Refrences suck sometimes, so drop them in directly. ;)
    let rfs: Rfetch = Rfetch::new(
        match uname() {
            Ok(u) => u,
            Err(e) => {
                eprintln!("{}: error: {}", program, e);
                exit(255);
            }
        },
        User::new(),
    );

    if argc < 2 {
        rfs.print_default();
        return;
    }

    for n in args.iter().take(argc).skip(1) {
        let arg_chrs: Vec<char> = n.chars().collect();

        for c in arg_chrs {
            match c {
                'A' => {
                    rfs.print_all();
                    return;
                }
                'h' => {
                    help();
                    return;
                }

                'a' => rfs.print_arch(),
                'd' => rfs.print_desktop(),
                'H' => rfs.print_home(),
                'k' => rfs.print_kernel(),
                'n' => rfs.print_host(),
                'o' => rfs.print_os(),
                's' => rfs.print_shell(),
                'S' => rfs.print_session(),
                't' => rfs.print_uptime(),
                'T' => rfs.print_uptime(),
                'u' => rfs.print_user(),
                _ => (),
            }
        }
    }
}

fn help() {
    println!("Usage: rfecth")
}
