use serde::{Deserialize, Serialize};

use std::collections::BTreeSet;
use std::path::PathBuf;



/// [`[lib]`, `[[bin]]`, `[[example]]`, `[[test]]`, or `[[bench]]`](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)
#[derive(Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
#[serde(rename_all="kebab-case")]
pub struct Target {
    // https://doc.rust-lang.org/cargo/reference/cargo-targets.html#configuring-a-target
    #[serde(default)] pub name:                 Option<String>,
    #[serde(default)] pub path:                 Option<PathBuf>,
    #[serde(default)] pub test:                 Option<bool>,
    #[serde(default)] pub doctest:              Option<bool>,
    #[serde(default)] pub bench:                Option<bool>,
    #[serde(default)] pub doc:                  Option<bool>,
    #[serde(default)] pub plugin:               Option<bool>, // deprecated
    #[serde(default)] pub proc_macro:           Option<bool>,
    #[serde(default)] pub harness:              Option<bool>,
    #[serde(default)] pub edition:              Option<String>,
    #[serde(default)] pub crate_type:           Vec<String>,
    #[serde(default)] pub required_features:    BTreeSet<String>,
    #[serde(flatten)] rest:                     toml::value::Table
}
