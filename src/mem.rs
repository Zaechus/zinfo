#[cfg(not(target_os = "windows"))]
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[cfg(target_os = "windows")]
use crate::{get_output, io};

#[cfg(not(any(target_os = "freebsd", target_os = "windows")))]
pub fn get_mem() -> io::Result<String> {
    let mut used = 0;
    let mut total = 0;

    for line in BufReader::new(File::open("/proc/meminfo")?).lines() {
        let l: Vec<String> = line?.split_whitespace().map(str::to_owned).collect();
        match l[0].as_str() {
            "MemTotal:" => {
                total = l[1].parse::<i32>().unwrap_or(0);
                used = total;
            }
            "MemFree:" | "Buffers:" | "Cached:" => {
                used -= l[1].parse::<i32>().unwrap_or(0);
            }
            "SReclaimable:" => {
                used -= l[1].parse::<i32>().unwrap_or(0);
                break;
            }
            _ => (),
        }
    }

    Ok(format!("{}M / {}M", used / 1024, total / 1024))
}

#[cfg(target_os = "freebsd")]
pub fn get_mem() -> io::Result<String> {
    let total = get_output("sysctl", &["hw.physmem"])?
        .split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0)
        / 1024;

    format!(
        "{}M / {}M",
        total
            - get_output("vmstat", &["-H"])?
                .split('\n')
                .nth(2)
                .unwrap_or("0")
                .split_whitespace()
                .nth(4)
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0)
                / 1024,
        total
    )
}

#[cfg(target_os = "windows")]
pub fn get_mem() -> io::Result<String> {
    let total = get_output("cmd", &["/C", "wmic os get totalvisiblememorysize"])?
        .split('\n')
        .nth(1)
        .unwrap_or("0")
        .parse::<i64>()
        .unwrap_or(0)
        * 1000
        / 1048576;

    Ok(format!(
        "{}M / {}M",
        total
            - get_output("cmd", &["/C", "wmic os get freephysicalmemory"])?
                .split('\n')
                .nth(1)
                .unwrap_or("0")
                .parse::<i64>()
                .unwrap_or(0)
                * 1000
                / 1048576,
        total
    ))
}
