{
  description = "Flake exercising the built-in `packages` schema";

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
      packages = forEachSystem (stub: {
        default = stub { name = "packages-default"; };
        description = stub {
          name = "packages-description";
          meta.description = "a package with a description";
        };
      });
    };
}
