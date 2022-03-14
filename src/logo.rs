use crossterm::style::Color;

pub fn logo(os_id: &str) -> (Vec<&str>, Color) {
    match os_id {
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
    }
}
