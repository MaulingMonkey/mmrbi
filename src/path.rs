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
///
/// if cfg!(windows) {
///     assert_eq!(cleanup(r"C:\foo\bar"),          Path::new(r"C:\foo\bar"));
///     assert_eq!(cleanup(r"\\?\C:\foo\bar"),      Path::new(r"C:\foo\bar"));
/// }
/// ```
pub fn cleanup(path: impl AsRef<Path>) -> PathBuf {
    let mut p = PathBuf::new();
    let mut components = path.as_ref().components();
    while let Some(c) = components.next() {
        match c {
            Component::Prefix(pre) => match pre.kind() {
                Prefix::Disk(disk) | Prefix::VerbatimDisk(disk) => {
                    p.clear();
                    p.push(format!("{}:\\", char::from(disk)));
                    let _root = components.next();
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
