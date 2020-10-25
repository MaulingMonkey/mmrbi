use serde::*;
use std::path::PathBuf;



/// [`[package]`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section)
/// — Defines a package.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
#[serde(rename_all="kebab-case")]
pub struct Package<Metadata = toml::value::Table> {
    pub name:           String,
    pub version:        String,
    #[serde(default)] pub authors:        Vec<String>,
    #[serde(default)] pub edition:        Option<String>,
    #[serde(default)] pub description:    Option<String>,
    #[serde(default)] pub documentation:  Option<String>,
    #[serde(default)] pub readme:         Option<PathBuf>,
    #[serde(default)] pub homepage:       Option<String>,
    #[serde(default)] pub repository:     Option<String>,
    #[serde(default)] pub license:        Option<String>,
    #[serde(default)] pub license_file:   Option<PathBuf>,
    #[serde(default)] pub keywords:       Vec<String>,
    #[serde(default)] pub categories:     Vec<String>,
    #[serde(default)] pub workspace:      Option<PathBuf>,
    #[serde(default)] pub build:          Option<PathBuf>,
    #[serde(default)] pub links:          Option<String>,
    #[serde(default)] pub exclude:        Vec<String>,
    #[serde(default)] pub include:        Vec<String>,
    #[serde(default)] pub publish:        Option<String>,
    #[serde(default)] pub metadata:       Metadata,
    #[serde(default)] pub default_run:    Option<String>,
    #[serde(default)] pub autobins:       Option<bool>,
    #[serde(default)] pub autoexamples:   Option<bool>,
    #[serde(default)] pub autotests:      Option<bool>,
    #[serde(default)] pub autobenches:    Option<bool>,
    #[serde(flatten)] rest:               toml::value::Table
}
