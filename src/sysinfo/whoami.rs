use crate::SysInfo;

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
use crate::get_output;

impl SysInfo {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn whoami(&self) -> &str {
        if let Some(var) = self.envvars().get("USER") {
            var
        } else {
            "user"
        }
    }

    #[cfg(target_os = "windows")]
    pub fn whoami(&self) -> String {
        get_output("cmd", &["/C", "whoami"])
            .unwrap_or_else(|_| "user".to_owned())
            .split('\\')
            .rev()
            .next()
            .unwrap_or("user")
            .to_owned()
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd", target_os = "windows")))]
    pub fn whoami(&self) -> String {
        get_output("whoami", &[]).unwrap_or_else(|_| "user".to_owned())
    }
}
