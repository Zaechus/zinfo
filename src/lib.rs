use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[cfg(not(target_os = "linux"))]
use std::process::Command;

mod kver;
mod logo;
mod sysinfo;
mod uptime;

pub use kver::get_kver;
pub use logo::logo;
pub use sysinfo::SysInfo;
pub use uptime::get_uptime;

#[cfg(not(target_os = "linux"))]
pub fn get_output(command: &str, args: &[&str], default: &str) -> String {
    if let Ok(output) = Command::new(command).args(args).output() {
        if let Ok(s) = String::from_utf8(output.stdout) {
            s.trim().to_owned()
        } else {
            default.to_owned()
        }
    } else {
        default.to_owned()
    }
}

pub fn get_mem() -> io::Result<String> {
    let meminfo = File::open("/proc/meminfo")?;
    let reader = BufReader::new(meminfo).lines();

    let mut total = 0;
    let mut free = 0;

    for line in reader {
        let l: Vec<String> = line?.split_whitespace().map(str::to_owned).collect();
        match l[0].as_str() {
            "MemTotal:" => {
                total = l[1].parse::<i32>().unwrap_or(0);
                free = total;
            }
            "MemFree:" | "Buffers:" | "Cached:" => {
                free -= l[1].parse::<i32>().unwrap_or(0);
            }
            "SReclaimable:" => {
                free -= l[1].parse::<i32>().unwrap_or(0);
                break;
            }
            _ => (),
        }
    }

    Ok(format!("{}M / {}M", free / 1024, total / 1024))
}
