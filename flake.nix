
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/357cd3dfdb8993af11268d755d53357720675e66";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, nixpkgs-unstable, utils }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            config.allowUnfree = true;
            overlays = [ ];
          };
          pkgs-unstable = import nixpkgs-unstable {
            inherit system;
            config.allowUnfree = true;
            overlays = [ ];
          };
        in
        {
          devShells.default =
            with pkgs;
            mkShell {
              buildInputs =
                [
                  # Rust
                  pkgs-unstable.cargo-generate
                  pkgs-unstable.cargo-watch
                  pkgs-unstable.rustup
                  pkgs-unstable.rust-analyzer

                  iconv
                ] ++ (lib.optionals stdenv.isDarwin

                  (with pkgs.darwin.apple_sdk.frameworks;
                  [
                    SystemConfiguration
                    CoreServices
                    CoreBluetooth
                  ]));

            };
        });
}

