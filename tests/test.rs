#[cfg(test)]
mod tests {
    #[test]
    fn still_works() {
        for distro in [
            "alpine",
            "arch",
            "artix",
            "debian",
            "fedora",
            "gentoo",
            "nixos",
            "opensuse-leap",
            "opensuse-tumbleweed",
            "ubuntu",
            "void",
            "freebsd",
            "android",
            "windows",
            "zinfo",
        ] {
            let testing = String::from_utf8(
                std::process::Command::new("./target/debug/zinfo")
                    .arg(distro)
                    .output()
                    .unwrap()
                    .stdout,
            )
            .unwrap();
            let installed = String::from_utf8(
                std::process::Command::new("zinfo")
                    .arg(distro)
                    .output()
                    .unwrap()
                    .stdout,
            )
            .unwrap();

            assert_eq!(testing, installed);
        }
    }
}
