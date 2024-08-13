let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixpkgs-unstable";
  pkgs = import nixpkgs { };
in
{
  github-actions-runner-aws = pkgs.callPackage ./github-actions-runner-aws.nix { };
}
