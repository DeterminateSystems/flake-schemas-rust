use crate::{Collection, Entry, InspectOutput, InventoryItem};

use std::ops::ControlFlow;

use pretty_assertions::assert_eq;

macro_rules! kv_map {
    {} => { [].into() };
    {$key:tt: $value:expr} => { [($key.into(), $value.into())].into() };
    {$($key:tt: $value:expr,)+} => { [ $(($key.into(), $value.into()),)+ ].into() };
}

macro_rules! set {
    [] => { [].into() };
    [$head:tt $(, $tail:tt)* $(,)?] => { [$head.into() $(, $tail.into())* ].into() };
}

#[test]
fn collection_for_each_entry() {
    let entry_a = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry A".into()),
        derivation: Some("/nix/store/entry-a.drv".into()),
        for_systems: Some(set!["aarch64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-a",
        }),
    };

    let entry_b = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry B".into()),
        derivation: Some("/nix/store/entry-b.drv".into()),
        for_systems: Some(set!["x86_64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-b",
        }),
    };

    let collection = Collection {
        children: kv_map! {
            "aarch64-linux": entry_a.clone(),
            "x86_64-linux": entry_b.clone(),
        },
    };

    let expected = vec![
        (vec!["aarch64-linux".into()], entry_a.clone()),
        (vec!["x86_64-linux".into()], entry_b.clone()),
    ];

    let mut actual = Vec::new();
    collection.for_each_entry(|path, entry| {
        actual.push((path.into(), entry.clone()));
    });

    assert_eq!(expected, actual);
}

#[test]
fn collection_for_each_item() {
    let entry_a = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry A".into()),
        derivation: Some("/nix/store/entry-a.drv".into()),
        for_systems: Some(set!["aarch64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-a",
        }),
    };

    let entry_b = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry B".into()),
        derivation: Some("/nix/store/entry-b.drv".into()),
        for_systems: Some(set!["x86_64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-b",
        }),
    };

    let collection = Collection {
        children: kv_map! {
            "aarch64-linux": entry_a.clone(),
            "x86_64-linux": entry_b.clone(),
        },
    };

    let expected = vec![
        (
            vec!["aarch64-linux".into()],
            InventoryItem::Entry(entry_a.clone()),
        ),
        (
            vec!["x86_64-linux".into()],
            InventoryItem::Entry(entry_b.clone()),
        ),
    ];

    let mut actual = Vec::new();
    let result = collection.for_each_item::<()>(|path, item| {
        actual.push((path.into(), item.clone()));
        ControlFlow::Continue(())
    });

    assert_eq!(expected, actual);
    assert_eq!(ControlFlow::Continue(()), result);
}

#[test]
fn collection_for_each_item_with_break() {
    let entry_a = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry A".into()),
        derivation: Some("/nix/store/entry-a.drv".into()),
        for_systems: Some(set!["aarch64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-a",
        }),
    };

    let entry_b = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry B".into()),
        derivation: Some("/nix/store/entry-b.drv".into()),
        for_systems: Some(set!["x86_64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-b",
        }),
    };

    let collection = Collection {
        children: kv_map! {
            "aarch64-linux": entry_a.clone(),
            "x86_64-linux": entry_b.clone(),
        },
    };

    // We visit at least one node in our call, and it's not the root of the collection
    let expected: Vec<(Vec<String>, InventoryItem)> = vec![(
        vec!["aarch64-linux".into()],
        InventoryItem::Entry(entry_a.clone()),
    )];

    let mut actual = Vec::new();
    let result = collection.for_each_item(|path, item| {
        actual.push((path.to_vec(), item.clone()));
        ControlFlow::Break(42)
    });

    assert_eq!(expected, actual);
    assert_eq!(ControlFlow::Break(42), result);
}

#[test]
fn inventory_entry_for_each_entry() {
    let entry = Entry {
        what: Some("a derivation".into()),
        short_description: Some("".into()),
        derivation: Some("/nix/store/derivation.drv".into()),
        for_systems: Some(set!["x86_64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/derivation",
        }),
    };

    let expected = vec![(Vec::new(), entry.clone())];

    let mut actual = Vec::new();

    let inventory = InventoryItem::Entry(entry.clone());
    inventory.for_each_entry(|path, entry| {
        actual.push((path.into(), entry.clone()));
    });

    assert_eq!(expected, actual);
}

