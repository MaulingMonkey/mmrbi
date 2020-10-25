use crate::cargo::toml;

use std::ops::Deref;
use std::path::PathBuf;



/// A workspace directory, and possibly a corresponding `[workspace]`
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Workspace {
    pub directory:  PathBuf,
    pub toml:       toml::Workspace,
}

impl Deref for Workspace {
    type Target = toml::Workspace;
    fn deref(&self) -> &toml::Workspace { &self.toml }
}
