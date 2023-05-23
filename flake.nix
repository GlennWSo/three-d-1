{
  description = "A devShell that can run three-d examples";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        deps = with pkgs; [
          openssl
          pkg-config
          rust-bin.stable.latest.default
          glibc
          mesa
        ];

        utils = with pkgs; [
          # checks video driver info
          pciutils 
          glxinfo
        ];
        
        libPath = with pkgs; lib.makeLibraryPath [
          xorg.libX11
          xorg.libXcursor
          xorg.libXxf86vm
          xorg.libXi
          xorg.libXrandr
          libGL
         ];    
      in
      with pkgs;
      {
        devShells.default = mkShell {
          name = "rust graphics"; 
          buildInputs = deps ++ utils;
          LD_LIBRARY_PATH=libPath;
          shellHook = ''
          echo Hello, Dev!
          '';
        };
      }
    );
}
