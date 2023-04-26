use std::io;

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
use crate::get_output;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub fn get_os() -> io::Result<(String, String)> {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    let mut id = String::new();
    let mut name = "Unix".to_owned();
    let mut version_id = String::new();

    for line in BufReader::new(File::open("/etc/os-release")?).lines() {
        let line = line?;
        let line: Vec<_> = line.split('=').collect();

        match line[0] {
            "ID" => id = line[1].trim_matches('"').to_owned(),
            "NAME" => name = line[1].trim_matches('"').to_owned(),
            "VERSION_ID" => version_id = line[1].trim_matches('"').to_owned(),
            _ => (),
        }
    }

    Ok((id, format!("{} {}", name, version_id)))
}

#[cfg(target_os = "android")]
pub fn get_os() -> io::Result<(String, String)> {
    Ok((
        "android".to_owned(),
        format!(
            "{} {}",
            get_output("uname", &["-o"]).unwrap_or("Android".to_owned()),
            get_output("getprop", &["ro.build.version.release"]).unwrap_or(String::new()),
        ),
    ))
}

#[cfg(windows)]
pub fn get_os() -> io::Result<(String, String)> {
    Ok((
        "windows".to_owned(),
        if let Ok(o) = get_output("cmd", &["/C", "wmic os get Caption"]) {
            o.lines()
                .nth(1)
                .unwrap_or("Microsoft Windows")
                .chars()
                .skip(10)
                .collect()
        } else {
            "Windows".to_owned()
        },
    ))
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "android",
    windows
)))]
pub fn get_os() -> io::Result<(String, String)> {
    Ok((
        String::new(),
        get_output("uname", &["-o"]).unwrap_or("Name".to_owned()),
    ))
}
