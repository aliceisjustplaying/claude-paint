{ fetchurl, stdenvNoCC }:

let
  sources = {
    aarch64-darwin = {
      target = "aarch64-apple-darwin";
      hash = "sha256-ENCeFGhSqN+Hls9kLKWIchB6fzkbdkfsjMJTzyia8Kk=";
    };
    x86_64-linux = {
      target = "x86_64-unknown-linux-musl";
      hash = "sha256-8vZVXe5QlFwqkKf7eTwghKAMxkWdCKwjBtONkFBkMhs=";
    };
  };
  source = sources.${stdenvNoCC.hostPlatform.system};
in
stdenvNoCC.mkDerivation rec {
  pname = "kache";
  version = "1.2.2";
  src = fetchurl {
    url = "https://github.com/kunobi-ninja/kache/releases/download/v${version}/kache-${source.target}.tar.gz";
    hash = source.hash;
  };
  sourceRoot = ".";
  dontBuild = true;
  dontFixup = true;
  installPhase = ''
    runHook preInstall
    install -Dm755 kache "$out/bin/kache"
    runHook postInstall
  '';
}
