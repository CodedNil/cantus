{
  description = "A beautiful interactive music widget for wayland";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      eachSystem = f: lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});
    in
    {
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          name = "cantus";
          inputsFrom = [ self.packages.${pkgs.stdenv.system}.cantus ];
          shellHook = ''
            export LD_LIBRARY_PATH=${
              pkgs.lib.makeLibraryPath [
                pkgs.wayland
                pkgs.vulkan-loader
              ]
            }:$LD_LIBRARY_PATH
          '';
        };
      });

      packages = eachSystem (pkgs: {
        default = self.packages.${pkgs.stdenv.system}.cantus;
        cantus = pkgs.callPackage ./nix/package.nix { };
      });

      homeManagerModules = {
        default = self.homeManagerModules.cantus;
        cantus = import ./nix/module.nix { inherit self; };
      };
    };
}
