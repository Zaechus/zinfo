use crate::SysInfo;

impl SysInfo {
    #[cfg(target_os = "windows")]
    pub fn get_shell(&self) -> String {
        "cmd".to_owned()
    }
    #[cfg(not(target_os = "windows"))]
    pub fn get_shell(&self) -> String {
        if let Some(var) = self.envvars().get("SHELL") {
            if let Some(last_slash) = var.rfind('/') {
                &var[last_slash + 1..]
            } else {
                var
            }
        } else {
            "sh"
        }
        .to_owned()
    }
}
