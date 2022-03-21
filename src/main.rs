use std::{env, io::stdout};

use crossterm::{
    self,
    style::{style, Attribute, ResetColor, Stylize},
    ExecutableCommand,
};

use zinfo::*;

fn main() -> crossterm::Result<()> {
    let system = SysInfo::new();

    let os_name = system.get_os_name();

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    let os_id = if let Some(arg) = env::args().nth(1) {
        arg.to_lowercase()
    } else {
        system.get_os_info("ID")
    };
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    let os_id = if let Some(arg) = env::args().nth(1) {
        arg.to_lowercase()
    } else {
        os_name
            .split_whitespace()
            .next()
            .unwrap_or(&os_name)
            .to_lowercase()
    };

    let (logo, logo_color) = logo(&os_id);

    let info = [
        format!(
            "{}@{}",
            style(system.whoami())
                .with(logo_color)
                .attribute(Attribute::Bold),
            style(get_hostname())
                .with(logo_color)
                .attribute(Attribute::Bold)
        ),
        format!("{}{}", style("os    ").with(logo_color), os_name),
        format!("{}{}", style("kver  ").with(logo_color), get_kver()),
        format!("{}{}", style("up    ").with(logo_color), get_uptime()),
        format!("{}{}", style("sh    ").with(logo_color), system.get_shell()),
        if let Ok(m) = get_mem() {
            format!("{}{}", style("mem   ").with(logo_color), m)
        } else {
            "0M / 0M".to_owned()
        },
    ];

    for (x, item) in info.iter().enumerate() {
        println!(
            "{}  {}",
            style(logo.get(x + 1).unwrap_or(&"          "))
                .with(logo_color)
                .attribute(Attribute::Bold),
            item
        )
    }

    stdout().execute(ResetColor)?;

    Ok(())
}
