#[cfg(not(target_os = "linux"))]
use crate::get_output;

#[cfg(target_os = "windows")]
pub fn get_os_name() -> String {
    get_output("cmd", &["/C", "wmic os get Caption"], "Windows")
        .split('\n')
        .next()
        .unwrap_or("Microsoft Windows")
        .split(' ')[1..]
        .collect()
}
