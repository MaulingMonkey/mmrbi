use std::path::{Path, PathBuf};

/// Utility methods for [std::path::Path](std::path::Path)\[[Buf](std::path::PathBuf)\]
pub trait PathExt : AsRef<Path> {
    /// Cleanup/simplify path as much as possible
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::path::Path;
    /// use mmrbi::PathExt;
    ///
    /// assert_eq!(Path::new("a/b").cleanup(),                      Path::new("a/b"));
    /// assert_eq!(Path::new("a/b/..").cleanup(),                   Path::new("a"));
    /// assert_eq!(Path::new("a/b/../..").cleanup(),                Path::new("."));
    /// assert_eq!(Path::new("a/b/../../..").cleanup(),             Path::new(".."));
    /// assert_eq!(Path::new("a/b/../../../..").cleanup(),          Path::new("../.."));
    ///
    /// assert_eq!(Path::new("../../a/b").cleanup(),                Path::new("../../a/b"));
    /// assert_eq!(Path::new("../../a/b/..").cleanup(),             Path::new("../../a"));
    /// assert_eq!(Path::new("../../a/b/../..").cleanup(),          Path::new("../.."));
    /// assert_eq!(Path::new("../../a/b/../../..").cleanup(),       Path::new("../../.."));
    /// assert_eq!(Path::new("../../a/b/../../../..").cleanup(),    Path::new("../../../.."));
    ///
    /// if cfg!(windows) {
    ///     assert_eq!(Path::new(r"C:\foo\bar").cleanup(),          Path::new(r"C:\foo\bar"));
    ///     assert_eq!(Path::new(r"\\?\C:\foo\bar").cleanup(),      Path::new(r"C:\foo\bar"));
    /// }
    /// ```
    fn cleanup(&self) -> PathBuf { crate::path::cleanup(self) }
}

impl<P: AsRef<Path>> PathExt for P {}
