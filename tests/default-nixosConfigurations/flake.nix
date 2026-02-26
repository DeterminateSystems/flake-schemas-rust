{
  description = "Flake exercising the built-in `nixosConfigurations` schema";

  inputs = { };

  outputs =
    _:
    let
      stubFor =
        system:
        { name }:
        # This is just cheating, but we really don't want to pull in the NixOS module system just to read in some JSON
        {
          pkgs.stdenv.system = system;
          config.system.build.toplevel = derivation {
            name = "nixos-system-${name}";
            inherit system;

            builder = "/bin/sh";
            args = [
              "-c"
              ''
                echo "out: $name/$system" >$out
              ''
            ];

            outputs = [
              "out"
            ];
          };
        };
    in
    {
      nixosConfigurations.server = stubFor "aarch64-linux" { name = "server"; };
      nixosConfigurations.workstation = stubFor "x86_64-linux" { name = "workstation"; };
    };
}
