use pretty_assertions::assert_eq;

use crate::{Collection, Entry, InspectOutput};

// Helper macro to simplify writing maps because otherwise these tests will take even longer to write than they already do
macro_rules! kv_map {
    {} => { [].into() };
    {$key:tt: $value:expr} => { [($key.into(), $value.into())].into() };
    {$($key:tt: $value:expr,)+} => { [ $(($key.into(), $value.into()),)+ ].into() };
}

macro_rules! set {
    [] => { [].into() };
    [$head:tt $(, $tail:tt)* $(,)?] => { [$head.into() $(, $tail.into())* ].into() };
}

fn do_test(path: &str, expected: InspectOutput) {
    let actual = crate::inspect(path).unwrap();

    assert_eq!(actual, expected, "Inspecting flake at {path}");
}

#[test]
fn empty() {
    // Test: an empty flake shouldn't surprise us with unexpected output
    do_test(
        "./tests/empty",
        InspectOutput {
            version: 1,
            docs: kv_map! {},
            inventory: kv_map! {},
        },
    );
}

#[test]
fn custom_by_system() {
    // Test: Ensure that custom schemas can mimic the `formatter` output (that is, { output.${system} = derivation; } works)
    do_test(
        "./tests/custom-bySystem",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "bySystem": "The `bySystem` flake output defines a per-system output, like a flake's `formatter`.\n",
            },
            inventory: kv_map! {
                "bySystem": Collection {
                    children: kv_map! {
                        "aarch64-darwin": Entry {
                            what: "bySystem".into(),
                            short_description: Some("".into()),
                            for_systems: Some(set!["aarch64-darwin"]),
                            derivation: Some("/nix/store/0hbva7czr00sf4m97jlw4swagc8ccz87-simple.drv".into()),
                            outputs: Some(kv_map! {
                                "doc": "/nix/store/3jvc25bixqghypgda09hhg7sm2sdvi1k-simple-doc",
                                "out": "/nix/store/9nnh702lr0klxsgsqff3p198ycxa2gbj-simple",
                            })
                        },
                        "x86_64-linux": Entry {
                            what: "bySystem".into(),
                            short_description: Some("".into()),
                            for_systems: Some(set!["x86_64-linux"]),
                            derivation: Some("/nix/store/pxlb58dhphzbzhk40gbasdlaxnrjxndr-simple.drv".into()),
                            outputs: Some(kv_map! {
                                "doc": "/nix/store/5yf9i39g2kk2xs7kzwq3v3pcar92r47z-simple-doc",
                                "out": "/nix/store/bc36n9f0as5lxkq35xyvc2ch40j7y3ya-simple",
                            }),
                        },
                    },
                },
            },
        },
    );
}

