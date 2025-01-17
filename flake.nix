{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system: let pkgs = import nixpkgs { inherit system; }; in {
      devShells.default = pkgs.mkShell {
        version = "0.1.0";
        buildInputs = with pkgs; [
          # For the backend
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
          ])

          pkg-config
          libudev-zero
          openssl # For reqwest

          # For the frontend
          pnpm
          nodejs

          # Miscellanous
          socat

          # For the music visualization
          cava
        ];

        shellHook = let additionalLibraryPath = pkgs.lib.makeLibraryPath [
          pkgs.libudev-zero
        ]; in ''
        LD_LIBRARY_PATH=${additionalLibraryPath}:$LD_LIBRARY_PATH
        '';
      };
    });
}