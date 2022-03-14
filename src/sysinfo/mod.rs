#[cfg(target_os = "linux")]
use std::fs;

#[cfg(not(target_os = "windows"))]
use std::{collections::HashMap, env};

mod osname;
mod shell;
mod whoami;

pub struct SysInfo {
    #[cfg(not(target_os = "windows"))]
    envvars: HashMap<String, String>,
    #[cfg(target_os = "linux")]
    os_release: String,
}

impl SysInfo {
    pub fn new() -> Self {
        Self {
            #[cfg(not(target_os = "windows"))]
            envvars: env::vars().collect(),

            #[cfg(target_os = "linux")]
            os_release: if let Ok(s) = fs::read_to_string("/etc/os-release") {
                s
            } else {
                "ID=linux\nPRETTYNAME=\"Linux\"".to_owned()
            },
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn envvars(&self) -> &HashMap<String, String> {
        &self.envvars
    }

    #[cfg(target_os = "linux")]
    pub fn os_release(&self) -> &str {
        &self.os_release
    }

    #[cfg(target_os = "linux")]
    pub fn get_os_info(&self, key: &str) -> String {
        if let Some(pretty_name) = self
            .os_release()
            .split('\n')
            .map(|line| line.split('=').collect::<Vec<_>>())
            .find(|line| line[0] == key)
        {
            pretty_name[1].trim_matches('"').to_owned()
        } else {
            "Linux".to_owned()
        }
    }
}
