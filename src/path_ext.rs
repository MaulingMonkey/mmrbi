use std::ffi::OsStr;
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

    /// Check the extension of a file
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::path::Path;
    /// use mmrbi::PathExt;
    ///
    /// // The full filename doesn't count as an extension
    /// assert!(!Path::new("foo.tar.gz").has_extension(".foo.tar.gz"));
    /// assert!(!Path::new("foo.tar.gz").has_extension( "foo.tar.gz"));
    /// assert!(!Path::new("foo.tar.gz").has_extension(  "oo.tar.gz"));
    ///
    /// // Only the full extension (with or without a leading `.`) counts
    /// assert!( Path::new("foo.tar.gz").has_extension(".tar.gz"));
    /// assert!( Path::new("foo.tar.gz").has_extension( "tar.gz"));
    /// assert!(!Path::new("foo.tar.gz").has_extension(  "ar.gz"));
    ///
    /// // Case sensitive
    /// assert!(!Path::new("foo.tAr.gZ").has_extension(".tar.gz"));
    /// assert!(!Path::new("foo.tAr.gZ").has_extension( "tar.gz"));
    /// assert!(!Path::new("foo.tAr.gZ").has_extension(  "ar.gz"));
    ///
    /// assert!( Path::new("foo.tar.gz").has_extension(".gz"));
    /// assert!( Path::new("foo.tar.gz").has_extension( "gz"));
    /// assert!(!Path::new("foo.tar.gz").has_extension(  "z"));
    /// ```
    fn has_extension(&self, ext: impl AsRef<OsStr>) -> bool { crate::path::has_extension(self, ext) }

    /// Check the extension of a file
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::path::Path;
    /// use mmrbi::PathExt;
    ///
    /// // The full filename doesn't count as an extension
    /// assert!(!Path::new("foo.tar.gz").has_extension_ignore_ascii_case(".foo.tar.gz"));
    /// assert!(!Path::new("foo.tar.gz").has_extension_ignore_ascii_case( "foo.tar.gz"));
    /// assert!(!Path::new("foo.tar.gz").has_extension_ignore_ascii_case(  "oo.tar.gz"));
    ///
    /// // Only the full extension (with or without a leading `.`) counts
    /// assert!( Path::new("foo.tar.gz").has_extension_ignore_ascii_case(".tar.gz"));
    /// assert!( Path::new("foo.tar.gz").has_extension_ignore_ascii_case( "tar.gz"));
    /// assert!(!Path::new("foo.tar.gz").has_extension_ignore_ascii_case(  "ar.gz"));
    ///
    /// // Case insensitive
    /// assert!( Path::new("foo.tAr.gZ").has_extension_ignore_ascii_case(".tar.gz"));
    /// assert!( Path::new("foo.tAr.gZ").has_extension_ignore_ascii_case( "tar.gz"));
    /// assert!(!Path::new("foo.tAr.gZ").has_extension_ignore_ascii_case(  "ar.gz"));
    ///
    /// assert!( Path::new("foo.tar.gz").has_extension_ignore_ascii_case(".gz"));
    /// assert!( Path::new("foo.tar.gz").has_extension_ignore_ascii_case( "gz"));
    /// assert!(!Path::new("foo.tar.gz").has_extension_ignore_ascii_case(  "z"));
    /// ```
    fn has_extension_ignore_ascii_case(&self, ext: impl AsRef<OsStr>) -> bool { crate::path::has_extension_ignore_ascii_case(self, ext) }
}

impl<P: AsRef<Path>> PathExt for P {}
