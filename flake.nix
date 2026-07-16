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
              overlays = [ rust-overlay.overlays.default ];
            }
          )
        );
      runtimeLibraries =
        pkgs: with pkgs; [
          wayland
          vulkan-loader
        ];
    in
    {
      packages = forAllSystems (pkgs: rec {
        default = cantus;
        cantus = pkgs.rustPlatform.buildRustPackage {
          inherit pname;
          version = (lib.importTOML ./crates/cantus_cpu/Cargo.toml).package.version;

          src = lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;

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
      });

      devShells = forAllSystems (pkgs: {
        default =
          let
            shaderRust = pkgs.rust-bin.nightly."2026-05-22".default.override {
              extensions = [
                "rust-src"
                "rustc-dev"
                "llvm-tools"
              ];
            };
          in
          pkgs.mkShell {
            name = pname;
            packages = with pkgs; [
              cargo
              rustc
              rustfmt
              clippy
              mold
              pkg-config
              just
            ];
            buildInputs = runtimeLibraries pkgs;
            CANTUS_SHADER_RUST = shaderRust;
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
            setting =
              type: default: description:
              mkOption { inherit type default description; };
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

              autoStart = setting types.bool true "Whether to start the Cantus widget automatically.";

              settings = mkOption {
                type = types.nullOr (
                  types.submodule {
                    options = {
                      spotify_client_id =
                        setting (types.nullOr types.str) null
                          "Spotify client ID to use for authentication.";

                      monitor = setting (types.nullOr types.str) null "Monitor to display Cantus on.";

                      location =
                        setting (types.addCheck (types.listOf types.number) (coordinates: builtins.length coordinates == 2))
                          [
                            51.5074
                            (-0.1278)
                          ]
                          "Latitude and longitude used for weather.";

                      height = setting types.number 50.0 "Height of the timeline in logical pixels.";

                      layer = setting (types.enum [
                        "background"
                        "bottom"
                        "top"
                        "overlay"
                      ]) "top" "Layer the app should be displayed on.";

                      layer_anchor = setting (types.enum [
                        "top"
                        "bottom"
                      ]) "top" "Screen edge the app should anchor to.";

                      timeline_future_minutes =
                        setting types.number 12.0
                          "Minutes in the future to display in the timeline.";

                      timeline_past_minutes =
                        setting types.number 1.5
                          "Minutes before the current time to display in the timeline.";

                      history_width =
                        setting types.number 100.0
                          "Width in logical pixels where previous tracks are displayed.";

                      playlists = mkOption {
                        type = types.addCheck (types.listOf types.str) (items: builtins.length items <= 8) // {
                          description = "list of strings with at most 8 entries";
                        };
                        default = [ ];
                        description = "Favourite playlists to display as buttons.";
                      };

                      ratings_enabled = setting types.bool false "Whether star ratings should be enabled.";
                    };
                  }
                );
                default = null;
                description = "Settings written as TOML to `~/.config/cantus/cantus.toml`.";
                example = {
                  monitor = "eDP-1";
                  location = [
                    51.5
                    (-0.1)
                  ];
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
