{
  description = "Gestor de ventas terramoda backend";
  inputs.nixpkgs.url = "github:NixOs/nixpkgs/nixos-25.05";
  outputs = {self, nixpkgs}:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in
  {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        bashInteractive
        rust-analyzer
        rustfmt
        rustc
        cargo
        openssl
        pkgconf
        sqlx-cli
      ];
    };
  };
}
