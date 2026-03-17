// NOTE: using BTreeMap preserves the ordering we see from nix (via the inspect flake)
use std::collections::{BTreeMap, BTreeSet};
use std::ops::ControlFlow;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Represents an object in a flake schema.
/// Typically this is a derivation that can be built, but not always (such as NixOS modules or overlays).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub struct Entry {
    /// The descriptor for this object, like "package" or "Nixpkgs overlay".
    pub what: String,

    /// The short description, if one is present.
    /// Most often, this is either always `Some` or always `None` for a given class of flake output.
    /// (That is, if a package is expected to have a description and doesn't, flake-schemas will default to the empty string instead of `null`.)
    pub short_description: Option<String>,

    /// A map of its output names to store paths.
    /// Empty if this entry is not a derivation.
    pub outputs: Option<BTreeMap<String, PathBuf>>,

    /// Which systems, if any, this entry applies to.
    pub for_systems: Option<BTreeSet<String>>,

    /// If this entry is a derivation, the path to the `.drv` file.
    pub derivation: Option<PathBuf>,
}

/// Represents a key-value collection of objects in a flake schema.
/// At various points in the tree, the keys can represent output types (like `packages` or `nixosModules`), systems (`x86_64-linux` or `aarch64-darwin`), or other.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Collection {
    pub children: BTreeMap<String, InventoryItem>,
}

/// Represents an arbitrary point in a flake-schemas inventory.
/// This can be a key-value collection or an entry.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
#[cfg_attr(test, derive(PartialEq))]
pub enum InventoryItem {
    Collection(Collection),
    Entry(Entry),
}

// Test helper for manual construction
#[cfg(test)]
impl From<Collection> for InventoryItem {
    fn from(value: Collection) -> Self {
        Self::Collection(value)
    }
}

// Test helper for manual construction
#[cfg(test)]
impl From<Entry> for InventoryItem {
    fn from(value: Entry) -> Self {
        Self::Entry(value)
    }
}

/// Represents the output of the [`inspect` flake][1].
///
/// [1]: https://github.com/DeterminateSystems/inspect
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(test, derive(PartialEq))]
pub struct InspectOutput {
    // TODO(gustavderdrache): validate that this is always 1
    /// The version of this output.
    /// Should always be 1.
    pub version: u32,

    /// A map of documentation snippets for each collected output in this flake.
    pub docs: BTreeMap<String, String>,

    /// A map of inventoried objects
    pub inventory: BTreeMap<String, InventoryItem>,
}

impl InspectOutput {
    /// Create an empty inspection result.
    /// This is useful in code paths that wish to avoid inspecting flakes for various reasons.
    pub fn new() -> Self {
        Self {
            version: 1,
            docs: Default::default(),
            inventory: Default::default(),
        }
    }
}

impl InventoryItem {
    fn for_each_item_impl<T>(
        &self,
        path: &mut Vec<String>,
        visitor: &mut impl FnMut(&[String], &InventoryItem) -> ControlFlow<T>,
    ) -> ControlFlow<T> {
        visitor(path, self)?;

        if let InventoryItem::Collection(collection) = self {
            collection.for_each_item_impl(path, visitor)?;
        }

        ControlFlow::Continue(())
    }

    /// Visit each [`InventoryItem`] recursively.
    /// The `visitor` callback is expected to return a [`ControlFlow`] indicating if recursive processing should continue.
    ///
    /// Visitors are called with two arguments:
    /// 1. `path: &[String]`, indicating where in the flake this inventory item is (e.g., `["packages", "x86_64-linux", "default"]`).
    /// 2. `item: &InventoryItem`, an inventoried item itself (a [`Collection`] or [`Entry`]).
    ///
    /// See also:
    /// * [`InventoryItem::for_each_entry`], a helper that unconditionally visits [`Entry`] nodes.
    /// * [`Collection::for_each_item`], the same helper for `Collection` objects.
    /// * [`InspectOutput::for_each_item`], the same helper for `InspectOutput` objects.
    #[inline]
    pub fn for_each_item<T>(
        &self,
        mut visitor: impl FnMut(&[String], &InventoryItem) -> ControlFlow<T>,
    ) -> ControlFlow<T> {
        let mut path = Vec::new();
        self.for_each_item_impl(&mut path, &mut visitor)
    }

    /// Unconditionally visit each [`Entry`] potentially contained within this inventory item.
    ///
    /// Visitors are called with two arguments:
    /// 1. `path: &[String]`, just like [`InventoryItem::for_each_item`].
    /// 2. `entry: &Entry`, a leaf node in this inventory.
    ///
    /// See also:
    /// * [`Collection::for_each_entry`], the same helper for `Collection` objects.
    /// * [`InspectOutput::for_each_entry`], the same helper for `InspectOutput` objects.
    #[inline]
    pub fn for_each_entry(&self, mut visitor: impl FnMut(&[String], &Entry)) {
        let _ = self.for_each_item::<()>(|path, item| {
            if let InventoryItem::Entry(entry) = item {
                visitor(path, entry);
            }

            ControlFlow::Continue(())
        });
    }
}

