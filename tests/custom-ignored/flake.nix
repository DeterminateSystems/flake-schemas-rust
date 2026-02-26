{
  description = "Flake with ignored outputs while using custom schemas";

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
        "aarch64-linux"
        "x86_64-linux"
      ];

      forEachSystem = f: genAttrs systems (system: f (stubFor system));
    in
    {
      schemas.custom = {
        version = 1;
        doc = ''
          A custom schema not covering the `ignored` attribute of this flake
        '';

        inventory = output: {
          children = builtins.mapAttrs (name: module: {
            what = "custom";
          }) output;
        };
      };

      # Since there's no schema for these, there shouldn't be any outputs
      ignored = forEachSystem (stub: {
        default = stub { name = "ignored"; };
      });
    };
}
