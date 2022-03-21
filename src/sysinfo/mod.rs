#[cfg(any(target_os = "linux", target_os = "freebsd"))]
use std::fs;

#[cfg(not(target_os = "windows"))]
use std::{collections::HashMap, env};

mod osname;
mod shell;
mod whoami;

pub struct SysInfo {
    #[cfg(not(target_os = "windows"))]
    envvars: HashMap<String, String>,
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    os_release: Vec<Vec<String>>,
}

impl SysInfo {
    pub fn new() -> Self {
        Self {
            #[cfg(not(target_os = "windows"))]
            envvars: env::vars().collect(),

            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            os_release: if let Ok(s) = fs::read_to_string("/etc/os-release") {
                s
            } else {
                "ID=unix\nPRETTYNAME=\"Unix\"".to_owned()
            }
            .split('\n')
            .map(|line| line.split('=').map(str::to_owned).collect::<Vec<String>>())
            .collect(),
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn envvars(&self) -> &HashMap<String, String> {
        &self.envvars
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn os_release(&self) -> &Vec<Vec<String>> {
        &self.os_release
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn get_os_info(&self, key: &str) -> String {
        if let Some(k) = self.os_release().iter().find(|line| line[0] == key) {
            k[1].trim_matches('"').to_owned()
        } else {
            "Unix".to_owned()
        }
    }
}

impl Default for SysInfo {
    fn default() -> Self {
        Self::new()
    }
}
