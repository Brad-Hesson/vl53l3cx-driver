{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    esp-idf = {
      url = "github:mirrexagon/nixpkgs-esp-dev";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = flakes:
    let
      system = "x86_64-linux";
      pkgs = import flakes.nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };
      fenix = flakes.fenix.packages.${system};
      esp-idf = flakes.esp-idf.packages.${system};
      llvm = pkgs.llvmPackages_17;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          fenix.latest.toolchain
          llvm.clang
          # esp-idf.esp-idf-esp32c3
        ];
        shellHook = ''
          export LIBCLANG_PATH="${llvm.libclang.lib}/lib";
        '';
      };
    };
}