#[test]
fn custom_collection_by_system() {
    do_test(
        "./tests/custom-collectionBySystem",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "collectionBySystem": "The `collectionBySystem` output defines, per system, named collections instead of individual packages.\n",
            },
            inventory: kv_map! {
                "collectionBySystem": Collection {
                    children: kv_map! {
                        "aarch64-darwin": Collection {
                            children: kv_map! {
                                "default": Collection {
                                    children: kv_map! {
                                        "bar": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("".into()),
                                            for_systems: Some(set!["aarch64-darwin"]),
                                            derivation: Some("/nix/store/d39z97pvwv04lvfrrwqbc3g2sq49gdg1-collection-default-bar.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/6apgj7nnkmjx41jacp9is3r7nnmb2ypg-collection-default-bar",
                                            }),
                                        },
                                        "foo": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("".into()),
                                            for_systems: Some(set!["aarch64-darwin"]),
                                            derivation: Some("/nix/store/hxg4vk2knpldpyb1nc7xjw339jsgizc9-collection-default-foo.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/ihyp5ia9gsfi297xdyg81xy2rchcjg4d-collection-default-foo",
                                            }),
                                        },
                                    },
                                },

                                "description": Collection {
                                    children: kv_map! {
                                        "bar": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("The `bar` output of the `description` collection".into()),
                                            for_systems: Some(set!["aarch64-darwin"]),
                                            derivation: Some("/nix/store/v3ychybm80s9lkjkk3grwy8ygwr1p7zi-collection-description-bar.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/w8gcd0qx5rxlghcllfq96524vs3578d5-collection-description-bar",
                                            }),
                                        },
                                        "foo": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("The `foo` output of the `description` collection".into()),
                                            for_systems: Some(set!["aarch64-darwin"]),
                                            derivation: Some("/nix/store/cz32kxw19d9hn8kzy42xqsj0r57napvf-collection-description-foo.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/91wvh13bv2q2i01nm91yvl0ha5wdarps-collection-description-foo",
                                            }),
                                        },
                                    },
                                },
                            },
                        },

                        "x86_64-linux": Collection {
                            children: kv_map! {
                                "default": Collection {
                                    children: kv_map! {
                                        "bar": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("".into()),
                                            for_systems: Some(set!["x86_64-linux"]),
                                            derivation: Some("/nix/store/6ivswh0y9dnw7chyzqbykvdwhfw1r4xn-collection-default-bar.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/7ls0qg7d76ixhxa4a5mrkqqk8qk32pc0-collection-default-bar",
                                            }),
                                        },
                                        "foo": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("".into()),
                                            for_systems: Some(set!["x86_64-linux"]),
                                            derivation: Some("/nix/store/gy5pmk2br6w2c69p8j08ndrpy1l2yj55-collection-default-foo.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/hawszf0lh6wz4c3xd64s23zajyfjj8ww-collection-default-foo",
                                            }),
                                        },
                                    },
                                },

                                "description": Collection {
                                    children: kv_map! {
                                        "bar": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("The `bar` output of the `description` collection".into()),
                                            for_systems: Some(set!["x86_64-linux"]),
                                            derivation: Some("/nix/store/nm7nd8asviddd4cn6qylh4id7y8x7z4k-collection-description-bar.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/qhbbm9k4zlfapjfj1w7pkfxz9n183s4k-collection-description-bar",
                                            }),
                                        },
                                        "foo": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("The `foo` output of the `description` collection".into()),
                                            for_systems: Some(set!["x86_64-linux"]),
                                            derivation: Some("/nix/store/2vnzwljjz9nikl372ksjb2x4zymava62-collection-description-foo.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/z26vxvqbxv0k2hszcx1hhxg0pj63617g-collection-description-foo",
                                            }),
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
        },
    );
}

#[test]
fn custom_nested_by_system() {
    do_test(
        "./tests/custom-nestedBySystem",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "nestedBySystem": "The `nestedBySystem` output defines multiple per-system outputs, like a flake's `packages`.\n",
            },
            inventory: kv_map! {
                "nestedBySystem": Collection {
                    children: kv_map! {
                        "aarch64-darwin": Collection {
                            children: kv_map! {
                                "default": Entry {
                                    what: "nestedBySystem".into(),
                                    short_description: Some("".into()),
                                    for_systems: Some(set!["aarch64-darwin"]),
                                    derivation: Some("/nix/store/jgqjdxgjh762dmczvj6x54npk52vvdh7-system-default.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/4cvwa8vipkad2xhzfphg1p83jamb0l1k-system-default-doc",
                                        "out": "/nix/store/70zxqk18jrzdw0s5x0q49m1hnavxxkzq-system-default",
                                    }),
                                },
                                "description": Entry {
                                    what: "nestedBySystem".into(),
                                    short_description: Some("a nestedBySystem derivation with a description".into()),
                                    for_systems: Some(set!["aarch64-darwin"]),
                                    derivation: Some("/nix/store/3i1fydlawif9vvkrkzlamfbg5k49s0hw-system-description.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/c06zi9iqnxa8xfpzj2a06r4ay92fjz29-system-description-doc",
                                        "out": "/nix/store/d3wjmgr703b0ap5111nrikspgmirgy6l-system-description",
                                    }),
                                },
                            },
                        },

                        "x86_64-linux": Collection {
                            children: kv_map! {
                                "default": Entry {
                                    what: "nestedBySystem".into(),
                                    short_description: Some("".into()),
                                    for_systems: Some(set!["x86_64-linux"]),
                                    derivation: Some("/nix/store/jpnmfd0057k8dz0viwpyc621v2fkcp21-system-default.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/512pz3dg40phcyw0dj5skd75y15235az-system-default-doc",
                                        "out": "/nix/store/0rr5dh8rqlpvl5bb1sb6b3mafi7v7g43-system-default",
                                    }),
                                },
                                "description": Entry {
                                    what: "nestedBySystem".into(),
                                    short_description: Some("a nestedBySystem derivation with a description".into()),
                                    for_systems: Some(set!["x86_64-linux"]),
                                    derivation: Some("/nix/store/ggy02x051ragig56864ahkljsvzhx6dd-system-description.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/bydpjqzw93579lzfd6v1y23qam42bksh-system-description-doc",
                                        "out": "/nix/store/n4yy7gyqbpc0v5vk06lkkiagj4bx20fm-system-description",
                                    }),
                                },
                            },
                        },
                    },
                }
            },
        },
    );
}

