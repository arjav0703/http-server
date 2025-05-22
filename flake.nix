{
  description = "A fast and lightweight (less than 2 MB) HTTP server written in Rust 🦀";

  inputs = {
    # Use the latest stable version of Rust
    rust-overlay.url = "github:rust-lang/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import rust-overlay.nixpkgs {
        system = system;
      };
      rust = pkgs.rust-bin.stable;
    in {
      packages.http-server = pkgs.buildRustPackage {
        pname = "crab-http";
        version = "0.1.5"; # Update this to your app's version

        src = ./.; # Assuming your source code is in the current directory

        # Specify any dependencies your app needs
        cargoSha256 = "sha256-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"; # Replace with the actual sha256

      };

      # Optionally, you can define a default package
      defaultPackage = self.packages.${system}.crab-http;

      # Optionally, you can define a development shell
      devShell = pkgs.mkShell {
        buildInputs = [
          rust
        ];
      };
    });
}

