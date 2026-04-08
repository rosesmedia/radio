{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils }:
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

        streamControllerDependencies = with pkgs; [
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];

        craneLib = crane.mkLib pkgs;
      in
      {
        devShells.default = craneLib.devShell {
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath(with pkgs; [
            libpq
            file
          ] ++ streamControllerDependencies)}";
          packages = with pkgs; [
            libpq
            file
            (python313.withPackages devDependencies)
            uv
            ffmpeg
            liquidsoap
            socat
            rust-analyzer
          ] ++ streamControllerDependencies;
        };
      }
    );
}
