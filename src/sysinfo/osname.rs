use crate::SysInfo;

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
use crate::get_output;

impl SysInfo {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn get_os_name(&self) -> String {
        self.get_os_info("PRETTY_NAME")
    }

    #[cfg(target_os = "android")]
    pub fn get_os_name(&self) -> String {
        format(
            "{} {}",
            get_output("uname", &["-o"]).unwrap_or_else(|_| "Android".to_owned()),
            get_output("getprop", &["ro.build.version.release"]).unwrap_or_else(|_| String::new()),
        )
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

    #[cfg(not(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "android",
        target_os = "windows"
    )))]
    pub fn get_os_name(&self) -> String {
        get_output("uname", &["-o"]).unwrap_or_else(|_| "Name".to_owned())
    }
}
