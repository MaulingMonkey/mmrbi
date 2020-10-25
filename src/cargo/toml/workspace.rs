use serde::*;
use std::path::PathBuf;



/// [`[workspace]`](https://doc.rust-lang.org/cargo/reference/workspaces.html)
/// — The workspace definition.
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
#[serde(rename_all="kebab-case")]
pub struct Workspace<Metadata = toml::value::Table> {
    #[serde(default)] pub members:          Vec<PathBuf>,
    #[serde(default)] pub exclude:          Vec<PathBuf>,
    #[serde(default)] pub default_members:  Vec<PathBuf>,
    #[serde(default)] pub metadata:         Metadata,
}
