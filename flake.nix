{
  description = "Weather station wrapper: Ambient Weather websocket behind a cached REST API";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "weather_station_wrapper";
          version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
          src = nixpkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
          meta.mainProgram = "weather_station_wrapper";
        };
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = [ self.packages.${pkgs.stdenv.hostPlatform.system}.default ];
          packages = with pkgs; [
            clippy
            rustfmt
            rust-analyzer
          ];
        };
      });

      nixosModules.default = import ./nix/module.nix self;

      checks = forAllSystems (pkgs: {
        package = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
        # Evaluates the module so option typos fail `nix flake check`.
        module =
          (nixpkgs.lib.nixosSystem {
            system = pkgs.stdenv.hostPlatform.system;
            modules = [
              self.nixosModules.default
              {
                boot.isContainer = true;
                system.stateVersion = "25.11";
                services.weather-station-wrapper = {
                  enable = true;
                  apiKeyFile = "/run/secrets/api_key";
                  applicationKeyFile = "/run/secrets/application_key";
                };
              }
            ];
          }).config.systemd.units."weather-station-wrapper.service".unit;
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt);
    };
}
