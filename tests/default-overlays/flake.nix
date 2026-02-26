{
  description = "Flake exercising the default `overlays` schema";

  inputs = {};

  outputs = _: {
    overlays.default = _: _: {};
    overlays.other = _: _: {};
  };
}
