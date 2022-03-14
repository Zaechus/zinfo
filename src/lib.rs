use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[cfg(target_os = "linux")]
use std::fs;

#[cfg(not(target_os = "linux"))]
use std::process::Command;

mod logo;
mod osname;
mod sysinfo;

pub use logo::logo;
pub use sysinfo::SysInfo;

#[cfg(target_os = "windows")]
pub use osname::get_os_name;

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

#[cfg(target_os = "linux")]
pub fn get_uptime() -> String {
    if let Ok(uptime) = fs::read_to_string("/proc/uptime") {
        let mut seconds = uptime
            .split(' ')
            .next()
            .unwrap_or("0.0")
            .to_owned()
            .parse::<f64>()
            .unwrap_or(0.0) as i32;

        let days = seconds / 86400;
        seconds -= days * 86400;

        let hours = seconds / 3600;
        seconds -= hours * 3600;

        let minutes = seconds / 60;
        seconds -= minutes * 60;

        format!(
            "{}{}{}{}s",
            if days == 0 {
                String::default()
            } else {
                format!("{}d ", days)
            },
            if hours == 0 {
                String::default()
            } else {
                format!("{}h ", hours)
            },
            if minutes == 0 {
                String::default()
            } else {
                format!("{}m ", minutes)
            },
            seconds
        )
    } else {
        "0 s".to_owned()
    }
}

#[cfg(not(target_os = "linux"))]
pub fn get_uptime() -> String {
    let uptime: Vec<_> = get_output("uptime", &["-p"], "0 minutes")
        .split(' ')
        .rev()
        .map(str::to_owned)
        .collect();

    let mut minutes = 0;
    let mut hours = 0;
    let mut days = 0;

    for pair in uptime.chunks(2).filter(|c| c.len() == 2) {
        match pair[0].as_str() {
            "minutes" => minutes = pair[1].parse().unwrap_or(0),
            "hours," => hours = pair[1].parse().unwrap_or(0),
            "days," => days = pair[1].parse().unwrap_or(0),
            _ => (),
        }
    }

    format!(
        "{}{}{}m",
        if days == 0 {
            String::default()
        } else {
            format!("{}d ", days)
        },
        if hours == 0 {
            String::default()
        } else {
            format!("{}h ", hours)
        },
        minutes
    )
}

pub fn get_mem() -> io::Result<String> {
    let meminfo = File::open("/proc/meminfo")?;
    let reader = BufReader::new(meminfo).lines();

    let mut total = 0;
    let mut free = 0;
    let mut buffers = 0;
    let mut cached = 0;

    for line in reader {
        let l: Vec<String> = line?.split_whitespace().map(str::to_owned).collect();
        match l[0].as_str() {
            "MemTotal:" => {
                total = l[1].parse::<i32>().unwrap_or(0);
            }
            "MemFree:" => {
                free = l[1].parse::<i32>().unwrap_or(0);
            }
            "Buffers" => {
                buffers = l[1].parse::<i32>().unwrap_or(0);
            }
            "Cached:" => {
                cached = l[1].parse::<i32>().unwrap_or(0);
                break;
            }
            _ => (),
        }
    }

    Ok(format!(
        "{}M / {}M",
        (total - free - buffers - cached) / 1024,
        total / 1024
    ))
}
