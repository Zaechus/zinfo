use crate::SysInfo;

#[cfg(not(target_os = "linux"))]
use crate::get_output;

impl SysInfo {
    #[cfg(target_os = "linux")]
    pub fn get_os_name(&self) -> String {
        self.get_os_info("PRETTY_NAME")
    }

    #[cfg(target_os = "windows")]
    pub fn get_os_name(&self) -> String {
        if let Ok(o) = get_output("cmd", &["/C", "wmic os get Caption"]) {
            o.split('\n')
                .nth(1)
                .unwrap_or("Microsoft Windows")
                .chars()
                .skip(10)
                .collect()
        } else {
            "Windows".to_owned()
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    pub fn get_os_name(&self) -> String {
        get_output("uname", &["-o"]).unwrap_or("Linux".to_owned())
    }
}
