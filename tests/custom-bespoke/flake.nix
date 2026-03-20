{
  description = "Flake with less-structured bespoke outputs";

  inputs = {};

  outputs = _:
    let
      
    in
    {
      bespoke.foo = {};
      bespoke.bar = {};

      schemas.bespoke = {
        version = 1;
        doc = ''
          The `bespoke` flake output exposes a bespoke flake output, such as a library function or code meant to be printed as JSON.
        '';
        inventory = _:
          {
            children = {
              foo = {};
              bar = {
                shortDescription = "bar contains a description";
              };
            };
          };
      };
    };
}
