{
  description = "A systemd unit watcher.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, naersk, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk' = pkgs.callPackage naersk { };
      in
      {
        packages.default = naersk'.buildPackage {
          pname = "uwatch";
          version = "0.1.0";
          src = ./.;
          cargoLock = ./Cargo.lock;
          release = true;
          meta = {
            description = "A systemd unit watcher.";
            homepage = "https://github.com/e-tho/uwatch";
            license = pkgs.lib.licenses.gpl3;
            maintainers = [
              {
                github = "e-tho";
              }
            ];
            mainProgram = "uwatch";
          };
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            pkg-config
          ];
        };
      });
}
