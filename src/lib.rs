#[cfg(not(target_os = "linux"))]
use std::{io, process::Command};

mod hostname;
mod kver;
mod logo;
mod mem;
mod sysinfo;
mod uptime;

pub use hostname::get_hostname;
pub use kver::get_kver;
pub use logo::logo;
pub use mem::get_mem;
pub use sysinfo::SysInfo;
pub use uptime::get_uptime;

#[cfg(not(target_os = "linux"))]
pub fn get_output(command: &str, args: &[&str]) -> io::Result<String> {
    if let Ok(s) = String::from_utf8(Command::new(command).args(args).output()?.stdout) {
        Ok(s.trim().to_owned())
    } else {
        Err(io::Error::from(io::ErrorKind::InvalidData))
    }
}
