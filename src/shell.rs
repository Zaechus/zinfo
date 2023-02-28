#[cfg(not(target_os = "windows"))]
use std::env;

#[cfg(target_os = "windows")]
pub fn get_shell() -> &'static str {
    "cmd"
}

#[cfg(not(target_os = "windows"))]
pub fn get_shell() -> String {
    if let Ok(var) = env::var("SHELL") {
        if let Some(last_slash) = var.rfind('/') {
            var[last_slash + 1..].to_owned()
        } else {
            var
        }
    } else {
        "sh".to_owned()
    }
}