#[test]
fn custom_ignored() {
    // Test: don't inventory outputs, even if there's a custom schema that covers something else
    do_test(
        "./tests/custom-ignored",
        InspectOutput {
            version: 1,
            docs: kv_map! {},
            inventory: kv_map! {},
        },
    );
}

#[test]
fn custom_modules() {
    // Test: ensure that custom schemas can mimic modules
    do_test(
        "./tests/custom-modules",
        InspectOutput {
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
        },
    );
}

#[test]
fn default_devshells() {
    do_test(
        "./tests/default-devShells",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "devShells": "The `devShells` flake output contains derivations that provide a development environment for `nix develop`.\n",
            },
            inventory: kv_map! {
                "devShells": Collection {
                    children: kv_map! {
                        "aarch64-darwin": Collection {
                            children: kv_map! {
                                "default": Entry {
                                    what: "development environment".into(),
                                    short_description: Some("".into()),
                                    for_systems: Some(set!["aarch64-darwin"]),
                                    derivation: Some("/nix/store/h7ccb1z1dhy19yih7h65awppmy68liws-devShells-default.drv".into()),
                                    outputs: Some(kv_map! {
                                        "out": "/nix/store/26q7y2507wrgk5jhjlhc7fa56r9d20ja-devShells-default",
                                    }),
                                },
                                "description": Entry {
                                    what: "development environment".into(),
                                    short_description: Some("a devShell with a description".into()),
                                    for_systems: Some(set!["aarch64-darwin"]),
                                    derivation: Some("/nix/store/pxkgrfggfngfj4imw8ndgsfaq6zkvh3n-devShells-description.drv".into()),
                                    outputs: Some(kv_map! {
                                        "out": "/nix/store/l5jcs66ni6ghiz2lvn9jc32nlydzrdmb-devShells-description",
                                    }),
                                },
                            },
                        },

                        "x86_64-linux": Collection {
                            children: kv_map! {
                                "default": Entry {
                                    what: "development environment".into(),
                                    short_description: Some("".into()),
                                    for_systems: Some(set!["x86_64-linux"]),
                                    derivation: Some("/nix/store/kaw66walr42pw81vi2lvmvrh60yx5x7a-devShells-default.drv".into()),
                                    outputs: Some(kv_map! {
                                        "out": "/nix/store/d5g0gz86sjax3xk8sxcrzhxxf9akdmiy-devShells-default",
                                    }),
                                },
                                "description": Entry {
                                    what: "development environment".into(),
                                    short_description: Some("a devShell with a description".into()),
                                    for_systems: Some(set!["x86_64-linux"]),
                                    derivation: Some("/nix/store/cxrd9yhac4xybx0vb2hqmv9c3lkanfa6-devShells-description.drv".into()),
                                    outputs: Some(kv_map! {
                                        "out": "/nix/store/x2iyqddcsv1zr5jg3j768l0pmvp605d5-devShells-description",
                                    }),
                                },
                            },
                        },

                    },
                },
            },
        },
    );
}

#[test]
fn default_formatter() {
    do_test(
        "./tests/default-formatter",
        InspectOutput {
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
        },
    );
}

