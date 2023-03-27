use std::{env, io::stdout, iter};

use crossterm::{self, style::Stylize, tty::IsTty};

use zinfo::*;

fn main() -> crossterm::Result<()> {
    let (os_id, os_name) = get_os()?;
    let os_id = if let Some(arg) = env::args().nth(1) {
        arg.to_lowercase()
    } else {
        os_id
    };

    let (logo, logo_color) = logo(&os_id);

    print!(
        "{}",
        if stdout().is_tty() {
            [
                format!(
                    "{}@{}",
                    whoami().bold().with(logo_color),
                    hostname().bold().with(logo_color)
                ),
                format!("{}{}", "os    ".with(logo_color), os_name),
                format!("{}{}", "kver  ".with(logo_color), get_kver()),
                format!("{}{}", "up    ".with(logo_color), uptime()),
                format!("{}{}", "sh    ".with(logo_color), get_shell()),
                format!("{}{}", "mem   ".with(logo_color), get_mem()?),
            ]
            .iter()
            .zip(logo.into_iter().chain(iter::repeat("          ")))
            .map(|x| format!("{}  {}\n", x.1.bold().with(logo_color), x.0))
            .collect::<String>()
        } else {
            [
                format!("{}@{}", whoami(), hostname()),
                format!("os    {}", os_name),
                format!("kver  {}", get_kver()),
                format!("up    {}", uptime()),
                format!("sh    {}", get_shell()),
                format!("mem   {}", get_mem()?),
            ]
            .iter()
            .zip(logo.into_iter().chain(iter::repeat("          ")))
            .map(|x| format!("{}  {}\n", x.1, x.0))
            .collect::<String>()
        }
    );

    Ok(())
}
