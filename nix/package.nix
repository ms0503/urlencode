{
  lib,
  mold,
  myLib,
  rustPlatform,
}:
let
  inherit (myLib) filters;
  inherit (myLib.build) cleanSourcePipe;
  cargoToml = ../Cargo.toml |> builtins.readFile |> builtins.fromTOML;
in
rustPlatform.buildRustPackage {
  inherit (cargoToml.package) version;
  RUSTFLAGS = "-Clink-arg=-fuse-ld=mold";
  cargoLock.lockFile = ../Cargo.lock;
  meta = {
    description = "A tool for converting between plain text and URL-encoded text";
    license = lib.licenses.mit;
    mainProgram = "urlencode";
    sourceProvenance = with lib.sourceTypes; [
      fromSource
    ];
  };
  nativeBuildInputs = [
    mold
  ];
  pname = cargoToml.package.name;
  src = cleanSourcePipe ../. [
    filters.isNotNixDirectory
    filters.isNotNixFiles
  ];
}
