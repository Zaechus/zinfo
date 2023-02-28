#[cfg(target_os = "linux")]
use std::fs;

#[cfg(not(target_os = "linux"))]
use crate::get_output;

#[cfg(target_os = "linux")]
pub fn hostname() -> String {
    if let Ok(s) = fs::read_to_string("/etc/hostname") {
        s.trim().to_owned()
    } else {
        "hostname".to_owned()
    }
}

#[cfg(not(target_os = "linux"))]
pub fn hostname() -> String {
    if let Ok(o) = get_output("hostname", &[]) {
        o
    } else {
        "hostname".to_owned()
    }
}
