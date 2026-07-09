rec {
  description = "A beautiful interactive music widget for wayland";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      crane,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      pname = "cantus";
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems =
        f:
        lib.genAttrs supportedSystems (
          system:
          f (
            import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            }
          )
        );
      runtimeLibraries =
        pkgs: with pkgs; [
          wayland
          vulkan-loader
          libxkbcommon
        ];
    in
    {
      packages = forAllSystems (
        pkgs:
        let
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (crane.mkLib pkgs).overrideToolchain (_: toolchain);
          cantus = craneLib.buildPackage {
            inherit pname;
            version = (lib.importTOML ./crates/cantus_cpu/Cargo.toml).package.version;

            src = lib.cleanSource ./.;
            cargoVendorDir = craneLib.vendorMultipleCargoDeps {
              cargoLockList = [
                ./Cargo.lock
                "${toolchain.passthru.availableComponents.rust-src}/lib/rustlib/src/rust/library/Cargo.lock"
              ];
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
              makeWrapper
              mold
            ];

            buildInputs = runtimeLibraries pkgs;

            postInstall = ''
              wrapProgram "$out/bin/${pname}" --set LD_LIBRARY_PATH "${lib.makeLibraryPath (runtimeLibraries pkgs)}"
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
        in
        {
          default = cantus;
          inherit cantus;
        }
      );

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          name = pname;
          packages = with pkgs; [
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            mold
            pkg-config
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
            number = types.either types.int types.float;
            settingsFormat = pkgs.formats.toml { };

            option =
              type: default: description:
              mkOption {
                inherit type default description;
              };

            nullableString = default: description: option (types.nullOr types.str) default description;
            numeric = option number;
            enum = values: option (types.enum values);
          in
          {
            options.programs.cantus = {
              enable = mkEnableOption "${pname}, ${description}";

              package = mkOption {
                type = types.package;
                default = self.packages.${pkgs.stdenv.hostPlatform.system}.cantus;
                defaultText = literalExpression "inputs.${pname}.packages.\${pkgs.stdenv.hostPlatform.system}.${pname}";
                description = "Cantus package to install.";
              };

              autoStart = option types.bool true "Whether to start the Cantus widget automatically.";

              settings = mkOption {
                type = types.nullOr (
                  types.submodule {
                    options = {
                      spotify_client_id = nullableString null "Spotify client ID to use for authentication.";
                      monitor = nullableString null "Monitor to display Cantus on.";
                      width = numeric 1050.0 "Width of the timeline in pixels.";
                      height = numeric 50.0 "Height of the timeline in pixels.";
                      layer = enum [
                        "background"
                        "bottom"
                        "top"
                        "overlay"
                      ] "top" "Layer shell layer to display Cantus on.";
                      layer_anchor = enum [
                        "top"
                        "bottom"
                      ] "top" "Screen edge Cantus should anchor to.";
                      timeline_future_minutes = numeric 12.0 "Minutes in the future to display in the timeline.";
                      timeline_past_minutes = numeric 1.5 "Minutes before the current time to display in the timeline.";
                      history_width = numeric 100.0 "Width in pixels where previous tracks are displayed.";
                      playlists = option (types.listOf types.str) [ ] "Favourite playlists to display as buttons.";
                      ratings_enabled = option types.bool false "Whether star ratings should be enabled.";
                    };
                  }
                );
                default = null;
                description = "Settings written as TOML to `~/.config/cantus/cantus.toml`.";
                example = {
                  monitor = "eDP-1";
                  width = 1050.0;
                  height = 40.0;
                  timeline_future_minutes = 12.0;
                  timeline_past_minutes = 1.5;
                  history_width = 100.0;
                  playlists = [
                    "Rock & Roll"
                    "Instrumental"
                    "Pop"
                  ];
                  ratings_enabled = true;
                };
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