#[test]
fn default_ignored() {
    // Test: if a flake's outputs don't have a schema, we shouldn't see them in the inventory
    do_test(
        "./tests/default-ignored",
        InspectOutput {
            version: 1,
            docs: kv_map! {},
            inventory: kv_map! {},
        },
    );
}

#[test]
fn default_home_configurations() {
    do_test(
        "./tests/default-homeConfigurations",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "homeConfigurations": "The `homeConfigurations` flake output defines [Home Manager configurations](https://github.com/nix-community/home-manager).\n"
            },
            inventory: kv_map! {
                "homeConfigurations": Collection {
                    children: kv_map! {
                        "laptop": Entry {
                            what: "Home Manager configuration".into(),
                            short_description: None,
                            for_systems: Some(set!["aarch64-darwin"]),
                            derivation: Some("/nix/store/8m3k7dxaa2ja3m37lvkmai3zkcyl8psp-laptop.drv".into()),
                            outputs: Some(kv_map! {
                                "out": "/nix/store/ck40cfcvn4lrqx2lkxg5sr3kgnvhivkx-laptop",
                            }),
                        },

                        "workstation": Entry {
                            what: "Home Manager configuration".into(),
                            short_description: None,
                            for_systems: Some(set!["x86_64-linux"]),
                            derivation: Some("/nix/store/hqzk8axwq1s5ivhmg6dzi92fvhhwvgmq-workstation.drv".into()),
                            outputs: Some(kv_map! {
                                "out": "/nix/store/q3m4zvdliindkjzrqiinak45jr4bk7ij-workstation"
                            }),
                        },
                    },
                },
            },
        },
    )
}

#[test]
fn default_legacy_packages() {
    do_test(
        "./tests/default-legacyPackages",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "legacyPackages": "The `legacyPackages` flake output is similar to `packages`, but it can be nested (i.e. contain attribute sets that contain more packages).\nSince enumerating the packages in nested attribute sets is inefficient, `legacyPackages` should be avoided in favor of `packages`.\n\nNote: the contents of `legacyPackages` are not shown in FlakeHub.\n",
            },
            inventory: kv_map! {
                "legacyPackages": Collection {
                    children: kv_map! {},
                },
            },
        },
    )
}

#[test]
fn default_nixos_configurations() {
    do_test(
        "./tests/default-nixosConfigurations",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "nixosConfigurations": "The `nixosConfigurations` flake output defines [NixOS system configurations](https://nixos.org/manual/nixos/stable/#ch-configuration).\n",
            },
            inventory: kv_map! {
                "nixosConfigurations": Collection {
                    children: kv_map! {
                        "server": Entry {
                            what: "NixOS configuration".into(),
                            short_description: None,
                            for_systems: Some(set!["aarch64-linux"]),
                            derivation: Some("/nix/store/jsk3cjnz8rnibzik13di2rzz7rr36aiw-nixos-system-server.drv".into()),
                            outputs: Some(kv_map! {
                                "out": "/nix/store/32pqrkdvaqlwk13n7qz1pbz7haql3icz-nixos-system-server"
                            }),
                        },

                        "workstation": Entry {
                            what: "NixOS configuration".into(),
                            short_description: None,
                            for_systems: Some(set!["x86_64-linux"]),
                            derivation: Some("/nix/store/iyx2kmms29yisl9y1j8xrmh9fqkq33kx-nixos-system-workstation.drv".into()),
                            outputs: Some(kv_map!{
                                "out": "/nix/store/5wh32nh6h9w10fpjyl4zs6ihl8qpa8r0-nixos-system-workstation"
                            }),
                        },
                    },
                },
            },
        },
    );
}

#[test]
fn default_nixos_modules() {
    do_test(
        "./tests/default-nixosModules",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "nixosModules": "The `nixosModules` flake output defines importable [NixOS modules](https://nixos.org/manual/nixos/stable/#sec-writing-modules).\n",
            },
            inventory: kv_map! {
                "nixosModules": Collection {
                    children: kv_map! {
                        "file": Entry {
                            what: "NixOS module".into(),
                            short_description: None,
                            for_systems: None,
                            derivation: None,
                            outputs: None,
                        },

                        "inline": Entry {
                            what: "NixOS module".into(),
                            short_description: None,
                            for_systems: None,
                            derivation: None,
                            outputs: None,
                        },
                    },
                },
            },
        },
    );
}