#[test]
fn inventory_entry_for_each_item() {
    let entry = Entry {
        what: Some("a derivation".into()),
        short_description: Some("".into()),
        derivation: Some("/nix/store/derivation.drv".into()),
        for_systems: Some(set!["x86_64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/derivation",
        }),
    };

    let inventory = InventoryItem::Entry(entry.clone());

    let expected = vec![(Vec::new(), inventory.clone())];

    let mut actual = Vec::new();

    let result = inventory.for_each_item::<()>(|path, item| {
        actual.push((path.into(), item.clone()));
        ControlFlow::Continue(())
    });

    assert_eq!(expected, actual);
    assert_eq!(ControlFlow::Continue(()), result);
}

#[test]
fn inventory_collection_for_each_entry() {
    let entry_a = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry A".into()),
        derivation: Some("/nix/store/entry-a.drv".into()),
        for_systems: Some(set!["aarch64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-a",
        }),
    };

    let entry_b = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry B".into()),
        derivation: Some("/nix/store/entry-b.drv".into()),
        for_systems: Some(set!["x86_64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-b",
        }),
    };

    let collection = Collection {
        children: kv_map! {
            "aarch64-linux": entry_a.clone(),
            "x86_64-linux": entry_b.clone(),
        },
    };

    let expected = vec![
        (vec!["aarch64-linux".into()], entry_a.clone()),
        (vec!["x86_64-linux".into()], entry_b.clone()),
    ];

    let inventory = InventoryItem::Collection(collection.clone());

    let mut actual = Vec::new();
    inventory.for_each_entry(|path, entry| {
        actual.push((path.into(), entry.clone()));
    });

    assert_eq!(expected, actual);
}

#[test]
fn inventory_collection_for_each_item() {
    let entry_a = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry A".into()),
        derivation: Some("/nix/store/entry-a.drv".into()),
        for_systems: Some(set!["aarch64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-a",
        }),
    };

    let entry_b = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry B".into()),
        derivation: Some("/nix/store/entry-b.drv".into()),
        for_systems: Some(set!["x86_64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-b",
        }),
    };

    let collection = Collection {
        children: kv_map! {
            "aarch64-linux": entry_a.clone(),
            "x86_64-linux": entry_b.clone(),
        },
    };

    let expected = vec![
        (vec![], InventoryItem::from(collection.clone())),
        (
            vec!["aarch64-linux".into()],
            InventoryItem::from(entry_a.clone()),
        ),
        (
            vec!["x86_64-linux".into()],
            InventoryItem::from(entry_b.clone()),
        ),
    ];

    let inventory = InventoryItem::Collection(collection.clone());

    let mut actual = Vec::new();
    let result = inventory.for_each_item::<()>(|path, item| {
        actual.push((path.into(), item.clone()));
        ControlFlow::Continue(())
    });

    assert_eq!(expected, actual);
    assert_eq!(ControlFlow::Continue(()), result);
}

#[test]
fn inventory_collection_for_each_item_with_break() {
    let entry_a = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry A".into()),
        derivation: Some("/nix/store/entry-a.drv".into()),
        for_systems: Some(set!["aarch64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-a",
        }),
    };

    let entry_b = Entry {
        what: Some("a derivation".into()),
        short_description: Some("Entry B".into()),
        derivation: Some("/nix/store/entry-b.drv".into()),
        for_systems: Some(set!["x86_64-linux"]),
        outputs: Some(kv_map! {
            "out": "/nix/store/entry-b",
        }),
    };

    let collection = Collection {
        children: kv_map! {
            "aarch64-linux": entry_a.clone(),
            "x86_64-linux": entry_b.clone(),
        },
    };

    let expected = vec![(vec![], InventoryItem::from(collection.clone()))];

    let inventory = InventoryItem::Collection(collection.clone());

    let mut actual = Vec::new();
    let result = inventory.for_each_item(|path, item| {
        actual.push((path.into(), item.clone()));
        ControlFlow::Break(42)
    });

    assert_eq!(expected, actual);
    assert_eq!(ControlFlow::Break(42), result);
}

