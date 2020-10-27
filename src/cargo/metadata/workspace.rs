use crate::cargo::toml;

use std::ops::Deref;
use std::path::{Path, PathBuf};



/// A workspace [directory](Workspace::directory), and corresponding <code>\[[workspace](toml::Workspace)\]</code>
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Workspace<WorkspaceMetadata = ::toml::value::Table> {
    pub(crate) directory:  PathBuf,
    pub(crate) toml:       toml::Workspace<WorkspaceMetadata>,
}

impl <WM> Workspace<WM> {
    /// Path to the directory containing the `Cargo.toml` this <code>\[[workspace](toml::Workspace)\]</code> was parsed out of
    pub fn directory(&self) -> &Path { &self.directory }

    /// Parsed `Cargo.toml` contents (also available via [Deref])
    pub fn toml(&self) -> &toml::Workspace<WM> { &self.toml }
}

impl<WM> Deref for Workspace<WM> {
    type Target = toml::Workspace<WM>;
    fn deref(&self) -> &toml::Workspace<WM> { &self.toml }
}
