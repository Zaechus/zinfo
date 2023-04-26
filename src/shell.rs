#[cfg(not(windows))]
pub fn get_shell() -> String {
    use std::env;

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

#[cfg(windows)]
pub fn get_shell() -> &'static str {
    "cmd"
}
