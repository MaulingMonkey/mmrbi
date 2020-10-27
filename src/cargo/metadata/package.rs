use crate::cargo::toml;

use std::path::{Path, PathBuf};
use std::ops::Deref;



/// Parsed Cargo.toml containing a `[package]`
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Package<PackageMetadata = ::toml::value::Table> {
    pub(crate) path: PathBuf,
    pub(crate) toml: toml::Cargo<toml::Package<PackageMetadata>, ()>,
}

impl<PM> Package<PM> {
    /// Path to the `Cargo.toml` this `[package]` was parsed out of
    pub fn manifest_path(&self) -> &Path { &self.path }

    /// Path to the directory containing the `Cargo.toml` this `[package]` was parsed out of
    pub fn directory(&self) -> &Path { self.path.parent().unwrap() }

    /// Parsed `Cargo.toml` contents (also available via [Deref])
    pub fn toml(&self) -> &toml::Cargo<toml::Package<PM>, ()> { &self.toml }
}

impl<PM> Deref for Package<PM> {
    type Target = toml::Cargo<toml::Package<PM>, ()>;
    fn deref(&self) -> &toml::Cargo<toml::Package<PM>, ()> { &self.toml }
}
