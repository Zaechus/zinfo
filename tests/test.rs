#[cfg(test)]
mod tests {
    #[test]
    fn still_works() {
        let testing = String::from_utf8(
            std::process::Command::new("./target/debug/zinfo")
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        let installed =
            String::from_utf8(std::process::Command::new("zinfo").output().unwrap().stdout)
                .unwrap();

        assert_eq!(testing, installed);
        assert_ne!(testing, "");
    }
}
