shader_target_dir := "target/cantus-gpu"
shader_output := "assets/cantus.spv"

default: run

fmt:
    cargo fmt --all
    cargo fmt --manifest-path crates/cantus_gpu/Cargo.toml
    nixfmt flake.nix generated-options.nix

update:
    cargo update
    cargo update --manifest-path crates/cantus_gpu/Cargo.toml
alias u := update

shader:
    @if [ ! -f "{{ shader_output }}" ] || find crates/cantus_gpu/src crates/cantus_shared/src crates/cantus_gpu/Cargo.toml crates/cantus_shared/Cargo.toml -type f -newer "{{ shader_output }}" | grep -q .; then \
        env -u RUSTC -u RUSTDOC -u RUSTUP_TOOLCHAIN \
            CARGO_TARGET_DIR="{{ shader_target_dir }}" \
            PATH="$CANTUS_SHADER_RUST/bin:$PATH" \
            "$CANTUS_SHADER_RUST/bin/cargo" run \
                --manifest-path crates/cantus_gpu/Cargo.toml \
                --features build-shader \
                --bin build-shader; \
    else \
        echo "{{ shader_output }} is up to date"; \
    fi

run: shader
    cargo run -p cantus_cpu --features generate-nix
