with import <nixpkgs> {};
pkgs.mkShell {
  buildInputs = [
    glib pkgconfig openssl cmake pango atk gdk-pixbuf gtk3 rustup
  ];
}
