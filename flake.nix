{
  description = "claude-paint test toolchain";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/8edc0c72e3a38faf5434e40d1f19431125fe4b30";

  outputs = { nixpkgs, ... }: {
    devShells = nixpkgs.lib.genAttrs [ "aarch64-darwin" "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        default = pkgs.mkShell {
          packages = [ pkgs.cargo pkgs.rustc pkgs.python3 pkgs.uv pkgs.perl pkgs.util-linux ];
          UV_PYTHON_DOWNLOADS = "never";
        };
      });
  };
}
