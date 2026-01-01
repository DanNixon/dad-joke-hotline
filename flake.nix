{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.cargo;
          rustc = pkgs.rustc;
        };

        lintingRustFlags = "-D unused-crate-dependencies";
      in {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            # Rust toolchain
            cargo
            rustc

            # Code analysis tools
            clippy
            rust-analyzer

            # Code formatting tools
            treefmt
            alejandra
            mdl
            rustfmt

            # Rust dependency linting
            cargo-deny

            # Container image management tool
            skopeo
          ];

          RUSTFLAGS = lintingRustFlags;
        };

        packages = rec {
          default = rustPlatform.buildRustPackage {
            pname = "dad-joke-hotline";
            version = "0.1.0";

            src = ./.;

            cargoLock.lockFile = ./Cargo.lock;
          };

          container-image = pkgs.dockerTools.buildImage {
            name = "dad-joke-hotline";
            tag = "latest";
            created = "now";

            copyToRoot = pkgs.buildEnv {
              name = "image-root";
              paths = [pkgs.bashInteractive pkgs.coreutils];
              pathsToLink = ["/bin"];
            };

            config = {
              Entrypoint = ["${pkgs.tini}/bin/tini" "--" "${default}/bin/dad-joke-hotline"];
              ExposedPorts = {
                "9090/tcp" = {};
              };
              Env = [
                "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
                "WEBHOOK_ADDRESS=0.0.0.0:8000"
                "OBSERVABILITY_ADDRESS=0.0.0.0:9090"
              ];
            };
          };
        };
      }
    );
}
