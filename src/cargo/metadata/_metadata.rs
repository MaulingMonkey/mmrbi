//! Locate and parse Cargo.toml files

#![cfg(feature = "serde")]
#![cfg(feature = "toml" )]
#![cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
#![cfg_attr(doc_cfg, doc(cfg(feature = "toml" )))]

mod diag;           pub use diag::*;
mod utils;          use utils::*;
mod package;        pub use package::Package;
mod packages;       pub use packages::{Packages, PackagesKey};
mod workspace;      pub use workspace::Workspace;

use crate::PathExt;
use super::toml;

use serde::de::DeserializeOwned;

use std::collections::{BTreeSet, BTreeMap};
use std::fmt::Debug;
use std::io;
use std::path::{Path, PathBuf};



/// Parsed `[workspace]` and `[package]`s
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Metadata<
    PackageMetadata     = ::toml::value::Table,
    WorkspaceMetadata   = ::toml::value::Table,
> {
    pub packages:       Packages<PackageMetadata>,
    pub workspace:      Workspace<WorkspaceMetadata>,
    pub diagnostics:    Vec<Diagnostic>,
}

impl<
    PM : Default + DeserializeOwned,
    WM : Default + DeserializeOwned,
> Metadata<PM, WM> {
    pub fn from_current_dir() -> io::Result<Self> {
        Self::from_dir(std::env::current_dir().map_err(|err| io::Error::new(err.kind(), format!("unable to resolve cwd: {}", err)))?)
    }

    pub fn from_dir(dir: impl AsRef<Path> + Into<PathBuf>) -> io::Result<Self> {
        let dir = dir.as_ref();
        let mut path = dir.canonicalize().map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{}: unable to canonicalize path: {}", dir.display(), err)))?.cleanup();
        loop {
            path.push("Cargo.toml");
            if path.exists() { return Ok(Self::from_file(path)) }
            path.pop();
            if !path.pop() { return Err(io::Error::new(io::ErrorKind::NotFound, format!("{}: Cargo.toml not found in directory nor ancestors", dir.display()))); }
        }
    }

    // TODO: util methods that convert "Diagnostics" to result errors above?
    // Errors beyond this point are packaged into Metadata.diagnostics

    // This mostly exists to get type inference right
    fn parse(bytes: &[u8]) -> Result<
        toml::Cargo<
            Option<toml::Package<PM>>,
            Option<toml::Workspace<WM>>,
        >,
        ::toml::de::Error
    > {
        ::toml::from_slice(bytes)
    }

    fn from_file(path: impl AsRef<Path> + Into<PathBuf>) -> Self {
        debug_assert!(path.as_ref().is_absolute(), "path not absolute: {}", path.as_ref().display()); // XXX: Before exposing, how do we want to handle relative paths?

        macro_rules! bail {
            ($msg:expr, $kind:expr) => { return Self {
                workspace:  Workspace { directory: pop1(path.as_ref()), toml: Default::default() },
                diagnostics:     vec![Diagnostic { path: Some(path.into()), message: $msg.into(), kind: $kind }],
                .. Self::default()
            }};
        }

        let bytes = match std::fs::read(path.as_ref())  { Ok(b) => b, Err(err) => bail!("unable to read manifest file",  DiagKind::Io(err)) };
        let cargo = match Self::parse(&bytes)           { Ok(c) => c, Err(err) => bail!("unable to parse manifest file", DiagKind::Toml(err)) };

        let (cargo, workspace) = cargo.take_workspace();
        match (workspace, cargo.with_package()) {
            (None,      None        ) => bail!("expected [package] or [workspace] table in manifest file", DiagKind::Malformed),
            (Some(ws),  None        ) => Self::from_file_workspace(path, ws),
            (None,      Some(pkg)   ) => Self::from_file_package(path, pkg),
            (Some(ws),  Some(pkg)   ) => Self::from_file_workspace_package(path, ws, pkg),
        }
    }

    pub fn from_file_workspace(path: impl AsRef<Path> + Into<PathBuf>, toml: toml::Workspace<WM>) -> Self {
        let path = path.as_ref().canonicalize().unwrap_or_else(|_| path.into()).cleanup();
        let mut metadata = Self {
            workspace:  Workspace { directory: pop1(&path), toml },
            .. Default::default()
        };

        let mut paths = BTreeSet::new();
        let mut patpath = metadata.workspace.directory.clone();

        for member in metadata.workspace.members.iter() {
            let n = paths.len();
            enum_manifest_pattern(true, &mut paths, &mut patpath, member .components(), member, &mut metadata.diagnostics);
            if n == paths.len() { metadata.diagnostics.push(Diagnostic {
                path:       Some(path.clone()),
                message:    format!("member pattern {:?} added no packages", member),
                kind:       DiagKind::Malformed,
            })}
        }

        for exclude in metadata.workspace.exclude.iter() {
            let n = paths.len();
            enum_manifest_pattern(false, &mut paths, &mut patpath, exclude.components(), exclude, &mut metadata.diagnostics);
            if n == paths.len() { metadata.diagnostics.push(Diagnostic {
                path:       Some(path.clone()),
                message:    format!("exclude pattern {:?} removed no packages", exclude),
                kind:       DiagKind::Warning,
            })}
        }

        for path in paths.into_iter() { metadata.load_pkg(path) }

        metadata
    }

    fn from_file_package(pkg_path: impl AsRef<Path> + Into<PathBuf>, pkg: toml::Cargo<toml::Package<PM>, ()>) -> Self {
        if let Some(workspace) = pkg.package.workspace.as_ref() {
            let directory = pkg_path.as_ref().join(workspace).cleanup();
            let ws_path = directory.join("Cargo.toml");

            macro_rules! bail { ($msg:expr, $kind:expr) => { return Self {
                workspace: Workspace {
                    toml: toml::Workspace {
                        members: pkg_path.as_ref().strip_prefix(&directory).ok().map(|p| p.join("..").cleanup()).into_iter().collect(),
                        ..Default::default()
                    },
                    directory,
                    ..Default::default()
                },
                packages: Packages {
                    active: 0,
                    by_name: { let mut bm = BTreeMap::new(); bm.insert(pkg.package.name.clone(), 0); bm },
                    by_path: { let mut bm = BTreeMap::new(); bm.insert(PathBuf::from(pkg_path.as_ref()), 0); bm },
                    list: vec![Package {
                        path: pkg_path.into(),
                        toml: pkg,
                    }],
                },
                diagnostics: vec![Diagnostic { path: Some(ws_path), message: $msg.into(), kind: $kind }],
                .. Default::default()
            }}}

            let bytes = match std::fs::read(&ws_path)   { Ok(b) => b, Err(err) => bail!("unable to read manifest file", DiagKind::Io(err)) };
            let cargo = match Self::parse(&bytes)       { Ok(c) => c, Err(err) => bail!("unable to parse manifest file", DiagKind::Toml(err)) };

            let (cargo, ws) = cargo.take_workspace();
            let ws = match ws { Some(ws) => ws, None => bail!("expected a [workspace]", DiagKind::Malformed) };

            let mut m = Self::from_file_workspace(&ws_path, ws);
            m.set_active(pkg_path);
            if cargo.package.is_some() {
                m.expect_contains(ws_path);
            }
            return m;

        } else { // no `package.workspace`, search for `[workspace]`-bearing Cargo.toml
            let mut search = PathBuf::from(pkg_path.as_ref());
            loop {
                search.pop();
                if !search.pop() {
                    // No `[workspace]`-bearing Cargo.toml s found, this package has no explicit workspace
                    return Self::from_file_package_standalone(pkg_path, pkg);
                }
                search.push("Cargo.toml");
                if search.exists() {
                    macro_rules! bail { ($msg:expr, $kind:expr) => {{
                        let mut m = Self::from_file_package_standalone(pkg_path, pkg);
                        m.diagnostics.push(Diagnostic{ path: Some(search), message: $msg.into(), kind: $kind });
                        return m;
                    }}}

                    let bytes = match std::fs::read(&search)    { Ok(b) => b, Err(err) => bail!("unable to read manifest file", DiagKind::Io(err)) };
                    let cargo = match Self::parse(&bytes)       { Ok(c) => c, Err(err) => bail!("unable to parse manifest file", DiagKind::Toml(err)) };
                    let (cargo, ws) = cargo.take_workspace();
                    if let Some(ws) = ws {
                        let mut m = Self::from_file_workspace(&search, ws);
                        m.set_active(pkg_path);
                        if cargo.package.is_some() {
                            m.expect_contains(search);
                        }
                        return m;
                    }
                    // else continue - not a workspace
                }
            }
        }
    }

    fn from_file_package_standalone(pkg_path: impl AsRef<Path> + Into<PathBuf>, pkg: toml::Cargo<toml::Package<PM>, ()>) -> Self {
        Self {
            workspace:  Workspace { directory: pop1(pkg_path.as_ref()), toml: toml::Workspace { members: vec![PathBuf::from(".")], ..Default::default() }, ..Default::default() },
            packages:   Packages {
                active: 0,
                by_name: { let mut bm = BTreeMap::new(); bm.insert(pkg.package.name.clone(), 0); bm },
                by_path: { let mut bm = BTreeMap::new(); bm.insert(PathBuf::from(pkg_path.as_ref()), 0); bm },
                list: vec![Package {
                    path: pkg_path.into(),
                    toml: pkg,
                }],
            },
            .. Default::default()
        }
    }

    fn from_file_workspace_package(path: impl AsRef<Path> + Into<PathBuf>, ws: toml::Workspace<WM>, _pkg: toml::Cargo<toml::Package<PM>, ()>) -> Self {
        let mut metadata = Self::from_file_workspace(path.as_ref(), ws);
        metadata.set_active(path);
        metadata
    }

    fn load_pkg(&mut self, path: PathBuf) {
        debug_assert!(path.is_absolute());
        debug_assert!(!self.packages.by_path.contains_key(&path));
        macro_rules! bail { ($msg:expr, $kind:expr) => { return self.diagnostics.push(Diagnostic{ path: Some(path), message: $msg.into(), kind: $kind })}; }

        let bytes = match std::fs::read(&path)  { Ok(b) => b, Err(err) => bail!("unable to read manifest file", DiagKind::Io(err)) };
        let cargo = match Self::parse(&bytes)   { Ok(c) => c, Err(err) => bail!("unable to parse manifest file", DiagKind::Toml(err)) };
        let (cargo, _ws) = cargo.take_workspace(); // TODO: Validate this matches the current workspace
        let pkg   = match cargo.with_package()  { Some(p) => p, None => bail!("missing [package] in manifest", DiagKind::Malformed) };

        let i = self.packages.list.len();

        if let Some(prev) = self.packages.by_name.insert(pkg.package.name.clone(), i) {
            let prev = &self.packages.list[prev];
            self.diagnostics.push(Diagnostic{
                path:       Some(path.clone()),
                message:    format!("multiple workspace packages named {:?}", prev.package.name),
                kind:       DiagKind::Malformed,
            });
            self.diagnostics.push(Diagnostic{
                path:       Some(prev.path.clone()),
                message:    format!("previous package named {:?}", prev.package.name),
                kind:       DiagKind::Malformed,
            });
        }

        if let Some(prev) = self.packages.by_path.insert(path.clone(), i) {
            let prev = &self.packages.list[prev];
            self.diagnostics.push(Diagnostic{
                path:       Some(prev.path.clone()),
                message:    format!("multiple packages at path"),
                kind:       DiagKind::Bug,
            });
        }

        self.packages.list.push(Package {
            path,
            toml: pkg,
        });
    }

    fn set_active(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        debug_assert!(path.is_absolute());
        debug_assert!(self.packages.active == std::usize::MAX);
        let active = self.packages.by_path.get(path).copied();
        self.packages.active = active.unwrap_or(std::usize::MAX);
        if active.is_none() { self.diagnostics.push(Diagnostic{
            path:       Some(PathBuf::from(path)),
            message:    format!("is expected to be the active project, but it is not part of the workspace"),
            kind:       DiagKind::Malformed,
        })}
    }

    fn expect_contains(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        debug_assert!(path.is_absolute());
        let found = self.packages.by_path.get(path);
        if found.is_none() { self.diagnostics.push(Diagnostic{
            path:       Some(PathBuf::from(path)),
            message:    format!("is expected to be part of the workspace, but it is not"),
            kind:       DiagKind::Malformed,
        })}
    }
}




