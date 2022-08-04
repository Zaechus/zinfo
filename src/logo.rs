use crossterm::style::Color;

pub fn logo(os_id: &str) -> (Vec<&str>, Color) {
    match os_id {
        "alpine" => (
            vec![
                r#"  ______  "#,
                r#" /      \ "#,
                r#"/  /\/\  \"#,
                r#"\ /, \ \ /"#,
                r#" \______/ "#,
            ],
            Color::Blue,
        ),
        "arch" | "artix" => (
            vec![
                r#"          "#,
                r#"    /\    "#,
                r#"   /` \   "#,
                r#"  / __'\  "#,
                r#" /-'  '-\ "#,
            ],
            Color::Cyan,
        ),
        "debian" => (
            vec![
                r#"    ___   "#,
                r#"   /   \  "#,
                r#"  |  (_/  "#,
                r#"   \      "#,
                r#"    `.    "#,
            ],
            Color::Red,
        ),
        "fedora" => (
            vec![
                r#"         "#,
                r#"    ffff "#,
                r#"    f    "#,
                r#"  fffff  "#,
                r#"    f    "#,
                r#" ffff    "#,
            ],
            Color::Blue,
        ),
        "gentoo" => (
            vec![
                r#"   _____  "#,
                r#"  /     \ "#,
                r#"  \  0   )"#,
                r#"  /     / "#,
                r#"  \____/  "#,
            ],
            Color::Magenta,
        ),
        "manjaro" => (
            vec![
                r#" ##### ## "#,
                r#" ##### ## "#,
                r#" ##    ## "#,
                r#" ## ## ## "#,
                r#" ## ## ## "#,
                r#" ## ## ## "#,
            ],
            Color::Green,
        ),
        "nixos" => (
            vec![
                r#"          "#,
                r#"  _\_\/   "#,
                r#" __/  \/_ "#,
                r#"  /\__/_  "#,
                r#"   /\ \   "#,
            ],
            Color::Blue,
        ),
        "opensuse-leap" => (
            vec![
                r#"          "#,
                r#"    .'.   "#,
                r#"  ,`   `, "#,
                r#"  `.`.`.` "#,
                r#"    `.`   "#,
            ],
            Color::Green,
        ),
        "opensuse-tumbleweed" => (
            vec![
                r#"          "#,
                r#"          "#,
                r#"  ,-, ,-, "#,
                r#" (   X   )"#,
                r#"  '-' '-' "#,
            ],
            Color::Green,
        ),
        "ubuntu" => (
            vec![
                r#"          "#,
                r#"  , ---() "#,
                r#" /   _   \"#,
                r#"()  (_)   "#,
                r#" \       /"#,
                r#"  ` ---() "#,
            ],
            Color::DarkRed,
        ),
        "void" => (
            vec![
                r#"          "#,
                r#"  ,-'''-, "#,
                r#" /       \"#,
                r#"  V O I D "#,
                r#" \       /"#,
                r#"  `-...-` "#,
            ],
            Color::Green,
        ),
        "freebsd" => (
            vec![
                r#"          "#,
                r#"-.,____,.-"#,
                r#"\/    ', /"#,
                r#"|       `|"#,
                r#"`,      ,'"#,
                r#"  `----`  "#,
            ],
            Color::Red,
        ),
        "android" => (
            vec![
                r#"          "#,
                r#"  \_____/ "#,
                r#"  / o o \ "#,
                r#" |_______|"#,
            ],
            Color::Green,
        ),
        "windows" => (
            vec![
                r#"          "#,
                r#"  ... ... "#,
                r#"  ''' ''' "#,
                r#"  ... ... "#,
                r#"  ''' ''' "#,
            ],
            Color::Cyan,
        ),
        _ => (
            vec![
                r#"          "#,
                r#"  ####### "#,
                r#"    ####  "#,
                r#"   ####   "#,
                r#"  ####### "#,
            ],
            Color::Yellow,
        ),
    }
}
