#[cfg(target_os = "linux")]
use std::fs;

#[cfg(target_os = "linux")]
pub fn get_uptime() -> String {
    if let Ok(uptime) = fs::read_to_string("/proc/uptime") {
        let mut seconds = uptime
            .split_whitespace()
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
        .split_whitespace()
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
