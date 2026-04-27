use std::{
    collections::{BTreeMap, BTreeSet},
    ops::ControlFlow,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Output {
    /// The version of this output.
    ///
    /// Should always be 2.
    pub version: u32,

    pub inventory: BTreeMap<String, InventoryItem>,
}

impl<'de> Deserialize<'de> for Output {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct OutputInner {
            version: u32,
            inventory: BTreeMap<String, InventoryItem>,
        }

        let inner = OutputInner::deserialize(deserializer)?;

        if inner.version != 2 {
            return Err(serde::de::Error::custom(format!(
                "expected `nix flake show` output version to be `2`, but got `{}` instead",
                inner.version
            )));
        }

        Ok(Self {
            version: inner.version,
            inventory: inner.inventory,
        })
    }
}

impl Output {
    pub fn visit_items<F, T>(&self, mut f: F) -> Option<T>
    where
        F: FnMut(&str, &[String], &InventoryItem) -> ControlFlow<T>,
    {
        let mut path = vec![];

        for (k, v) in &self.inventory {
            path.push(k.clone());

            if let ControlFlow::Break(e) = f(k, &path, v) {
                return Some(e);
            }

            if let ControlFlow::Break(e) = v.visit_children(&mut path, &mut f) {
                return Some(e);
            }

            path.pop();
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub what: String,
    pub short_description: Option<String>,
    #[serde(flatten)]
    pub contents: Option<EntryContents>,
}

impl Entry {
    #[must_use]
    pub const fn is_derivation(&self) -> bool {
        matches!(&self.contents, Some(EntryContents::Derivation { .. }))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EntryContents {
    #[serde(rename_all = "camelCase")]
    Derivation {
        derivation: Derivation,
        for_systems: BTreeSet<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Derivation {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub doc: Option<String>,
    pub output: Children,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Children {
    pub children: BTreeMap<String, InventoryItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum InventoryItem {
    Filtered {
        filtered: bool,
    },
    Unknown {
        unknown: bool,
    },
    #[serde(rename_all = "camelCase")]
    Legacy {
        is_legacy: bool,
    },
    Entry(Entry),
    Collection(Collection),
    Children(Children),
}

impl InventoryItem {
    pub fn visit_children<F, T>(&self, path: &mut Vec<String>, f: &mut F) -> ControlFlow<T>
    where
        F: FnMut(&str, &[String], &Self) -> ControlFlow<T>,
    {
        if let Self::Collection(collection) = self {
            for (k, v) in &collection.output.children {
                path.push(k.clone());

                if let ControlFlow::Break(e) = f(k, path, v) {
                    return ControlFlow::Break(e);
                }

                if let ControlFlow::Break(e) = v.visit_children(path, f) {
                    return ControlFlow::Break(e);
                }

                path.pop();
            }
        } else if let Self::Children(children) = self {
            for (k, v) in &children.children {
                path.push(k.clone());

                if let ControlFlow::Break(e) = f(k, path, v) {
                    return ControlFlow::Break(e);
                }

                if let ControlFlow::Break(e) = v.visit_children(path, f) {
                    return ControlFlow::Break(e);
                }

                path.pop();
            }
        }

        ControlFlow::Continue(())
    }
}
