#[cfg(target_os = "linux")]
use std::fs;

#[cfg(not(target_os = "linux"))]
use crate::get_output;

#[cfg(target_os = "linux")]
pub fn get_kver() -> String {
    if let Ok(ver) = fs::read_to_string("/proc/version") {
        ver.split_whitespace().nth(2).unwrap_or("linux").to_owned()
    } else {
        "linux".to_owned()
    }
}

#[cfg(target_os = "freebsd")]
pub fn get_kver() -> String {
    get_output("uname", &["-K"]).unwrap_or("freebsd".to_owned())
}

#[cfg(windows)]
pub fn get_kver() -> String {
    get_output("cmd", &["/C", "wmic os get Version"])
        .unwrap_or("NT".to_owned())
        .lines()
        .nth(1)
        .unwrap_or("NT")
        .to_owned()
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd", windows)))]
pub fn get_kver() -> String {
    get_output("uname", &["-r"]).unwrap_or("idk".to_owned())
}
