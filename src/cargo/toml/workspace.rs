use serde::*;

/// [`[workspace]`](https://doc.rust-lang.org/cargo/reference/workspaces.html)
/// — The workspace definition.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
#[serde(rename_all="kebab-case")]
pub struct Workspace<Metadata = toml::value::Table> {
    #[serde(default)] pub members:          Vec<String>,
    #[serde(default)] pub exclude:          Vec<String>,
    #[serde(default)] pub default_members:  Vec<String>,
    #[serde(default)] pub metadata:         Metadata,
}
