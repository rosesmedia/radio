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
          django
          psycopg
        ];
        devDependencies = ps: with ps; [
          python-lsp-server
        ] ++ (dependencies ps);
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            (python313.withPackages devDependencies)
          ];
        };
      }
    );
}
