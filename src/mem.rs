#[cfg(not(target_os = "windows"))]
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[cfg(target_os = "windows")]
use crate::{get_output, io};

#[cfg(not(target_os = "windows"))]
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

#[cfg(target_os = "windows")]
pub fn get_mem() -> Result<String, io::Error> {
    let free = get_output("cmd", &["/C", "wmic os get freephysicalmemory"])?
        .split('\n')
        .nth(1)
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0)
        / 1024;
    let total = get_output("cmd", &["/C", "wmic os get totalvisiblememorysize"])?
        .split('\n')
        .nth(1)
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0)
        / 1024;

    Ok(format!("{}M / {}M", total - free, total))
}
