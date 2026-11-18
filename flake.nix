{
  description = "Pulso — motion graphics and visual effects, a free alternative to After Effects, in pure Rust";

  inputs = {
    # Pulso needs edition 2024 and rustc ≥ 1.95, so the flake tracks unstable. Consumers on a
    # release channel can retarget it: `inputs.pulso.inputs.nixpkgs.follows = "nixpkgs";`
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      inherit (nixpkgs) lib;

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      # The AppStream metadata carries a release date. The flake source's own commit date is
      # reproducible, unlike "now"; a tarball or dirty tree without one falls back to the epoch.
      buildDate =
        let
          d = self.lastModifiedDate or "";
        in
        if builtins.stringLength d >= 8 then
          "${builtins.substring 0 4 d}-${builtins.substring 4 2 d}-${builtins.substring 6 2 d}"
        else
          "1970-01-01";

      packageArgs = {
        inherit buildDate;
      };

      forAllSystems = f: lib.genAttrs systems (system: f (import nixpkgs { inherit system; }) system);
    in
    {
      packages = forAllSystems (
        pkgs: _:
        let
          pulso = pkgs.callPackage ./nix/package.nix packageArgs;
        in
        {
          inherit pulso;
          default = pulso;
        }
      );

      # `nixpkgs.overlays = [ inputs.pulso.overlays.default ]` → `pkgs.pulso` everywhere.
      overlays.default = final: _prev: {
        pulso = final.callPackage ./nix/package.nix packageArgs;
      };

      apps = forAllSystems (
        _: system: {
          default = {
            type = "app";
            program = lib.getExe self.packages.${system}.pulso;
          };
        }
      );

      devShells = forAllSystems (
        pkgs: system:
        let
          pulso = self.packages.${system}.pulso;
        in
        {
          default = pkgs.mkShell ({
            # The package's own native inputs, so `cargo build` in the shell needs no extra setup.
            inputsFrom = [ pulso ];
            packages = with pkgs; [
              cargo
              rustc
              clippy
              rustfmt
              rust-analyzer
            ];
            # `cargo run` dlopens the Vulkan loader, libwayland and libxkbcommon like the packaged binary.
            LD_LIBRARY_PATH = lib.makeLibraryPath pulso.passthru.runtimeLibs;
            RUST_BACKTRACE = "1";
          });
        }
      );

      checks = forAllSystems (
        pkgs: system: {
          pulso = self.packages.${system}.pulso;

          # `nix fmt` formats the flake; this keeps it formatted.
          formatting =
            pkgs.runCommand "pulso-nix-formatting" { nativeBuildInputs = [ pkgs.nixfmt ]; }
              ''
                nixfmt --check ${./flake.nix} ${./nix}
                touch $out
              '';
        }
      );

      formatter = forAllSystems (pkgs: _: pkgs.nixfmt);
    };
}
