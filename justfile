run:
    cargo run --release

t:
    cargo t
    cargo clippy

x:
    cargo clippy
    cargo clippy --target aarch64-linux-android
    cargo clippy --target x86_64-unknown-freebsd
    cargo clippy --target x86_64-pc-windows-gnu
    cargo clippy --target x86_64-pc-windows-msvc
