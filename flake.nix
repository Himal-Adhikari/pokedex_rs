{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, naersk }: let

    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    naerskLib = pkgs.callPackage naersk {};

    buildInputs = with pkgs; [
      openssl
      expat
      fontconfig
      freetype
      freetype.dev
      libGL
      pkg-config
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
      wayland
      libxkbcommon
    ];

  in {
    packages."x86_64-linux".default = naerskLib.buildPackage {
      src = ./.;
      inherit buildInputs;
      nativeBuildInputs = [ pkgs.pkg-config ];
    };

    devShells."x86_64-linux".default = pkgs.mkShell {
      inherit buildInputs;
      packages = with pkgs; [
        cargo
        rustc
        clippy
        rustfmt
        rust-analyzer
      ];
      nativeBuildInputs = [ pkgs.pkg-config ];

      LD_LIBRARY_PATH =
        builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;

      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  };
}