#[cfg(test)] mod tests {
    use super::*;

    #[test] fn deserialize() {
        let cwd = std::env::current_dir().unwrap().cleanup();
        let meta : Metadata = Metadata::from_current_dir().unwrap();

        assert_eq!(meta.workspace.directory, cwd);
        assert!(meta.workspace.members.iter().any(|m| m == Path::new(".")), "meta.workspace.members: {:#?}",    meta.workspace.members);
        assert!(meta.workspace.exclude.len() > 0,                           "meta.workspace.exclude: {:#?}",    meta.workspace.exclude);
        assert!(meta.packages.len() >= 2,                                   "meta.packages: {:#?}",             meta.packages);

        let mmrbi = meta.packages.get("mmrbi").expect("meta.packages.get(\"mmrbi\")");
        assert_eq!(mmrbi.path,                                      cwd.join("Cargo.toml"));
        assert_eq!(mmrbi.package.name,                              "mmrbi");
        assert_eq!(mmrbi.package.repository.as_ref().unwrap(),      "https://github.com/MaulingMonkey/mmrbi");
        assert_eq!(mmrbi.package.documentation.as_ref().unwrap(),   "https://docs.rs/mmrbi/");
        assert_eq!(mmrbi.package.edition,                           toml::package::Edition::V2018);
        assert_eq!(mmrbi.package.publish,                           true);
        assert!(mmrbi.package.authors.iter().any(|a| a == "MaulingMonkey <git@maulingmonkey.com>"),    "mmrbi.package.authors: {:#?}", mmrbi.package.authors);
        // license, readme, description, keywords, categories, ...

        let script = meta.packages.get("examples-script").expect("meta.packages.get(\"examples-script\")");
        assert_eq!(script.path,                 cwd.join("examples").join("script").join("Cargo.toml"));
        assert_eq!(script.package.name,         "examples-script");
        assert_eq!(script.package.publish,      false);
    }

    #[test] fn deserialize_misc_dir() {
        let _meta : Metadata = Metadata::from_dir("src/cargo").unwrap();
    }

    #[test] fn deserialize_leaf_package() {
        let _meta : Metadata = Metadata::from_dir("examples/script").unwrap();
    }

    #[test] fn deserialize_leaf_package_explicit_ws() {
        let _meta : Metadata = Metadata::from_dir("examples/explicit-package-path").unwrap();
    }
}
