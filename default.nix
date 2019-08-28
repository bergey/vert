{ stdenv, rustPlatform}:

rustPlatform.buildRustPackage rec {
  name = "vert-${version}";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "sha256:0jacm96l1gw9nxwavqi1x4669cg6lzy9hr18zjpwlcyb3qkw9z7f";

  meta = with stdenv.lib; {
    description = "A simple terminal pager";
    homepage = https://github.com/bergey/vert;
    license = licenses.bsd3;
    maintainers = [ maintainers.bergey ];
    platforms = platforms.all;
  };
}
