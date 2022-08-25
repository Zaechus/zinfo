#[cfg(target_os = "linux")]
use std::fs;

#[cfg(not(target_os = "linux"))]
use crate::get_output;

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn seconds_to_date(mut seconds: i32) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else {
        let days = seconds / 86400;
        seconds -= days * 86400;

        let hours = seconds / 3600;
        seconds -= hours * 3600;

        let minutes = seconds / 60;

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
            minutes,
        )
    }
}

#[cfg(target_os = "linux")]
pub fn get_uptime() -> String {
    seconds_to_date(
        fs::read_to_string("/proc/uptime")
            .unwrap_or_default()
            .split_whitespace()
            .next()
            .unwrap_or("0.0")
            .to_owned()
            .parse::<f64>()
            .unwrap_or(0.0) as i32,
    )
}

#[cfg(target_os = "windows")]
pub fn get_uptime() -> String {
    if let Ok(o) = get_output("pwsh", &["-Command", "Get-Uptime"]) {
        seconds_to_date(
            o.lines()
                .nth(9)
                .unwrap_or("\u{1b}[0m0")
                .trim()
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
    if let Ok(o) = get_output("uptime", &[]) {
        let mut uptime = o.split_whitespace();

        let (days, time_index) = if o.contains("day") {
            (Some(uptime.nth(2)), 1)
        } else {
            (None, 2)
        };

        let (hours, minutes) = if o.contains("min") {
            (None, uptime.nth(2))
        } else if o.contains("sec") {
            (None, None)
        } else {
            let mut time = uptime
                .nth(time_index)
                .unwrap_or("0:00")
                .trim_matches(',')
                .split(':');
            (time.next(), time.next())
        };

        format!(
            "{}{}{}m",
            if let Some(Some(d)) = days {
                format!("{}d ", d)
            } else {
                String::default()
            },
            if let Some(hours) = hours {
                format!("{}h ", hours)
            } else {
                String::default()
            },
            minutes.unwrap_or("0").parse::<i32>().unwrap_or(0)
        )
    } else {
        "0m".to_owned()
    }
}
