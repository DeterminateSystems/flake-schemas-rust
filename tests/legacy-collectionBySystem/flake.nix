{
  description = "Flake with a custom schema emulating a builder with multiple interesting outputs";

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
            ''
          ];

          outputs = [
            "out"
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
      # A nested collection grouped by system
      #
      # (For example, imagine a system builder that takes a configuration and
      # has multiple derivations representing different VM image formats)
      schemas.collectionBySystem = {
        version = 1;
        doc = ''
          The `collectionBySystem` output defines, per system, named collections instead of individual packages.
        '';

        appendSystem = true;

        inventory =
          let
            childInventory = system: pkg: {
              what = "collectionBySystem";
              forSystems = [ system ];
              shortDescription = pkg.meta.description or "";
              derivation = pkg;
            };
          in
          output: {
            children = builtins.mapAttrs (system: collectionsBySystem: {
              forSystems = [ system ];
              children = builtins.mapAttrs (name: collection: {
                forSystems = [ system ];

                # This example assumes fixed outputs
                children.foo = childInventory system collection.foo;
                children.bar = childInventory system collection.bar;
              }) collectionsBySystem;
            }) output;
          };
      };

      collectionBySystem = forEachSystem (stub: {
        default = {
          foo = stub { name = "collection-default-foo"; };
          bar = stub { name = "collection-default-bar"; };
        };

        description = {
          foo = stub {
            name = "collection-description-foo";
            meta.description = "The `foo` output of the `description` collection";
          };
          bar = stub {
            name = "collection-description-bar";
            meta.description = "The `bar` output of the `description` collection";
          };
        };
      });
    };
}
