{ pkgs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [ "rustc" "cargo" "clippy" "rust-analyzer" ];
  };
  packages = with pkgs; [
    bacon
    cargo-seek
    cargo-nextest
    cargo-llvm-cov
    cargo-generate
  ];
  scripts.watcher = {
    exec = ''
        watchexec -c -e rs \
        "cargo clippy && cargo test && cargo run"
    '';
    packages = [ pkgs.watchexec ];
  };
  enterShell = ''
  echo "Crates ready to update";
  cargo update -n
  '';
  git-hooks.hooks = {
      clippy.enable = true;
      clippy.settings.denyWarnings = true;
  };
}
