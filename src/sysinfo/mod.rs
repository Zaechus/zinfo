use std::{collections::HashMap, env};

mod whoami;

pub struct SysInfo {
    #[cfg(not(target_os = "windows"))]
    envvars: HashMap<String, String>,
}

impl SysInfo {
    pub fn new() -> Self {
        Self {
            envvars: env::vars().collect(),
        }
    }

    pub fn envvars(&self) -> &HashMap<String, String> {
        &self.envvars
    }
}
