{}:
let
  rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/b91d162355e88de89b379f3d6a459ade92704474.tar.gz"));
  pkgs = (import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/23.05.tar.gz")) {
    overlays = [ rust-overlay ];
  });
  secrets = if builtins.pathExists ./secrets.nix then import ./secrets.nix else { ICE_SERVERS_URL="create secrets.nix file";};
in
pkgs.mkShell ({
  buildInputs = with pkgs; [
    (pkgs.rust-bin.stable.latest.default.override {
      extensions = ["rust-src" "llvm-tools-preview"];
    })
    pkg-config
    openssl
    git
  ];
  RUST_BACKTRACE="full";
} // secrets)
