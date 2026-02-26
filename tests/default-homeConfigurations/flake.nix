{
  description = "Flake exercising the `homeConfigurations` schema";

  inputs = { };

  outputs =
    _:
    let
      stubFor =
        system:
        { name }:
        # flake-schemas really only cares about the "activationPackage" attr, so that's all we mock
        {
          activationPackage = derivation {
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
          };
        };
    in
    {
      homeConfigurations.workstation = stubFor "x86_64-linux" { name = "workstation"; };
      homeConfigurations.laptop = stubFor "aarch64-darwin" { name = "laptop"; };
    };
}
