{
  description = "Flake with a custom schema mimicking module-like schemas (e.g., `nixosModules` or `homeModules`)";

  inputs = { };

  outputs = _: {
    schemas.customModules = {
      version = 1;
      doc = ''
        The `customModules` flake output defines something the NixOS module system would consume.
      '';

      inventory = output: {
        children = builtins.mapAttrs (
          name: module: {
            what = "customModule";
          }
        ) output;
      };
    };

    customModules.file = ./module.nix;
    customModules.inline = _: {};
  };
}
