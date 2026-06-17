{
  description = "Unix-style CLI tools for Kagi search, maps, and URL summarization";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # crane splits the third-party dependency compile into its own cached
    # derivation (cargoArtifacts), so a change to kagi's own source recompiles only
    # kagi instead of all ~246 deps. crane is a pure lib (mkLib pkgs) with no
    # nixpkgs input to follow.
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      ...
    }:
    let
      supportedSystems = [
        "aarch64-darwin"
        "x86_64-linux"
      ];

      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      # crane's two stages share one argument set. cargoArtifacts compiles the
      # dependency tree once (keyed on Cargo.toml + Cargo.lock, with kagi's own
      # crate stubbed); the binary cache then serves it so a change to kagi's source
      # recompiles only kagi.
      craneFor =
        pkgs:
        let
          craneLib = crane.mkLib pkgs;
          commonArgs = {
            pname = "kagi";
            version = "0.4.0";
            # cleanSource, not crane's cleanCargoSource: postInstall installs the
            # skills/*/SKILL.md files from the build source, which a cargo-only
            # filter would drop.
            src = pkgs.lib.cleanSource ./.;
            strictDeps = true;
            nativeBuildInputs = [
              pkgs.cmake
              pkgs.git
              pkgs.pkg-config
              pkgs.rustPlatform.bindgenHook
            ];
          };
        in
        {
          inherit craneLib commonArgs;
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        };

      packageFor =
        pkgs:
        let
          c = craneFor pkgs;
        in
        c.craneLib.buildPackage (
          c.commonArgs
          // {
            inherit (c) cargoArtifacts;
            # The suite runs as its own check (checksFor) and in CI via `task test`;
            # the installable package only builds the binaries.
            doCheck = false;
            postInstall = ''
              install -Dm644 skills/search/SKILL.md "$out/share/kagi/skills/kagi-search/SKILL.md"
              install -Dm644 skills/maps/SKILL.md "$out/share/kagi/skills/kagi-maps/SKILL.md"
              install -Dm644 skills/summarize/SKILL.md "$out/share/kagi/skills/kagi-summarize/SKILL.md"
            '';
          }
        );

      # `cargo test`, reusing the cached dependency artifacts so it does not
      # recompile the dependency tree. Keeps `nix flake check` running the suite the
      # way the old `doCheck = true` package did.
      checksFor =
        pkgs:
        let
          c = craneFor pkgs;
        in
        c.craneLib.cargoTest (c.commonArgs // { inherit (c) cargoArtifacts; });

      devShellFor =
        pkgs:
        pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.cargo-deny
            pkgs.cargo-machete
            pkgs.cmake
            pkgs.git
            pkgs.go-task
            pkgs.nodejs
            pkgs.pkg-config
            pkgs.rustc
            pkgs.rustfmt
            pkgs.zig
            pkgs.clippy
          ];

          inputsFrom = [ (packageFor pkgs) ];
        };
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          kagi = packageFor pkgs;
        in
        {
          default = kagi;
          kagi = kagi;
        }
      );

      checks = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = checksFor pkgs;
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = devShellFor pkgs;
        }
      );

      homeManagerModules.default =
        {
          config,
          lib,
          pkgs,
          ...
        }:
        let
          cfg = config.programs.kagi;
          package = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
          skillNames = [
            "kagi-search"
            "kagi-maps"
            "kagi-summarize"
          ];

          mkSkillFiles =
            root:
            builtins.listToAttrs (
              map (name: {
                name = "${root}/${name}";
                value.source = "${cfg.package}/share/kagi/skills/${name}";
              }) skillNames
            );
        in
        {
          options.programs.kagi = {
            enable = lib.mkEnableOption "Kagi search, maps, and summarization CLIs and skills";

            package = lib.mkOption {
              type = lib.types.package;
              default = package;
              description = "Kagi package to install.";
            };
          };

          config = lib.mkIf cfg.enable {
            home.packages = [ cfg.package ];

            home.file =
              mkSkillFiles ".agents/skills"
              // mkSkillFiles ".claude/skills"
              // mkSkillFiles ".gemini/antigravity-cli/skills";
          };
        };
    };
}
