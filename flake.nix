{
  description = "claude-paint test toolchain";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/8edc0c72e3a38faf5434e40d1f19431125fe4b30";
  # glibc 2.39: the stored linux-x86_64 hashes (r19 surface, engine-2 replay) come from its libm
  inputs.nixpkgs-glibc.url = "github:NixOS/nixpkgs/nixos-24.05";

  outputs = { nixpkgs, nixpkgs-glibc, ... }: {
    devShells = nixpkgs.lib.genAttrs [ "aarch64-darwin" "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
        cc = (import nixpkgs-glibc { inherit system; }).stdenv.cc;
      in {
        default = pkgs.mkShell ({
          packages = [ pkgs.cargo pkgs.rustc pkgs.python3 pkgs.uv pkgs.perl pkgs.nodejs_24 pkgs.ffmpeg ] ++ nixpkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [ pkgs.procps ];
          UV_PYTHON_DOWNLOADS = "never";
        } // nixpkgs.lib.optionalAttrs (system == "x86_64-linux") {
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${cc}/bin/cc";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib ];
        });
      });
  };
}
