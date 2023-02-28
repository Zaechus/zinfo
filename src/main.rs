use std::{env, io::stdout};

use crossterm::{
    self,
    style::{style, Attribute, ResetColor, Stylize},
    ExecutableCommand,
};

use zinfo::*;

fn main() -> crossterm::Result<()> {
    let (os_id, os_name) = get_os()?;
    let os_id = if let Some(arg) = env::args().nth(1) {
        arg.to_lowercase()
    } else {
        os_id
    };

    let (logo, logo_color) = logo(&os_id);

    [
        format!(
            "{}@{}",
            style(whoami()).with(logo_color).attribute(Attribute::Bold),
            style(hostname())
                .with(logo_color)
                .attribute(Attribute::Bold)
        ),
        format!("{}{}", style("os    ").with(logo_color), os_name),
        format!("{}{}", style("kver  ").with(logo_color), get_kver()),
        format!("{}{}", style("up    ").with(logo_color), uptime()),
        format!("{}{}", style("sh    ").with(logo_color), get_shell()),
        if let Ok(m) = get_mem() {
            format!("{}{}", style("mem   ").with(logo_color), m)
        } else {
            "0M / 0M".to_owned()
        },
    ]
    .iter()
    .enumerate()
    .for_each(|(x, item)| {
        println!(
            "{}  {}",
            style(logo.get(x).unwrap_or(&"          "))
                .with(logo_color)
                .attribute(Attribute::Bold),
            item
        )
    });

    stdout().execute(ResetColor)?;

    Ok(())
}
