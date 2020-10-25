use crate::cargo::toml;

use std::path::PathBuf;
use std::ops::Deref;



/// Parsed Cargo.toml containing a `[package]`
#[derive(Debug)]
#[non_exhaustive]
pub struct Package {
    pub path: PathBuf,
    pub toml: toml::Cargo<toml::Package, ()>,
}

impl Deref for Package {
    type Target = toml::Cargo<toml::Package, ()>;
    fn deref(&self) -> &toml::Cargo<toml::Package, ()> { &self.toml }
}
