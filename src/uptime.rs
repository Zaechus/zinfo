#[cfg(target_os = "linux")]
use std::fs;

#[cfg(not(target_os = "linux"))]
use crate::get_output;

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn seconds_to_date(mut seconds: i32) -> String {
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
}

#[cfg(target_os = "linux")]
pub fn get_uptime() -> String {
    if let Ok(uptime) = fs::read_to_string("/proc/uptime") {
        seconds_to_date(
            uptime
                .split_whitespace()
                .next()
                .unwrap_or("0.0")
                .to_owned()
                .parse::<f64>()
                .unwrap_or(0.0) as i32,
        )
    } else {
        "0 s".to_owned()
    }
}

#[cfg(target_os = "windows")]
pub fn get_uptime() -> String {
    if let Ok(o) = get_output("pwsh", &["-Command", "Get-Uptime"]) {
        seconds_to_date(
            o.split('\n')
                .nth(9)
                .unwrap_or("\u{1b}[0m0")
                .split("\u{1b}[0m")
                .nth(1)
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0),
        )
    } else {
        "0m".to_owned()
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub fn get_uptime() -> String {
    if let Ok(o) = get_output("uptime", &["-p"]) {
        let uptime: Vec<_> = o.split_whitespace().rev().map(str::to_owned).collect();

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
    } else {
        "0m".to_owned()
    }
}
