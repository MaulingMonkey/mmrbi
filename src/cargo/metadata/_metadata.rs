//! Locate and parse Cargo.toml files

mod diag;           pub use diag::*;
mod utils;          use utils::*;
mod package;        pub use package::Package;
mod packages;       pub use packages::{Packages, PackagesKey};
mod workspace;      pub use workspace::Workspace;

use crate::PathExt;
use super::toml;

use nonmax::NonMaxUsize;

use std::collections::BTreeSet;
use std::fmt::Debug;
use std::io;
use std::path::{Path, PathBuf};



/// Parsed `[workspace]` and `[package]`s
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Metadata {
    pub packages:       Packages,
    pub workspace:      Workspace,
    pub diagnostics:    Vec<Diagnostic>,
}

impl Metadata {
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

    fn from_file(path: impl AsRef<Path> + Into<PathBuf>) -> Self {
        debug_assert!(path.as_ref().is_absolute(), "path not absolute: {}", path.as_ref().display()); // XXX: Before exposing, how do we want to handle relative paths?

        macro_rules! bail {
            ($msg:expr, $kind:expr) => { return Self {
                workspace:  Workspace { directory: pop1(path.as_ref()), toml: Default::default() },
                diagnostics:     vec![Diagnostic { path: Some(path.into()), message: $msg.into(), kind: $kind }],
                .. Self::default()
            }};
        }

        let bytes               = match std::fs::read(path.as_ref())    { Ok(b) => b, Err(err) => bail!("unable to read manifest file", DiagKind::Io(err)) };
        let cargo : toml::Cargo = match ::toml::from_slice(&bytes[..])  { Ok(c) => c, Err(err) => bail!("unable to parse manifest file", DiagKind::Toml(err)) };

        let (cargo, workspace) = cargo.take_workspace();
        match (workspace, cargo.with_package()) {
            (None,      None        ) => bail!("expected [package] or [workspace] table in manifest file", DiagKind::Malformed),
            (Some(ws),  None        ) => Self::from_file_workspace(path, ws),
            (None,      Some(_pkg)  ) => unimplemented!(),
            (Some(ws),  Some(pkg)   ) => Self::from_file_workspace_package(path, ws, pkg),
        }
    }

    fn from_file_workspace(path: impl AsRef<Path> + Into<PathBuf>, toml: toml::Workspace) -> Self {
        let mut metadata = Metadata {
            workspace:  Workspace { directory: pop1(path.as_ref()), toml },
            .. Default::default()
        };

        let mut paths = BTreeSet::new();
        let mut patpath = metadata.workspace.directory.clone();

        for member in metadata.workspace.members.iter() {
            let n = paths.len();
            enum_manifest_pattern(true, &mut paths, &mut patpath, member .components(), member, &mut metadata.diagnostics);
            if n == paths.len() { metadata.diagnostics.push(Diagnostic {
                path:       Some(PathBuf::from(path.as_ref())),
                message:    format!("member pattern {:?} added no packages", member),
                kind:       DiagKind::Malformed,
            })}
        }

        for exclude in metadata.workspace.exclude.iter() {
            let n = paths.len();
            enum_manifest_pattern(false, &mut paths, &mut patpath, exclude.components(), exclude, &mut metadata.diagnostics);
            if n == paths.len() { metadata.diagnostics.push(Diagnostic {
                path:       Some(PathBuf::from(path.as_ref())),
                message:    format!("exclude pattern {:?} removed no packages", exclude),
                kind:       DiagKind::Warning,
            })}
        }

        for path in paths.into_iter() { metadata.load_pkg(path) }

        metadata
    }

    fn from_file_workspace_package(path: impl AsRef<Path> + Into<PathBuf>, ws: toml::Workspace, _pkg: toml::Cargo<toml::Package, ()>) -> Self {
        let mut metadata = Self::from_file_workspace(path.as_ref(), ws);
        metadata.set_active(path);
        metadata
    }

    fn load_pkg(&mut self, path: PathBuf) {
        debug_assert!(path.is_absolute());
        debug_assert!(!self.packages.by_path.contains_key(&path));
        macro_rules! bail { ($msg:expr, $kind:expr) => { return self.diagnostics.push(Diagnostic{ path: Some(path), message: $msg.into(), kind: $kind })}; }

        let bytes               = match std::fs::read(&path)            { Ok(b) => b, Err(err) => bail!("unable to read manifest file", DiagKind::Io(err)) };
        let cargo : toml::Cargo = match ::toml::from_slice(&bytes[..])  { Ok(c) => c, Err(err) => bail!("unable to parse manifest file", DiagKind::Toml(err)) };
        let (cargo, _ws) = cargo.take_workspace(); // TODO: Validate this matches the current workspace
        let pkg                 = match cargo.with_package()            { Some(p) => p, None => bail!("missing [package] in manifest", DiagKind::Malformed) };

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
        debug_assert!(self.packages.active.is_none());
        let active = self.packages.by_path.get(path).and_then(|i| NonMaxUsize::new(*i));
        self.packages.active = active;
        if active.is_none() { self.diagnostics.push(Diagnostic{
            path:       Some(PathBuf::from(path)),
            message:    format!("is expected to be the active project, but it is not part of the workspace"),
            kind:       DiagKind::Malformed,
        })}
    }
}




#[cfg(test)] mod tests {
    use super::*;

    #[test] fn deserialize() {
        let cwd = std::env::current_dir().unwrap();
        let meta = Metadata::from_current_dir().unwrap();

        assert_eq!(meta.workspace.directory, cwd);
        assert!(meta.workspace.members.iter().any(|m| m == Path::new(".")), "meta.workspace.members: {:#?}",    meta.workspace.members);
        assert!(meta.workspace.exclude.len() > 0,                           "meta.workspace.exclude: {:#?}",    meta.workspace.exclude);
        assert!(meta.packages.len() >= 2,                                   "meta.packages: {:#?}",             meta.packages);

        let mmrbi = meta.packages.get("mmrbi").expect("meta.packages.get(\"mmrbi\")");
        assert_eq!(mmrbi.path,                  cwd.join("Cargo.toml"));
        assert_eq!(mmrbi.package.name,          "mmrbi");
        assert_eq!(mmrbi.package.repository,    Some(String::from("https://github.com/MaulingMonkey/mmrbi.git")));
        assert_eq!(mmrbi.package.documentation, Some(String::from("https://docs.rs/mmrbi/")));
        assert_eq!(mmrbi.package.edition,       Some(String::from("2018")));
        assert!(mmrbi.package.authors.iter().any(|a| a == "MaulingMonkey <git@maulingmonkey.com>"),    "mmrbi.package.authors: {:#?}", mmrbi.package.authors);
        // license, readme, description, keywords, categories, ...

        let script = meta.packages.get("examples-script").expect("meta.packages.get(\"examples-script\")");
        assert_eq!(script.path,                 cwd.join("examples").join("script").join("Cargo.toml"));
        assert_eq!(script.package.name,         "examples-script");
        assert_eq!(script.package.publish,      toml::Publish::Enabled(false));
    }
}
