use std::{env, fs, io::stdout};

use crossterm::{
    style::{style, Attribute, ResetColor, Stylize},
    ExecutableCommand, Result,
};

use zinfo::*;

#[cfg(target_os = "windows")]
use zinfo::get_os_name;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    let system = SysInfo::new();

    #[cfg(target_os = "linux")]
    let hostname = if let Ok(s) = fs::read_to_string("/etc/hostname") {
        s.trim().to_owned()
    } else {
        "hostname".to_owned()
    };
    #[cfg(not(target_os = "linux"))]
    let hostname = get_output("hostname", &[], "hostname");

    let os_release = if let Ok(s) = fs::read_to_string("/etc/os-release") {
        s
    } else {
        "ID=linux\nPRETTYNAME=\"Linux\"".to_owned()
    };

    #[cfg(target_os = "linux")]
    let os_name = get_os_info(&os_release, "PRETTY_NAME");
    #[cfg(target_os = "windows")]
    let os_name = get_os_name();
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    let os_name = get_output("uname", &["-o"], "Linux");

    let os_id = if let Some(arg) = args.get(1) {
        arg.to_owned()
    } else if cfg!(target_os = "linux") {
        get_os_info(&os_release, "ID")
    } else {
        os_name.clone()
    };

    let logo = logo(&os_id);

    #[cfg(target_os = "linux")]
    let kver = if let Ok(ver) = fs::read_to_string("/proc/version") {
        ver.split(' ').nth(2).unwrap_or("linux").to_owned()
    } else {
        "linux".to_owned()
    };
    #[cfg(not(target_os = "linux"))]
    let kver = get_output("uname", &["-r"], "linux");

    let uptime = get_uptime();

    #[cfg(target_os = "windows")]
    let shell = "cmd";
    #[cfg(not(target_os = "windows"))]
    let shell = get_shell(system.envvars());

    let memory = if let Ok(m) = get_mem() {
        m
    } else {
        "0M / 0M".to_owned()
    };

    let info = [
        format!("{}{}", style("os    ").with(logo.1), os_name),
        format!("{}{}", style("kver  ").with(logo.1), kver),
        format!("{}{}", style("up    ").with(logo.1), uptime),
        format!("{}{}", style("sh    ").with(logo.1), shell),
        format!("{}{}", style("mem   ").with(logo.1), memory),
    ];

    println!(
        "{}  {}@{}",
        style(logo.0[1]).with(logo.1).attribute(Attribute::Bold),
        style(system.whoami())
            .with(logo.1)
            .attribute(Attribute::Bold),
        style(hostname).with(logo.1).attribute(Attribute::Bold)
    );
    for (x, item) in info.iter().enumerate() {
        println!(
            "{}  {}",
            style(logo.0.get(x + 2).unwrap_or(&"          "))
                .with(logo.1)
                .attribute(Attribute::Bold),
            item
        )
    }
    stdout().execute(ResetColor)?;

    Ok(())
}
