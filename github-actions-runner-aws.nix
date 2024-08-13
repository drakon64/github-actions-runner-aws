{
  rustPlatform
}:

rustPlatform.buildRustPackage {
  pname = "github-actions-runner-aws";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;
}
