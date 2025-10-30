pkgs:
pkgs.stdenv.mkDerivation {
  name = "shell";
  nativeBuildInputs = with pkgs; [
    cargo
    alejandra
    rustc
  ];
  shellHook = ''
    alejandra .
    alejandra ./*
    cargo run
  '';
}
