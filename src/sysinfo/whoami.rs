use crate::SysInfo;

#[cfg(not(target_os = "linux"))]
use crate::get_output;

impl SysInfo {
    #[cfg(target_os = "linux")]
    pub fn whoami(&self) -> &str {
        if let Some(var) = self.envvars().get("USER") {
            var
        } else {
            "user"
        }
    }

    #[cfg(target_os = "windows")]
    pub fn whoami(&self) -> String {
        get_output("cmd", &["/C", "whoami"], "user")
            .unwrap_or("user")
            .split('\\')
            .rev()
            .next()
            .unwrap_or("user")
            .to_owned()
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    pub fn whoami(&self) -> String {
        get_output("whoami", &[]).unwrap_or("user".to_owned())
    }
}
