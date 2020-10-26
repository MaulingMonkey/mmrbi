use super::Target;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;



/// [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html)
/// — The root of the parsed manifest, **without** context like "what was Cargo.toml's path?"
#[derive(Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
#[serde(rename_all="kebab-case")]
pub struct Cargo<
    Package     = Option<super::Package>,
    Workspace   = Option<super::Workspace>,
> {
    #[serde(default)] pub cargo_features:       BTreeSet<String>,
    pub package:                                Package,

    // Target tables
    #[serde(default, rename="lib"       )] pub lib:         Option<Target>,
    #[serde(default, rename="bin"       )] pub bins:        Vec<Target>,
    #[serde(default, rename="example"   )] pub examples:    Vec<Target>,
    #[serde(default, rename="test"      )] pub tests:       Vec<Target>,
    #[serde(default, rename="bench"     )] pub benches:     Vec<Target>,

    // Dependency tables
    #[serde(default)] pub dependencies:         toml::value::Table,
    #[serde(default)] pub dev_dependencies:     toml::value::Table,
    #[serde(default)] pub build_dependencies:   toml::value::Table,
    #[serde(default)] pub target:               toml::value::Table,

    #[serde(default)] pub badges:               toml::value::Table,
    #[serde(default)] pub features:             toml::value::Table,
    #[serde(default)] pub patch:                toml::value::Table,
    #[serde(default)] pub replace:              toml::value::Table,
    #[serde(default)] pub profile:              toml::value::Table,

    pub workspace:                              Workspace,

    #[serde(flatten)] rest:                     toml::value::Table
}

impl<Package, Workspace> Cargo<Package, Workspace> {
    pub fn take_workspace(self) -> (Cargo<Package, ()>, Workspace) {
        let Cargo {
            cargo_features, package,
            lib, bins, examples, tests, benches,
            dependencies, dev_dependencies, build_dependencies, target,
            badges, features, patch, replace, profile,
            workspace, rest
        } = self;

        (Cargo {
            cargo_features, package,
            lib, bins, examples, tests, benches,
            dependencies, dev_dependencies, build_dependencies, target,
            badges, features, patch, replace, profile,
            workspace: (), rest
        }, workspace)
    }

    pub fn with_workspace<W>(self, workspace: W) -> Cargo<Package, W> {
        let Cargo {
            cargo_features, package,
            lib, bins, examples, tests, benches,
            dependencies, dev_dependencies, build_dependencies, target,
            badges, features, patch, replace, profile,
            workspace: _, rest
        } = self;

        Cargo {
            cargo_features, package,
            lib, bins, examples, tests, benches,
            dependencies, dev_dependencies, build_dependencies, target,
            badges, features, patch, replace, profile,
            workspace, rest
        }
    }
}

impl<Package, Workspace> Cargo<Option<Package>, Workspace> {
    pub fn with_package(self) -> Option<Cargo<Package, Workspace>> {
        let Cargo {
            cargo_features, package,
            lib, bins, examples, tests, benches,
            dependencies, dev_dependencies, build_dependencies, target,
            badges, features, patch, replace, profile,
            workspace, rest
        } = self;

        match package {
            None => None,
            Some(package) => Some(Cargo{
                cargo_features, package,
                lib, bins, examples, tests, benches,
                dependencies, dev_dependencies, build_dependencies, target,
                badges, features, patch, replace, profile,
                workspace, rest
            }),
        }
    }
}

#[cfg(test)] mod tests {
    use super::*;
    use std::path::Path;

    #[test] fn deserialize() {
        let path = std::path::Path::new("Cargo.toml");
        let bytes = std::fs::read(path).unwrap();
        let cargo : Cargo = ::toml::from_slice(&bytes[..]).unwrap();

        let package = cargo.package.unwrap();
        let workspace = cargo.workspace.unwrap();
        assert!(workspace.members.iter().any(|member| member == Path::new(".")));
        assert_eq!(package.name, "mmrbi");
    }

    #[test] fn deserialize_leaf() {
        let path = std::path::Path::new("examples/script/Cargo.toml");
        let bytes = std::fs::read(path).unwrap();
        let cargo : Cargo = ::toml::from_slice(&bytes[..]).unwrap();

        let package = cargo.package.unwrap();
        assert!(cargo.workspace.is_none(), "[workspace] not expected in this manifest");
        assert_eq!(package.name, "examples-script");
    }
}
