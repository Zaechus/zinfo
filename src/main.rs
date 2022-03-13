use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, stdout, BufRead, BufReader},
};

#[cfg(not(target_os = "linux"))]
use std::process::Command;

use crossterm::{
    style::{style, Attribute, Color, ResetColor, Stylize},
    ExecutableCommand, Result,
};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let envvars: HashMap<_, _> = env::vars().collect();

    #[cfg(target_os = "linux")]
    let username = if let Some(var) = envvars.get("USER") {
        var
    } else {
        "user"
    };
    #[cfg(not(target_os = "linux"))]
    let username = get_output("whoami", &[], "user");

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
    #[cfg(not(target_os = "linux"))]
    let os_name = get_output("uname", &["-o"], "Linux");

    let os_id = if let Some(arg) = args.get(1) {
        arg.to_owned()
    } else if cfg!(target_os = "linux") {
        get_os_info(&os_release, "ID")
    } else {
        os_name.clone()
    };

    let logo: (Vec<_>, Color) = match os_id.as_str() {
        "alpine" => (
            r#"
  ______  
 /      \ 
/  /\/\  \
\ /, \ \ /
 \______/ "#
                .split('\n')
                .collect(),
            Color::Blue,
        ),
        "arch" => (
            r#" 
          
    /\    
   /` \   
  / __'\  
 /-'  '-\ "#
                .split('\n')
                .collect(),
            Color::Cyan,
        ),

        "gentoo" => (
            r#"
   _____  
  /     \ 
  \  0   )
  /     / 
  \____/  "#
                .split('\n')
                .collect(),
            Color::Magenta,
        ),
        "manjaro" => (
            r#"
 ##### ## 
 ##### ## 
 ##    ## 
 ## ## ## 
 ## ## ## 
 ## ## ## "#
                .split('\n')
                .collect(),
            Color::Green,
        ),
        "nixos" => (
            r#"
          
  _\_\/   
 __/  \/_ 
  /\__/_  
   /\ \   "#
                .split('\n')
                .collect(),
            Color::Blue,
        ),
        "opensuse-leap" => (
            r#"
          
    .'.   
  ,`   `, 
  `.`.`.` 
    `.`   "#
                .split('\n')
                .collect(),
            Color::Green,
        ),
        "opensuse-tumbleweed" => (
            r#"
          
          
  ,-, ,-, 
 (   X   )
  '-' '-' "#
                .split('\n')
                .collect(),
            Color::Green,
        ),
        "ubuntu" => (
            r#"
          
  , ---() 
 /   _   \
()  (_)   
 \       /
  ` ---() "#
                .split('\n')
                .collect(),
            Color::DarkRed,
        ),
        "void" => (
            r#"
          
  ,-'''-, 
 /       \
  V O I D 
 \       /
  `-...-` "#
                .split('\n')
                .collect(),
            Color::Green,
        ),
        "Android" => (
            r#"
          
  \_____/ 
  / o o \ 
 |_______|"#
                .split('\n')
                .collect(),
            Color::Green,
        ),
        _ => (
            r#"
          
  ####### 
    ####  
   ####   
  ####### "#
                .split('\n')
                .collect(),
            Color::White,
        ),
    };

    #[cfg(target_os = "linux")]
    let kver = if let Ok(ver) = fs::read_to_string("/proc/version") {
        ver.split(' ').nth(2).unwrap_or("linux").to_owned()
    } else {
        "linux".to_owned()
    };
    #[cfg(not(target_os = "linux"))]
    let kver = get_output("uname", &["-r"], "linux");

    let uptime = get_uptime();
    let shell = get_shell(&envvars);
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
        style(username).with(logo.1).attribute(Attribute::Bold),
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

#[cfg(not(target_os = "linux"))]
fn get_output(command: &str, args: &[&str], default: &str) -> String {
    if let Ok(output) = Command::new(command).args(args).output() {
        if let Ok(s) = String::from_utf8(output.stdout) {
            s.trim().to_owned()
        } else {
            default.to_owned()
        }
    } else {
        default.to_owned()
    }
}

fn get_os_info(os_release: &str, key: &str) -> String {
    if let Some(pretty_name) = os_release
        .split('\n')
        .map(|line| line.split('=').collect::<Vec<_>>())
        .find(|line| line[0] == key)
    {
        pretty_name[1].trim_matches('"').to_owned()
    } else {
        "Linux".to_owned()
    }
}

fn get_shell(envvars: &HashMap<String, String>) -> String {
    if let Some(var) = envvars.get("SHELL") {
        if let Some(last_slash) = var.rfind('/') {
            &var[last_slash + 1..]
        } else {
            var
        }
    } else {
        "sh"
    }
    .to_owned()
}

fn get_uptime() -> String {
    if let Ok(uptime) = fs::read_to_string("/proc/uptime") {
        let mut seconds = uptime
            .split(' ')
            .next()
            .unwrap_or("0")
            .to_owned()
            .parse::<f64>()
            .unwrap_or(0.0) as i32;

        let days = seconds / 86400;
        seconds -= days * 86400;

        let hours = seconds / 3600;
        seconds -= hours * 3600;

        let minutes = seconds / 60;
        seconds -= minutes * 60;

        format!(
            "{}{}{}{}s",
            if days == 0 {
                String::default()
            } else {
                format!("{}d ", days)
            },
            if hours == 0 {
                String::default()
            } else {
                format!("{}h ", hours)
            },
            if minutes == 0 {
                String::default()
            } else {
                format!("{}m ", minutes)
            },
            seconds
        )
    } else {
        "0 s".to_owned()
    }
}

fn get_mem() -> io::Result<String> {
    let meminfo = File::open("/proc/meminfo")?;
    let reader = BufReader::new(meminfo).lines();

    let mut total = 0;
    let mut free = 0;
    let mut buffers = 0;
    let mut cached = 0;

    for line in reader {
        let l: Vec<String> = line?.split_whitespace().map(str::to_owned).collect();
        match l[0].as_str() {
            "MemTotal:" => {
                total = l[1].parse::<i32>().unwrap_or(0);
            }
            "MemFree:" => {
                free = l[1].parse::<i32>().unwrap_or(0);
            }
            "Buffers" => {
                buffers = l[1].parse::<i32>().unwrap_or(0);
            }
            "Cached:" => {
                cached = l[1].parse::<i32>().unwrap_or(0);
                break;
            }
            _ => (),
        }
    }

    Ok(format!(
        "{}M / {}M",
        (total - free - buffers - cached) / 1024,
        total / 1024
    ))
}
