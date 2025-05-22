{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  cargoSha256 = pkgs.lib.fakeSha256;

  pname = "crab-http";
  version = "0.1.5";

  src = pkgs.fetchFromGitHub {
    owner = "arjav0703";
    repo = "http-server";
    rev = "your-commit-hash-here";
    sha256 = pkgs.lib.fakeSha256;
  };


  meta = with pkgs.lib; {
    description = "A fast and lightweight (less than 2 MB) HTTP server written in Rust 🦀";
    homepage = "https://github.com/arjav0703/http-server";
    license = licenses.mit;
    maintainers = [];  # Optional: Add maintainers from Nixpkgs maintainers list
  };
}

