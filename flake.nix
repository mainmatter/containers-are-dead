{
  description = "Flake providing a development shell for the workshop";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
        url = "github:oxalica/rust-overlay";
        inputs = {
            nixpkgs.follows = "nixpkgs";
        };
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = with pkgs; rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = with pkgs; mkShell rec {
          name = "k23-dev";
          buildInputs = [
            # compilers
            rustToolchain

            # inspecting and running wasm
            wasm-tools
            wasmtime
            binaryen
            wizer

            fermyon-spin

            # devtools
            mdbook
            jujutsu
            typos
            dprint
          ];
        };
      }
    );
}
