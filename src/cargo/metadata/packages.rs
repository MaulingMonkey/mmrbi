use super::Package;

use nonmax::NonMaxUsize;

use std::collections::BTreeMap;
use std::fmt::{self, Debug, Formatter};
use std::ops::Index;
use std::path::{Path, PathBuf};



/// An indexed list of Cargo.toml files containing `[package]`s
#[derive(Default)]
#[non_exhaustive]
pub struct Packages {
    pub(super) list:    Vec<Package>,
    pub(super) by_name: BTreeMap<String, usize>,
    pub(super) by_path: BTreeMap<PathBuf, usize>,
    pub(super) active:  Option<NonMaxUsize>,
}

impl Packages {
    pub fn active(&self) -> Option<&Package> { self.active.map(|a| &self.list[a.get()]) }
    pub fn get(&self, key: impl PackagesKey) -> Option<&Package> { key.get(self) }
    pub fn is_empty(&self) -> bool { self.list.is_empty() }
    pub fn len(&self) -> usize { self.list.len() }
    pub fn iter(&self) -> std::slice::Iter<Package> { self.list.iter() }
}

/// A means of looking up parsed Cargo.toml `[package]`s
pub trait PackagesKey           { fn get(self, packages: &Packages) -> Option<&Package>; }
impl PackagesKey for usize      { fn get(self, packages: &Packages) -> Option<&Package> { packages.list.get(self) } }
impl PackagesKey for &str       { fn get(self, packages: &Packages) -> Option<&Package> { packages.list.get(*packages.by_name.get(self)?) } }
impl PackagesKey for  String    { fn get(self, packages: &Packages) -> Option<&Package> { packages.list.get(*packages.by_name.get(&self)?) } }
impl PackagesKey for &String    { fn get(self, packages: &Packages) -> Option<&Package> { packages.list.get(*packages.by_name.get(self)?) } }
impl PackagesKey for &Path      { fn get(self, packages: &Packages) -> Option<&Package> { packages.list.get(*packages.by_path.get(self)?) } }
impl PackagesKey for  PathBuf   { fn get(self, packages: &Packages) -> Option<&Package> { packages.list.get(*packages.by_path.get(&self)?) } }
impl PackagesKey for &PathBuf   { fn get(self, packages: &Packages) -> Option<&Package> { packages.list.get(*packages.by_path.get(self)?) } }

impl Index<usize> for Packages {
    type Output = Package;
    fn index(&self, index: usize) -> &Self::Output { &self.list[index] }
}

impl Debug for Packages { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&self.list, fmt) } }
