{
  description = "Flake with a custom schema mimicking `packages`";

  inputs = { };

  outputs =
    _:
    let
      nameValuePair = name: value: { inherit name value; };
      genAttrs = names: f: builtins.listToAttrs (map (n: nameValuePair n (f n)) names);

      stubFor =
        system:
        {
          name,
          meta ? { },
        }:
        derivation {
          inherit name system;

          builder = "/bin/sh";
          args = [
            "-c"
            ''
              echo "out: $name/$system" >$out
              echo "doc: $name/$system" >$doc
            ''
          ];

          outputs = [
            "out"
            "doc"
          ];
        }
        // {
          inherit meta;
        };

      systems = [
        "aarch64-darwin"
        "x86_64-linux"
      ];

      forEachSystem = f: genAttrs systems (system: f (stubFor system));
    in
    {
      # Schemas like packages.${system}.${name}
      schemas.nestedBySystem = {
        version = 1;
        doc = ''
          The `nestedBySystem` output defines multiple per-system outputs, like a flake's `packages`.
        '';

        appendSystem = true;
        defaultAttrPath = [ "default" ];

        inventory = output: {
          children = builtins.mapAttrs (system: packagesBySystem: {
            forSystems = [ system ];
            children = builtins.mapAttrs (name: package: {
              what = "nestedBySystem";
              forSystems = [ system ];
              shortDescription = package.meta.description or "";
              derivationAttrPath = [ ];
            }) packagesBySystem;
          }) output;
        };
      };

      nestedBySystem = forEachSystem (stub: {
        default = stub { name = "system-default"; };

        description = stub {
          name = "system-description";
          meta.description = "a nestedBySystem derivation with a description";
        };
      });
    };
}
