use pretty_assertions::assert_eq;

use crate::{Collection, Entry, InspectOutput};

macro_rules! kv_map {
    {} => { [].into() };
    {$key:tt: $value:expr} => { [($key.into(), $value.into())].into() };
    {$($key:tt: $value:expr,)+} => { [ $(($key.into(), $value.into()),)+ ].into() };
}

macro_rules! set {
    [] => { [].into() };
    [$head:tt $(, $tail:tt)* $(,)?] => { [$head.into() $(, $tail.into())* ].into() };
}

fn do_test(actual: InspectOutput, expected: &str) {
    let actual = serde_json::to_string_pretty(&actual).unwrap();

    assert_eq!(actual, expected);
}

#[test]
fn empty() {
    let actual = InspectOutput::new();

    let expected = indoc::indoc!(
        r#"{
          "version": 1,
          "docs": {},
          "inventory": {}
        }"#
    );

    do_test(actual, expected);
}

#[test]
fn custom_modules() {
    let actual = InspectOutput {
        version: 1,
        docs: kv_map! {
            "customModules": "The `customModules` flake output defines something the NixOS module system would consume.\n",
        },
        inventory: kv_map! {
            "customModules": Collection {
                children: kv_map! {
                    "file": Entry {
                        what: "customModule".into(),
                        short_description: None,
                        for_systems: None,
                        derivation: None,
                        outputs: None,
                    },

                    "inline": Entry {
                        what: "customModule".into(),
                        short_description: None,
                        for_systems: None,
                        derivation: None,
                        outputs: None,
                    },
                },
            },
        },
    };

    let expected = indoc::indoc!(
        r#"{
          "version": 1,
          "docs": {
            "customModules": "The `customModules` flake output defines something the NixOS module system would consume.\n"
          },
          "inventory": {
            "customModules": {
              "children": {
                "file": {
                  "what": "customModule",
                  "shortDescription": null,
                  "outputs": null,
                  "forSystems": null,
                  "derivation": null
                },
                "inline": {
                  "what": "customModule",
                  "shortDescription": null,
                  "outputs": null,
                  "forSystems": null,
                  "derivation": null
                }
              }
            }
          }
        }"#
    );

    do_test(actual, expected);
}

#[test]
fn default_formatter() {
    let actual = InspectOutput {
        version: 1,
        docs: kv_map! {
            "formatter": "The `formatter` output specifies the package to use to format the project.\n",
        },
        inventory: kv_map! {
            "formatter": Collection {
                children: kv_map! {
                    "aarch64-darwin": Entry {
                        what: "formatter".into(),
                        short_description: Some("".into()),
                        for_systems: Some(set!["aarch64-darwin"]),
                        derivation: Some("/nix/store/1qrjgbhp9m8qmaik22yqivncgidzrzsw-formatter.drv".into()),
                        outputs: Some(kv_map! {
                            "out": "/nix/store/hy13k3nf6xksf71j1frwx1rmqc6zzbz8-formatter",
                        }),
                    },

                    "x86_64-linux": Entry {
                        what: "formatter".into(),
                        short_description: Some("".into()),
                        for_systems: Some(set!["x86_64-linux"]),
                        derivation: Some("/nix/store/lr0j2n0a8xxzfwkabid29hj1fw73f3h3-formatter.drv".into()),
                        outputs: Some(kv_map! {
                            "out": "/nix/store/5ylvgzxb24cs77awvn4rdh4m573i929d-formatter",
                        }),
                    },
                }
            },
        },
    };

    let expected = indoc::indoc!(
        r#"{
          "version": 1,
          "docs": {
            "formatter": "The `formatter` output specifies the package to use to format the project.\n"
          },
          "inventory": {
            "formatter": {
              "children": {
                "aarch64-darwin": {
                  "what": "formatter",
                  "shortDescription": "",
                  "outputs": {
                    "out": "/nix/store/hy13k3nf6xksf71j1frwx1rmqc6zzbz8-formatter"
                  },
                  "forSystems": [
                    "aarch64-darwin"
                  ],
                  "derivation": "/nix/store/1qrjgbhp9m8qmaik22yqivncgidzrzsw-formatter.drv"
                },
                "x86_64-linux": {
                  "what": "formatter",
                  "shortDescription": "",
                  "outputs": {
                    "out": "/nix/store/5ylvgzxb24cs77awvn4rdh4m573i929d-formatter"
                  },
                  "forSystems": [
                    "x86_64-linux"
                  ],
                  "derivation": "/nix/store/lr0j2n0a8xxzfwkabid29hj1fw73f3h3-formatter.drv"
                }
              }
            }
          }
        }"#
    );

    do_test(actual, expected);
}