impl Collection {
    fn for_each_item_impl<T>(
        &self,
        path: &mut Vec<String>,
        visitor: &mut impl FnMut(&[String], &InventoryItem) -> ControlFlow<T>,
    ) -> ControlFlow<T> {
        for (name, value) in &self.children {
            path.push(name.into());

            value.for_each_item_impl(path, visitor)?;

            path.pop();
        }

        ControlFlow::Continue(())
    }

    /// Visit each [`InventoryItem`] recursively.
    /// The `visitor` callback is expected to return a [`ControlFlow`] indicating if recursive processing should continue.
    ///
    /// Visitors are called with two arguments:
    /// 1. `path: &[String]`, indicating where in the flake this inventory item is (e.g., `["packages", "x86_64-linux", "default"]`).
    /// 2. `item: &InventoryItem`, an inventoried item itself (a [`Collection`] or [`Entry`]).
    ///
    /// See also:
    /// * [`Collection::for_each_entry`], a helper that unconditionally visits [`Entry`] nodes.
    /// * [`InventoryItem::for_each_item`], the same helper for `InventoryItem` objects.
    /// * [`InspectOutput::for_each_item`], the same helper for `InspectOutput` objects.
    #[inline]
    pub fn for_each_item<T>(
        &self,
        mut visitor: impl FnMut(&[String], &InventoryItem) -> ControlFlow<T>,
    ) -> ControlFlow<T> {
        let mut path = Vec::new();
        self.for_each_item_impl(&mut path, &mut visitor)
    }

    /// Unconditionally visit each [`Entry`] potentially contained within this inventory item.
    ///
    /// Visitors are called with two arguments:
    /// 1. `path: &[String]`, just like [`Collection::for_each_item`].
    /// 2. `entry: &Entry`, a leaf node in this inventory.
    ///
    /// See also:
    /// * [`InventoryItem::for_each_entry`], the same helper for `InventoryItem` objects.
    /// * [`InspectOutput::for_each_entry`], the same helper for `InspectOutput` objects.
    #[inline]
    pub fn for_each_entry(&self, mut visitor: impl FnMut(&[String], &Entry)) {
        let _ = self.for_each_item::<()>(|path, item| {
            if let InventoryItem::Entry(entry) = item {
                visitor(path, entry);
            }

            ControlFlow::Continue(())
        });
    }
}

impl InspectOutput {
    /// Visit each [`InventoryItem`] recursively.
    /// The `visitor` callback is expected to return a [`ControlFlow`] indicating if recursive processing should continue.
    ///
    /// Visitors are called with two arguments:
    /// 1. `path: &[String]`, indicating where in the flake this inventory item is (e.g., `["packages", "x86_64-linux", "default"]`).
    /// 2. `item: &InventoryItem`, an inventoried item itself (a [`Collection`] or [`Entry`]).
    ///
    /// See also:
    /// * [`InspectOutput::for_each_entry`], a helper that unconditionally visits [`Entry`] nodes.
    /// * [`InventoryItem::for_each_item`], the same helper for `InventoryItem` objects.
    /// * [`Collection::for_each_item`], the same helper for `Collection` objects.
    #[inline]
    pub fn for_each_item<T>(
        &self,
        mut visitor: impl FnMut(&[String], &InventoryItem) -> ControlFlow<T>,
    ) -> ControlFlow<T> {
        let mut path = Vec::new();
        for (name, value) in &self.inventory {
            path.push(name.into());
            value.for_each_item_impl(&mut path, &mut visitor)?;
            path.pop();
        }

        ControlFlow::Continue(())
    }

    /// Unconditionally visit each [`Entry`] potentially contained within this inventory item.
    ///
    /// Visitors are called with two arguments:
    /// 1. `path: &[String]`, just like [`InspectOutput::for_each_item`].
    /// 2. `entry: &Entry`, a leaf node in this inventory.
    ///
    /// See also:
    /// * [`InventoryItem::for_each_entry`], the same helper for `InventoryItem` objects.
    /// * [`Collection::for_each_entry`], the same helper for `Collection` objects.
    #[inline]
    pub fn for_each_entry(&self, mut visitor: impl FnMut(&[String], &Entry)) {
        let _ = self.for_each_item::<()>(|path, item| {
            if let InventoryItem::Entry(entry) = item {
                visitor(path, entry);
            }

            ControlFlow::Continue(())
        });
    }
}
