use std::{env, io::stdout};

#[cfg(target_os = "linux")]
use std::fs;

use crossterm::{
    style::{style, Attribute, ResetColor, Stylize},
    ExecutableCommand, Result,
};

use zinfo::*;

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
    let hostname = if let Ok(o) = get_output("hostname", &[]) {
        o
    } else {
        "hostname"
    };

    let os_name = system.get_os_name();

    #[cfg(target_os = "linux")]
    let os_id = if let Some(arg) = args.get(1) {
        arg.to_owned()
    } else {
        system.get_os_info("ID")
    };
    #[cfg(not(target_os = "linux"))]
    let os_id = if let Some(arg) = args.get(1) {
        arg.to_owned()
    } else {
        os_name.clone()
    };

    let logo = logo(&os_id);

    let memory = if let Ok(m) = get_mem() {
        m
    } else {
        "0M / 0M".to_owned()
    };

    let info = [
        format!("{}{}", style("os    ").with(logo.1), os_name),
        format!("{}{}", style("kver  ").with(logo.1), get_kver()),
        format!("{}{}", style("up    ").with(logo.1), get_uptime()),
        format!("{}{}", style("sh    ").with(logo.1), system.get_shell()),
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
