use std::process;

pub fn check_root() {
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("This program must be run as root. Exiting...");
        process::exit(1);
    }
}
