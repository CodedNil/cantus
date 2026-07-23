default: run

shader:
    CARGO_TARGET_DIR=target/cantus-gpu \
        PATH="$CANTUS_SHADER_RUST/bin:$PATH" \
        cargo run --quiet -p cantus_gpu --features build-shader --bin build-shader

run: shader
    cargo run -p cantus_cpu --features generate-nix
