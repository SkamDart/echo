with import <nixpkgs> {};

let
  crust = (rustChannels.stable.rust.override { extensions = [ "rust-src" ]; });
in
stdenv.mkDerivation {
  name = "echo";
  buildInputs = [ crust rustracer ];
  RUST_SRC_PATH = "${crust}/lib/rustlib/src/rust/src";
}
