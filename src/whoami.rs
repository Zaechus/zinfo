#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
use crate::get_output;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub fn whoami() -> String {
    use std::env;

    if let Ok(var) = env::var("USER") {
        var
    } else {
        "user".to_owned()
    }
}

#[cfg(windows)]
pub fn whoami() -> String {
    get_output("cmd", &["/C", "whoami"])
        .unwrap_or("user".to_owned())
        .split('\\')
        .last()
        .unwrap_or("user")
        .to_owned()
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd", windows)))]
pub fn whoami() -> String {
    get_output("whoami", &[]).unwrap_or("user".to_owned())
}
