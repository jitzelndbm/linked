{
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "linked";
  version = "0.1.0";
  src = ./..;
  cargoLock.lockFile = ../Cargo.lock;
}
