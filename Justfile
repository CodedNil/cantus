rust_gpu_bin := `dirname "$(rustup which --toolchain nightly-2026-05-22 rustc)"`
shader_output := "assets/cantus.spv"

default: run

shader:
    @if [ ! -f "{{ shader_output }}" ] || find crates/cantus_gpu/src crates/cantus_shared/src crates/cantus_gpu/Cargo.toml crates/cantus_shared/Cargo.toml -type f -newer "{{ shader_output }}" | grep -q .; then \
        env -u RUSTC -u RUSTDOC -u RUSTUP_TOOLCHAIN \
            PATH="{{ rust_gpu_bin }}:$PATH" \
            cargo run \
                --manifest-path crates/cantus_gpu/Cargo.toml \
                --features build-shader \
                --bin build-shader; \
    else \
        echo "{{ shader_output }} is up to date"; \
    fi

run: shader
    cargo run -p cantus_cpu
