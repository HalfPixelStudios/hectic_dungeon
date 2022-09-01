
default: debug

debug:
    cargo run -- -e -s ingame

web:
    cargo watch -cx "run --release --target wasm32-unknown-unknown"

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

chk:
    cargo check

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo

lint-fix:
    cargo clippy --fix -- -W clippy::unwrap_used -W clippy::cargo
