{
  description = "Kett's generic shell for Rust+iced";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs =
    {
      nixpkgs,
      ...
    }: let 
        system = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.${system};
        deps = with pkgs; [
            cargo
            cmake
            pkg-config

            clang
            libclang

            libGL
            glfw
            xorg.libX11
            xorg.libX11.dev
            xorg.libXcursor
            xorg.libXinerama
            xorg.libXrandr
            xorg.libXi
        ];
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        pname = manifest.name;
    in with pkgs; {
        packages.${system}.default = rustPlatform.buildRustPackage {
            name = manifest.name;
            version = manifest.version;
            cargoLock.lockFile = ./Cargo.lock;
            src = lib.cleanSource ./.;

            nativeBuildInputs = deps ++ [makeWrapper];
            buildInputs = deps;

            postFixup = ''
                wrapProgram $out/bin/${pname} \
                    --set LD_LIBRARY_PATH "${lib.makeLibraryPath deps}"
            '';
        };

        devShells.${system}.default = mkShell {
            buildInputs = deps;
            LD_LIBRARY_PATH = "${lib.makeLibraryPath deps}";
        };
    };
}

