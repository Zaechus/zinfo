run:
    cargo run --release

x:
    cargo clippy --target x86_64-unknown-linux-gnu
    cargo clippy --target x86_64-unknown-linux-musl
    cargo clippy --target aarch64-linux-android
    cargo clippy --target x86_64-unknown-freebsd
    cargo clippy --target x86_64-pc-windows-gnu
    cargo clippy --target x86_64-pc-windows-msvc
