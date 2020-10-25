//! Similar to [std::path::*](std::path)

use std::path::{Component, Path, PathBuf, Prefix};

/// Cleanup/simplify path as much as possible
///
/// # Examples
///
/// ```rust
/// # use std::path::Path;
/// # use mmrbi::path::cleanup;
/// assert_eq!(cleanup("a/b"),                      Path::new("a/b"));
/// assert_eq!(cleanup("a/b/.."),                   Path::new("a"));
/// assert_eq!(cleanup("a/b/../.."),                Path::new("."));
/// assert_eq!(cleanup("a/b/../../.."),             Path::new(".."));
/// assert_eq!(cleanup("a/b/../../../.."),          Path::new("../.."));
///
/// assert_eq!(cleanup("../../a/b"),                Path::new("../../a/b"));
/// assert_eq!(cleanup("../../a/b/.."),             Path::new("../../a"));
/// assert_eq!(cleanup("../../a/b/../.."),          Path::new("../.."));
/// assert_eq!(cleanup("../../a/b/../../.."),       Path::new("../../.."));
/// assert_eq!(cleanup("../../a/b/../../../.."),    Path::new("../../../.."));
/// ```
pub fn cleanup(path: impl AsRef<Path>) -> PathBuf {
    let mut p = PathBuf::new();
    for c in path.as_ref().components() {
        match c {
            Component::Prefix(pre) => match pre.kind() {
                Prefix::VerbatimDisk(disk) => {
                    p.clear();
                    p.push(format!("{}:", char::from(disk)));
                },
                _other => {
                    p.clear();
                    p.push(c);
                }
            },
            c @ Component::RootDir => {
                p.clear();
                p.push(c);
            },
            Component::CurDir => {},
            Component::ParentDir => {
                if p.ends_with("..") || !p.pop() {
                    p.push("..");
                }
            },
            Component::Normal(c) => {
                p.push(c);
            },
        }
    }
    if p == Path::new("") {
        p.push(".");
    }
    p
}
