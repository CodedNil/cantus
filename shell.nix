{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell rec {
  nativeBuildInputs = with pkgs; [
    pkg-config
    rustc
    cargo
    rustfmt
    clippy
    mold
  ];

  buildInputs = with pkgs; [
    wayland
    dbus
    libxkbcommon
    vulkan-loader
    libGL
    fontconfig
    openssl
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
}
