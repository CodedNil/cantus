default: run

shader:
    PATH="$(nix build --no-link --print-out-paths .#rust-nightly)/bin:$PATH" cargo run -p isthmus-build --features compiler --bin isthmus -- build crates/cantus

run: shader
    cargo run -p cantus

schema:
    cargo run -p cantus --features generate-nix

launcher:
    ./target/debug/cantus --launcher
