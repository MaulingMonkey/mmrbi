use crate::cargo::toml;

use std::ops::Deref;
use std::path::PathBuf;



/// A workspace directory, and possibly a corresponding `[workspace]`
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Workspace<WorkspaceMetadata = ::toml::value::Table> {
    pub directory:  PathBuf,
    pub toml:       toml::Workspace<WorkspaceMetadata>,
}

impl<WM> Deref for Workspace<WM> {
    type Target = toml::Workspace<WM>;
    fn deref(&self) -> &toml::Workspace<WM> { &self.toml }
}
