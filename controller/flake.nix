{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system: let pkgs = import nixpkgs { inherit system; }; in {
      devShells.default = pkgs.mkShell {
        version = "0.1.0";
        buildInputs = with pkgs; [
          rye
        ];
      };
    });
}