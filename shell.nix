{}:
let
  rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/9651f0beee6e7a9783cc02eac722854851c65ae7.tar.gz"));
  pkgs = (import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/22.11.tar.gz")) {
    overlays = [ rust-overlay ];
  });
  secrets = if builtins.pathExists ./secrets.nix then import ./secrets.nix else { ICE_SERVERS_URL="create secrets.nix file";};
in
pkgs.mkShell ({
  buildInputs = with pkgs; [
    (pkgs.rust-bin.stable.latest.default.override {
      extensions = ["rust-src"];
    })
    pkg-config
    openssl
    git
  ];
  RUST_BACKTRACE="full";
} // secrets)