#[test]
fn default_overlays() {
    do_test(
        "./tests/default-overlays",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "overlays": r#"The `overlays` flake output defines ["overlays"](https://nixos.org/manual/nixpkgs/stable/#chap-overlays) that can be plugged into Nixpkgs.
Overlays add additional packages or modify or replace existing packages.
"#,
            },
            inventory: kv_map! {
                "overlays": Collection {
                    children: kv_map! {
                        "default": Entry {
                            what: "Nixpkgs overlay".into(),
                            short_description: None,
                            for_systems: None,
                            derivation: None,
                            outputs: None,
                        },

                        "other": Entry {
                            what: "Nixpkgs overlay".into(),
                            short_description: None,
                            for_systems: None,
                            derivation: None,
                            outputs: None,
                        },
                    },
                },
            },
        },
    );
}

#[test]
fn default_packages() {
    do_test(
        "./tests/default-packages",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "packages": "The `packages` flake output contains packages that can be added to a shell using `nix shell`.\n",
            },
            inventory: kv_map! {
                "packages": Collection {
                    children: kv_map! {
                        "aarch64-darwin": Collection {
                            children: kv_map! {
                                "default": Entry {
                                    what: "package".into(),
                                    short_description: Some("".into()),
                                    for_systems: Some(set!["aarch64-darwin"]),
                                    derivation: Some("/nix/store/3b9q5shmmmc3a7614m8mm28ywailr561-packages-default.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/3zpxy0985h8giwp78qrviz2p3hz58bln-packages-default-doc",
                                        "out": "/nix/store/f8d5pc9xr1sqq1kpqgdkfv36c1nyv66y-packages-default",
                                    }),
                                },

                                "description": Entry {
                                    what: "package".into(),
                                    short_description: Some("a package with a description".into()),
                                    for_systems: Some(set!["aarch64-darwin"]),
                                    derivation: Some("/nix/store/dry1mkjbjc5fy6g28836q6pykdv0g4gp-packages-description.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/qdr25n3jliaynhdhjylwkjaihd8jjrwx-packages-description-doc",
                                        "out": "/nix/store/v53j2i5fmm3kdsc7bdd149bv3pl2mfvh-packages-description",
                                    }),
                                },
                            },
                        },

                        "x86_64-linux": Collection {
                            children: kv_map! {
                                "default": Entry {
                                    what: "package".into(),
                                    short_description: Some("".into()),
                                    for_systems: Some(set!["x86_64-linux"]),
                                    derivation: Some("/nix/store/23wwnr2p47p6id9kza6h0a07lysg4g1c-packages-default.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/91gy3swd4kd7zikyzasrzdd54zh96485-packages-default-doc",
                                        "out": "/nix/store/cmczy7lmw1v4d6qg1rx7hcl5pmw079ja-packages-default",
                                    }),
                                },

                                "description": Entry {
                                    what: "package".into(),
                                    short_description: Some("a package with a description".into()),
                                    for_systems: Some(set!["x86_64-linux"]),
                                    derivation: Some("/nix/store/vfx86h8rdh5hngcmwmg5yiqq8phd3q4a-packages-description.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/m5n06xldcpjlf3snk9d54q1w0qmrb0q2-packages-description-doc",
                                        "out": "/nix/store/3a7v1ggngkg86hgnxm416ps3n9ky9rwn-packages-description",
                                    }),
                                },
                            },
                        },
                    },
                },
            },
        },
    );
}

