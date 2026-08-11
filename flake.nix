rec {
  description = "A beautiful interactive music widget for wayland";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      pname = "cantus";
      forAllSystems =
        f:
        lib.genAttrs [ "x86_64-linux" "aarch64-linux" ] (
          system:
          f (
            import nixpkgs {
              inherit system;
              overlays = [ rust-overlay.overlays.default ];
            }
          )
        );
      runtimeLibraries =
        pkgs: with pkgs; [
          wayland
          vulkan-loader
          libxkbcommon
        ];
      runtimeTools =
        pkgs: with pkgs; [
          pipewire
          wireplumber
        ];
      stableRust =
        pkgs:
        pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "clippy"
            "rustfmt"
          ];
        };
      shaderRust =
        pkgs:
        pkgs.rust-bin.nightly."2026-05-22".default.override {
          extensions = [
            "rust-src"
            "rustc-dev"
            "llvm-tools"
          ];
        };
    in
    {
      packages = forAllSystems (pkgs: rec {
        default = cantus;
        rust-stable = stableRust pkgs;
        rust-nightly = shaderRust pkgs;
        cantus = pkgs.rustPlatform.buildRustPackage {
          inherit pname;
          version = (lib.importTOML ./crates/cantus/Cargo.toml).package.version;

          src = lib.cleanSource ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "rustc_codegen_spirv-0.10.0-alpha.1" = "sha256-M3/puV8CnGDp4I4C/F4lrH/Dfbs6Lj4T4j4vwdBMzrU=";
              "sysinfo-0.39.6" = "sha256-HgD13E5L5Gtwj1I1mD+vU3ln0dfj61Zeet8LHyUIdkk=";
            };
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper
            mold
          ];

          buildInputs = runtimeLibraries pkgs;

          postInstall = ''
            wrapProgram "$out/bin/${pname}" \
              --set LD_LIBRARY_PATH "${lib.makeLibraryPath (runtimeLibraries pkgs)}" \
              --prefix PATH : "${lib.makeBinPath (runtimeTools pkgs)}"
          '';

          meta = {
            inherit description;
            homepage = "https://github.com/CodedNil/cantus";
            license = lib.licenses.mit;
            maintainers = with lib.maintainers; [ CodedNil ];
            platforms = lib.platforms.linux;
            mainProgram = pname;
          };
        };
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          name = pname;
          packages = with pkgs; [
            (stableRust pkgs)
            mold
            pkg-config
            just
            nixfmt
            pipewire
            wireplumber
          ];
          buildInputs = runtimeLibraries pkgs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (runtimeLibraries pkgs);
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt);

      homeManagerModules = {
        default = self.homeManagerModules.cantus;
        cantus =
          {
            config,
            lib,
            pkgs,
            ...
          }:
          let
            inherit (lib)
              literalExpression
              mkEnableOption
              mkIf
              mkOption
              optional
              optionalAttrs
              types
              ;

            cfg = config.programs.cantus;
            settingsFormat = pkgs.formats.toml { };
            settingsOptions = import ./generated-options.nix { inherit lib; };
            settingsDefaults = lib.mapAttrs (_: option: option.default) settingsOptions;
          in
          {
            options.programs.cantus = {
              enable = mkEnableOption description;

              package = mkOption {
                type = types.package;
                default = self.packages.${pkgs.stdenv.hostPlatform.system}.cantus;
                defaultText = literalExpression "inputs.${pname}.packages.\${pkgs.stdenv.hostPlatform.system}.${pname}";
                description = "Cantus package to install.";
              };

              autoStart = mkOption {
                type = types.bool;
                default = true;
                description = "Whether to start the Cantus widget automatically.";
              };

              settings = mkOption {
                type = types.nullOr (
                  types.submodule {
                    options = settingsOptions;
                  }
                );
                default = null;
                description = "Settings written as TOML to `~/.config/cantus/cantus.toml`.";
                example = settingsDefaults;
              };
            };

            config = mkIf cfg.enable {
              home.packages = [ cfg.package ];

              xdg.configFile = optionalAttrs (cfg.settings != null) {
                "cantus/cantus.toml".source = settingsFormat.generate "cantus.toml" (
                  lib.filterAttrs (_: value: value != null) cfg.settings
                );
              };

              systemd.user.services.cantus = mkIf cfg.autoStart {
                Unit = {
                  Description = description;
                  After = [ config.wayland.systemd.target ];
                  X-Restart-Triggers = optional (
                    cfg.settings != null
                  ) config.xdg.configFile."cantus/cantus.toml".source;
                };

                Service = {
                  Type = "simple";
                  ExecStart = "${cfg.package}/bin/${pname}";
                  Restart = "on-failure";
                };

                Install.WantedBy = [ config.wayland.systemd.target ];
              };
            };
          };
      };
    };
}
