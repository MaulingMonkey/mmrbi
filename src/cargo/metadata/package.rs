use crate::cargo::toml;

use std::path::PathBuf;
use std::ops::Deref;



/// Parsed Cargo.toml containing a `[package]`
#[derive(Debug)]
#[non_exhaustive]
pub struct Package<PackageMetadata = ::toml::value::Table> {
    pub path: PathBuf,
    pub toml: toml::Cargo<toml::Package<PackageMetadata>, ()>,
}

impl<PM> Deref for Package<PM> {
    type Target = toml::Cargo<toml::Package<PM>, ()>;
    fn deref(&self) -> &toml::Cargo<toml::Package<PM>, ()> { &self.toml }
}