#[test]
fn legacy_by_system() {
    // Test: Ensure that custom schemas can mimic the `formatter` output (that is, { output.${system} = derivation; } works)
    do_test(
        "./tests/legacy-bySystem",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "bySystem": "The `bySystem` flake output defines a per-system output, like a flake's `formatter`.\n",
            },
            inventory: kv_map! {
                "bySystem": Collection {
                    children: kv_map! {
                        "aarch64-darwin": Entry {
                            what: "bySystem".into(),
                            short_description: Some("".into()),
                            for_systems: Some(set!["aarch64-darwin"]),
                            derivation: Some("/nix/store/0hbva7czr00sf4m97jlw4swagc8ccz87-simple.drv".into()),
                            outputs: Some(kv_map! {
                                "doc": "/nix/store/3jvc25bixqghypgda09hhg7sm2sdvi1k-simple-doc",
                                "out": "/nix/store/9nnh702lr0klxsgsqff3p198ycxa2gbj-simple",
                            })
                        },
                        "x86_64-linux": Entry {
                            what: "bySystem".into(),
                            short_description: Some("".into()),
                            for_systems: Some(set!["x86_64-linux"]),
                            derivation: Some("/nix/store/pxlb58dhphzbzhk40gbasdlaxnrjxndr-simple.drv".into()),
                            outputs: Some(kv_map! {
                                "doc": "/nix/store/5yf9i39g2kk2xs7kzwq3v3pcar92r47z-simple-doc",
                                "out": "/nix/store/bc36n9f0as5lxkq35xyvc2ch40j7y3ya-simple",
                            }),
                        },
                    },
                },
            },
        },
    );
}

#[test]
fn legacy_collection_by_system() {
    do_test(
        "./tests/legacy-collectionBySystem",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "collectionBySystem": "The `collectionBySystem` output defines, per system, named collections instead of individual packages.\n",
            },
            inventory: kv_map! {
                "collectionBySystem": Collection {
                    children: kv_map! {
                        "aarch64-darwin": Collection {
                            children: kv_map! {
                                "default": Collection {
                                    children: kv_map! {
                                        "bar": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("".into()),
                                            for_systems: Some(set!["aarch64-darwin"]),
                                            derivation: Some("/nix/store/d39z97pvwv04lvfrrwqbc3g2sq49gdg1-collection-default-bar.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/6apgj7nnkmjx41jacp9is3r7nnmb2ypg-collection-default-bar",
                                            }),
                                        },
                                        "foo": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("".into()),
                                            for_systems: Some(set!["aarch64-darwin"]),
                                            derivation: Some("/nix/store/hxg4vk2knpldpyb1nc7xjw339jsgizc9-collection-default-foo.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/ihyp5ia9gsfi297xdyg81xy2rchcjg4d-collection-default-foo",
                                            }),
                                        },
                                    },
                                },

                                "description": Collection {
                                    children: kv_map! {
                                        "bar": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("The `bar` output of the `description` collection".into()),
                                            for_systems: Some(set!["aarch64-darwin"]),
                                            derivation: Some("/nix/store/v3ychybm80s9lkjkk3grwy8ygwr1p7zi-collection-description-bar.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/w8gcd0qx5rxlghcllfq96524vs3578d5-collection-description-bar",
                                            }),
                                        },
                                        "foo": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("The `foo` output of the `description` collection".into()),
                                            for_systems: Some(set!["aarch64-darwin"]),
                                            derivation: Some("/nix/store/cz32kxw19d9hn8kzy42xqsj0r57napvf-collection-description-foo.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/91wvh13bv2q2i01nm91yvl0ha5wdarps-collection-description-foo",
                                            }),
                                        },
                                    },
                                },
                            },
                        },

                        "x86_64-linux": Collection {
                            children: kv_map! {
                                "default": Collection {
                                    children: kv_map! {
                                        "bar": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("".into()),
                                            for_systems: Some(set!["x86_64-linux"]),
                                            derivation: Some("/nix/store/6ivswh0y9dnw7chyzqbykvdwhfw1r4xn-collection-default-bar.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/7ls0qg7d76ixhxa4a5mrkqqk8qk32pc0-collection-default-bar",
                                            }),
                                        },
                                        "foo": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("".into()),
                                            for_systems: Some(set!["x86_64-linux"]),
                                            derivation: Some("/nix/store/gy5pmk2br6w2c69p8j08ndrpy1l2yj55-collection-default-foo.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/hawszf0lh6wz4c3xd64s23zajyfjj8ww-collection-default-foo",
                                            }),
                                        },
                                    },
                                },

                                "description": Collection {
                                    children: kv_map! {
                                        "bar": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("The `bar` output of the `description` collection".into()),
                                            for_systems: Some(set!["x86_64-linux"]),
                                            derivation: Some("/nix/store/nm7nd8asviddd4cn6qylh4id7y8x7z4k-collection-description-bar.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/qhbbm9k4zlfapjfj1w7pkfxz9n183s4k-collection-description-bar",
                                            }),
                                        },
                                        "foo": Entry {
                                            what: "collectionBySystem".into(),
                                            short_description: Some("The `foo` output of the `description` collection".into()),
                                            for_systems: Some(set!["x86_64-linux"]),
                                            derivation: Some("/nix/store/2vnzwljjz9nikl372ksjb2x4zymava62-collection-description-foo.drv".into()),
                                            outputs: Some(kv_map! {
                                                "out": "/nix/store/z26vxvqbxv0k2hszcx1hhxg0pj63617g-collection-description-foo",
                                            }),
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
        },
    );
}

