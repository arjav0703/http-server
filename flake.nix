{
  description = "A Rust HTTP server application";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay"; # Use a specific release
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import rust-overlay {
        system = system;
      };
      rust = pkgs.rust-bin.stable;
    in {
      packages.http-server = pkgs.buildRustPackage {
        pname = "http-server";
        version = "0.1.0"; # Update this to your app's version

        src = ./.; # Assuming your source code is in the current directory

        # Specify any dependencies your app needs
        cargoSha256 = "sha256-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"; # Replace with the actual sha256

        # Optional: Specify build flags or features
        buildInputs = [ pkgs.openssl ]; # Example: if you need OpenSSL
      };

      defaultPackage = self.packages.${system}.http-server;

      # Correctly define the development shell
      devShell = pkgs.mkShell {
        buildInputs = [
          rust
          pkgs.openssl # Example: if you need OpenSSL
        ];
      };
    });
}


