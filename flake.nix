{
  description = "Unix-style CLI tools for Kagi search, maps, and URL summarization";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      supportedSystems = [
        "aarch64-darwin"
        "x86_64-linux"
      ];

      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      packageFor =
        pkgs:
        pkgs.rustPlatform.buildRustPackage {
          pname = "kagi";
          version = "0.3.0";

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = [
            pkgs.cmake
            pkgs.git
            pkgs.pkg-config
            pkgs.rustPlatform.bindgenHook
          ];

          doCheck = true;

          postInstall = ''
            install -Dm644 skills/search/SKILL.md "$out/share/kagi/skills/kagi-search/SKILL.md"
            install -Dm644 skills/maps/SKILL.md "$out/share/kagi/skills/kagi-maps/SKILL.md"
            install -Dm644 skills/summarize/SKILL.md "$out/share/kagi/skills/kagi-summarize/SKILL.md"
          '';
        };

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
          default = packageFor pkgs;
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
