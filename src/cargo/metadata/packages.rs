use crate::cargo::toml::package::Name;
use super::Package;

use nonmax::NonMaxUsize;

use std::collections::BTreeMap;
use std::fmt::{self, Debug, Formatter};
use std::ops::Index;
use std::path::{Path, PathBuf};



/// An indexed list of Cargo.toml files containing `[package]`s
#[derive(Default)]
#[non_exhaustive]
pub struct Packages<PackageMetadata = toml::value::Table> {
    pub(super) list:    Vec<Package<PackageMetadata>>,
    pub(super) by_name: BTreeMap<Name, usize>,
    pub(super) by_path: BTreeMap<PathBuf, usize>,
    pub(super) active:  Option<NonMaxUsize>,
}

impl<PM> Packages<PM> {
    pub fn active(&self) -> Option<&Package<PM>> { self.active.map(|a| &self.list[a.get()]) }
    pub fn get(&self, key: impl PackagesKey) -> Option<&Package<PM>> { key.get(self) }
    pub fn is_empty(&self) -> bool { self.list.is_empty() }
    pub fn len(&self) -> usize { self.list.len() }
    pub fn iter(&self) -> std::slice::Iter<Package<PM>> { self.list.iter() }
}

/// A means of looking up parsed Cargo.toml `[package]`s
pub trait PackagesKey           { fn get<PM>(self, packages: &Packages<PM>) -> Option<&Package<PM>>; }
impl PackagesKey for usize      { fn get<PM>(self, packages: &Packages<PM>) -> Option<&Package<PM>> { packages.list.get(self) } }
impl PackagesKey for &str       { fn get<PM>(self, packages: &Packages<PM>) -> Option<&Package<PM>> { packages.list.get(*packages.by_name.get(self)?) } }           // deprecate?
impl PackagesKey for  String    { fn get<PM>(self, packages: &Packages<PM>) -> Option<&Package<PM>> { packages.list.get(*packages.by_name.get(self.as_str())?) } }  // deprecate?
impl PackagesKey for &String    { fn get<PM>(self, packages: &Packages<PM>) -> Option<&Package<PM>> { packages.list.get(*packages.by_name.get(self.as_str())?) } }  // deprecate?
impl PackagesKey for  Name      { fn get<PM>(self, packages: &Packages<PM>) -> Option<&Package<PM>> { packages.list.get(*packages.by_name.get(self.as_str())?) } }
impl PackagesKey for &Name      { fn get<PM>(self, packages: &Packages<PM>) -> Option<&Package<PM>> { packages.list.get(*packages.by_name.get(self.as_str())?) } }
impl PackagesKey for &Path      { fn get<PM>(self, packages: &Packages<PM>) -> Option<&Package<PM>> { packages.list.get(*packages.by_path.get(self)?) } }
impl PackagesKey for  PathBuf   { fn get<PM>(self, packages: &Packages<PM>) -> Option<&Package<PM>> { packages.list.get(*packages.by_path.get(&self)?) } }
impl PackagesKey for &PathBuf   { fn get<PM>(self, packages: &Packages<PM>) -> Option<&Package<PM>> { packages.list.get(*packages.by_path.get(self)?) } }

impl<PM> Index<usize> for Packages<PM> {
    type Output = Package<PM>;
    fn index(&self, index: usize) -> &Self::Output { &self.list[index] }
}

impl<PM : Debug> Debug for Packages<PM> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&self.list, fmt) }
}
