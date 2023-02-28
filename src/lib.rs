#[cfg(not(target_os = "linux"))]
use std::{io, process::Command};

mod hostname;
mod kver;
mod logo;
mod mem;
mod os;
mod shell;
mod uptime;
mod whoami;

pub use hostname::hostname;
pub use kver::get_kver;
pub use logo::logo;
pub use mem::get_mem;
pub use os::get_os;
pub use shell::get_shell;
pub use uptime::uptime;
pub use whoami::whoami;

#[cfg(not(target_os = "linux"))]
pub fn get_output(command: &str, args: &[&str]) -> io::Result<String> {
    if let Ok(s) = String::from_utf8(Command::new(command).args(args).output()?.stdout) {
        Ok(s.trim().to_owned())
    } else {
        Err(io::Error::from(io::ErrorKind::InvalidData))
    }
}
