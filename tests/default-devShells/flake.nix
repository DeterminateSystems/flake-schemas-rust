{
  description = "Flake exercising the built-in `devShells` schema";

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
        (derivation {
          inherit name system;

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
        })
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
      devShells = forEachSystem (stub: {
        default = stub { name = "devShells-default"; };
        description = stub {
          name = "devShells-description";
          meta.description = "a devShell with a description";
        };
      });
    };
}
