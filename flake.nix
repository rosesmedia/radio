{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem(system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        inherit (pkgs) lib;
        dependencies = ps: with ps; [
        ];
        devDependencies = ps: with ps; [
          python-lsp-server
        ] ++ (dependencies ps);
      in
      {
        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath(with pkgs; [
            libpq
          ])}";
          nativeBuildInputs = with pkgs; [
            libpq
            (python313.withPackages devDependencies)
            uv
          ];
        };
      }
    );
}
