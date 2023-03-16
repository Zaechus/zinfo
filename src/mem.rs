use std::io;

#[cfg(not(any(target_os = "freebsd", windows)))]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[cfg(any(target_os = "freebsd", windows))]
use crate::get_output;

#[cfg(not(any(target_os = "freebsd", windows)))]
pub fn get_mem() -> io::Result<String> {
    let mut used = 0;
    let mut total = 0;

    for line in BufReader::new(File::open("/proc/meminfo")?).lines() {
        let line = line?;
        let line: Vec<_> = line.split_whitespace().collect();
        match line[0] {
            "MemTotal:" => {
                total = line[1].parse::<i32>().unwrap_or(0);
                used = total;
            }
            "MemFree:" | "Buffers:" | "Cached:" => {
                used -= line[1].parse::<i32>().unwrap_or(0);
            }
            "Shmem:" => {
                used += line[1].parse::<i32>().unwrap_or(0);
            }
            "SReclaimable:" => {
                used -= line[1].parse::<i32>().unwrap_or(0);
                break;
            }
            _ => (),
        }
    }

    Ok(format!("{}M / {}M", used / 1024, total / 1024))
}

#[cfg(target_os = "freebsd")]
pub fn get_mem() -> io::Result<String> {
    let total = get_output("sysctl", &["-n", "hw.physmem"])?
        .parse::<usize>()
        .unwrap_or(0);

    let mut used = total;

    let pagesize = get_output("sysctl", &["-n", "hw.pagesize"])?
        .parse::<usize>()
        .unwrap_or(0);
    for s in ["cache_count", "free_count", "inactive_count"] {
        used -= get_output("sysctl", &["-n", &format!("vm.stats.vm.v_{}", s)])?
            .parse::<usize>()
            .unwrap_or(0)
            * pagesize;
    }

    Ok(format!(
        "{}M / {}M",
        used / 1024 / 1024,
        total / 1024 / 1024
    ))
}

#[cfg(windows)]
pub fn get_mem() -> io::Result<String> {
    let total = get_output("cmd", &["/C", "wmic os get totalvisiblememorysize"])?
        .split('\n')
        .nth(1)
        .unwrap_or("0")
        .parse::<usize>()
        .unwrap_or(0)
        * 1000
        / 1024
        / 1024;

    Ok(format!(
        "{}M / {}M",
        total
            - get_output("cmd", &["/C", "wmic os get freephysicalmemory"])?
                .split('\n')
                .nth(1)
                .unwrap_or("0")
                .parse::<usize>()
                .unwrap_or(0)
                * 1000
                / 1024
                / 1024,
        total
    ))
}