#[test]
fn legacy_nested_by_system() {
    do_test(
        "./tests/legacy-nestedBySystem",
        InspectOutput {
            version: 1,
            docs: kv_map! {
                "nestedBySystem": "The `nestedBySystem` output defines multiple per-system outputs, like a flake's `packages`.\n",
            },
            inventory: kv_map! {
                "nestedBySystem": Collection {
                    children: kv_map! {
                        "aarch64-darwin": Collection {
                            children: kv_map! {
                                "default": Entry {
                                    what: "nestedBySystem".into(),
                                    short_description: Some("".into()),
                                    for_systems: Some(set!["aarch64-darwin"]),
                                    derivation: Some("/nix/store/jgqjdxgjh762dmczvj6x54npk52vvdh7-system-default.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/4cvwa8vipkad2xhzfphg1p83jamb0l1k-system-default-doc",
                                        "out": "/nix/store/70zxqk18jrzdw0s5x0q49m1hnavxxkzq-system-default",
                                    }),
                                },
                                "description": Entry {
                                    what: "nestedBySystem".into(),
                                    short_description: Some("a nestedBySystem derivation with a description".into()),
                                    for_systems: Some(set!["aarch64-darwin"]),
                                    derivation: Some("/nix/store/3i1fydlawif9vvkrkzlamfbg5k49s0hw-system-description.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/c06zi9iqnxa8xfpzj2a06r4ay92fjz29-system-description-doc",
                                        "out": "/nix/store/d3wjmgr703b0ap5111nrikspgmirgy6l-system-description",
                                    }),
                                },
                            },
                        },

                        "x86_64-linux": Collection {
                            children: kv_map! {
                                "default": Entry {
                                    what: "nestedBySystem".into(),
                                    short_description: Some("".into()),
                                    for_systems: Some(set!["x86_64-linux"]),
                                    derivation: Some("/nix/store/jpnmfd0057k8dz0viwpyc621v2fkcp21-system-default.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/512pz3dg40phcyw0dj5skd75y15235az-system-default-doc",
                                        "out": "/nix/store/0rr5dh8rqlpvl5bb1sb6b3mafi7v7g43-system-default",
                                    }),
                                },
                                "description": Entry {
                                    what: "nestedBySystem".into(),
                                    short_description: Some("a nestedBySystem derivation with a description".into()),
                                    for_systems: Some(set!["x86_64-linux"]),
                                    derivation: Some("/nix/store/ggy02x051ragig56864ahkljsvzhx6dd-system-description.drv".into()),
                                    outputs: Some(kv_map! {
                                        "doc": "/nix/store/bydpjqzw93579lzfd6v1y23qam42bksh-system-description-doc",
                                        "out": "/nix/store/n4yy7gyqbpc0v5vk06lkkiagj4bx20fm-system-description",
                                    }),
                                },
                            },
                        },
                    },
                }
            },
        },
    );
}

// Things in flake-schemas but not tested here (the inspect flake doesn't inventory them):
// * homeModules
// * nixDarwinConfigurations
// * nixDarwinModules
