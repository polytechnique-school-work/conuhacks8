{
  description = "A template for typescript development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, ... }@inputs: inputs.utils.lib.eachSystem [
    "x86_64-linux"
  ] (system: let
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
  in {
    devShells.default = pkgs.mkShell rec {
      name = "LOG2990";

      packages = with pkgs; [
        nodejs
        # You can set the major version of Node.js to a specific one instead
        # of the default version
        # pkgs.nodejs-19_x
        # You can choose pnpm, yarn, or none (npm).
        nodePackages.npm
        # pkgs.yarn

        nodePackages.typescript
        nodePackages.typescript-language-server
      ];
    };
  });
}