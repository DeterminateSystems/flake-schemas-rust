{
  description = "Flake exercising the built-in `nixosModules` schema";

  inputs = { };

  outputs = _: {
    nixosModules.file = ./module.nix;
    nixosModules.inline = _: { };
  };
}