#[test]
fn flake_outputs_for_each_entry() {
    let entry_a = Entry {
        what: Some("a formatter".into()),
        short_description: None,
        for_systems: Some(set!["aarch64-linux"]),
        derivation: Some("/nix/store/entry-a.drv".into()),
        outputs: None,
    };

    let entry_b = Entry {
        what: Some("a formatter".into()),
        short_description: None,
        for_systems: Some(set!["x86_64-linux"]),
        derivation: Some("/nix/store/entry-b.drv".into()),
        outputs: None,
    };

    let entry_c = Entry {
        what: Some("a package".into()),
        short_description: None,
        for_systems: Some(set!["x86_64-linux"]),
        derivation: Some("/nix/store/entry-c.drv".into()),
        outputs: None,
    };

    let formatters = Collection {
        children: kv_map! {
            "aarch64-linux": entry_a.clone(),
            "x86_64-linux": entry_b.clone(),
        },
    };

    let outputs = InspectOutput {
        version: 1,
        docs: Default::default(),
        inventory: kv_map! {
            "formatters": formatters.clone(),
            "package": entry_c.clone(),
        },
    };

    let expected = vec![
        (
            vec!["formatters".into(), "aarch64-linux".into()],
            entry_a.clone(),
        ),
        (
            vec!["formatters".into(), "x86_64-linux".into()],
            entry_b.clone(),
        ),
        (vec!["package".into()], entry_c.clone()),
    ];

    let mut actual = Vec::new();
    outputs.for_each_entry(|path, entry| {
        actual.push((path.into(), entry.clone()));
    });

    assert_eq!(expected, actual);
}

#[test]
fn flake_outputs_for_each_item() {
    let entry_a = Entry {
        what: Some("a formatter".into()),
        short_description: None,
        for_systems: Some(set!["aarch64-linux"]),
        derivation: Some("/nix/store/entry-a.drv".into()),
        outputs: None,
    };

    let entry_b = Entry {
        what: Some("a formatter".into()),
        short_description: None,
        for_systems: Some(set!["x86_64-linux"]),
        derivation: Some("/nix/store/entry-b.drv".into()),
        outputs: None,
    };

    let entry_c = Entry {
        what: Some("a package".into()),
        short_description: None,
        for_systems: Some(set!["x86_64-linux"]),
        derivation: Some("/nix/store/entry-c.drv".into()),
        outputs: None,
    };

    let formatters = Collection {
        children: kv_map! {
            "aarch64-linux": entry_a.clone(),
            "x86_64-linux": entry_b.clone(),
        },
    };

    let outputs = InspectOutput {
        version: 1,
        docs: Default::default(),
        inventory: kv_map! {
            "formatters": formatters.clone(),
            "package": entry_c.clone(),
        },
    };

    let expected = vec![
        (
            vec!["formatters".into()],
            InventoryItem::from(formatters.clone()),
        ),
        (
            vec!["formatters".into(), "aarch64-linux".into()],
            InventoryItem::from(entry_a.clone()),
        ),
        (
            vec!["formatters".into(), "x86_64-linux".into()],
            InventoryItem::from(entry_b.clone()),
        ),
        (vec!["package".into()], InventoryItem::from(entry_c.clone())),
    ];

    let mut actual = Vec::new();
    let result = outputs.for_each_item::<()>(|path, item| {
        actual.push((path.into(), item.clone()));
        ControlFlow::Continue(())
    });

    assert_eq!(expected, actual);
    assert_eq!(ControlFlow::Continue(()), result);
}

#[test]
fn flake_outputs_for_each_item_with_break() {
    let entry_a = Entry {
        what: Some("a formatter".into()),
        short_description: None,
        for_systems: Some(set!["aarch64-linux"]),
        derivation: Some("/nix/store/entry-a.drv".into()),
        outputs: None,
    };

    let entry_b = Entry {
        what: Some("a formatter".into()),
        short_description: None,
        for_systems: Some(set!["x86_64-linux"]),
        derivation: Some("/nix/store/entry-b.drv".into()),
        outputs: None,
    };

    let entry_c = Entry {
        what: Some("a package".into()),
        short_description: None,
        for_systems: Some(set!["x86_64-linux"]),
        derivation: Some("/nix/store/entry-c.drv".into()),
        outputs: None,
    };

    let formatters = Collection {
        children: kv_map! {
            "aarch64-linux": entry_a.clone(),
            "x86_64-linux": entry_b.clone(),
        },
    };

    let outputs = InspectOutput {
        version: 1,
        docs: Default::default(),
        inventory: kv_map! {
            "formatters": formatters.clone(),
            "package": entry_c.clone(),
        },
    };

    let expected = vec![
        (
            vec!["formatters".into()],
            InventoryItem::from(formatters.clone()),
        ),
        (
            vec!["formatters".into(), "aarch64-linux".into()],
            InventoryItem::from(entry_a.clone()),
        ),
    ];

    let mut actual = Vec::new();
    let result = outputs.for_each_item(|path, item| {
        actual.push((path.into(), item.clone()));
        if path.len() >= 2 {
            ControlFlow::Break("depth exceeded")
        } else {
            ControlFlow::Continue(())
        }
    });

    assert_eq!(expected, actual);
    assert_eq!(ControlFlow::Break("depth exceeded"), result);
}
