use std::path::{Path, PathBuf};

/// Utility methods for [std::path::Path](std::path::Path)\[[Buf](std::path::PathBuf)\]
pub trait PathExt {
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
    /// ```
    fn cleanup(&self) -> PathBuf;
}

impl PathExt for &Path      { fn cleanup(&self) -> PathBuf { crate::path::cleanup(self) } }
impl PathExt for PathBuf    { fn cleanup(&self) -> PathBuf { crate::path::cleanup(self) } }
