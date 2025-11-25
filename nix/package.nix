{
  rustPlatform,
  lib,
  pkg-config,
  makeWrapper,
  wayland,
  vulkan-loader,
  libxkbcommon,
  openssl,
}:
rustPlatform.buildRustPackage rec {
  pname = "cantus";
  version = (builtins.fromTOML (builtins.readFile ../Cargo.toml)).package.version;

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../Cargo.toml
      ../Cargo.lock
      ../src
      ../assets
    ];
  };

  cargoLock.lockFile = "${src}/Cargo.lock";

  nativeBuildInputs = [
    pkg-config
    makeWrapper
  ];

  buildInputs = [
    wayland
    vulkan-loader
    libxkbcommon
    openssl
  ];

  postInstall = "wrapProgram $out/bin/cantus --set LD_LIBRARY_PATH ${lib.makeLibraryPath buildInputs}";

  meta = {
    description = "A beautiful interactive music widget for wayland";
    homepage = "https://github.com/CodedNil/cantus";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ CodedNil ];
    platforms = lib.platforms.linux;
    mainProgram = "cantus";
  };
}
