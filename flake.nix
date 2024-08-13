{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        craneLib = (crane.mkLib nixpkgs.legacyPackages.${system});
        src = ./.;

        cargoArtifacts = craneLib.buildDepsOnly { inherit src; };
      in
      {
        defaultPackage = craneLib.buildPackage { inherit cargoArtifacts src; };
      }
    );
}
