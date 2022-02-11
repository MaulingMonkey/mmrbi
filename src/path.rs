//! Similar to [std::path::*](std::path)

use std::ffi::OsStr;
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

/// Check the extension of a file
///
/// # Examples
///
/// ```rust
/// # use mmrbi::path::*;
/// // The full filename doesn't count as an extension
/// assert!(!has_extension("foo.tar.gz", ".foo.tar.gz"));
/// assert!(!has_extension("foo.tar.gz",  "foo.tar.gz"));
/// assert!(!has_extension("foo.tar.gz",   "oo.tar.gz"));
///
/// // Only the full extension (with or without a leading `.`) counts
/// assert!( has_extension("foo.tar.gz", ".tar.gz"));
/// assert!( has_extension("foo.tar.gz",  "tar.gz"));
/// assert!(!has_extension("foo.tar.gz",   "ar.gz"));
///
/// // Case sensitive
/// assert!(!has_extension("foo.tAr.gZ", ".tar.gz"));
/// assert!(!has_extension("foo.tAr.gZ",  "tar.gz"));
/// assert!(!has_extension("foo.tAr.gZ",   "ar.gz"));
///
/// assert!( has_extension("foo.tar.gz", ".gz"));
/// assert!( has_extension("foo.tar.gz",  "gz"));
/// assert!(!has_extension("foo.tar.gz",   "z"));
/// ```
pub fn has_extension(path: impl AsRef<Path>, ext: impl AsRef<OsStr>) -> bool {
    has_extension_impl(path, ext, |a,b| a.eq(b))
}

/// Check the extension of a file
///
/// # Examples
///
/// ```rust
/// # use mmrbi::path::*;
/// // The full filename doesn't count as an extension
/// assert!(!has_extension_ignore_ascii_case("foo.tar.gz", ".foo.tar.gz"));
/// assert!(!has_extension_ignore_ascii_case("foo.tar.gz",  "foo.tar.gz"));
/// assert!(!has_extension_ignore_ascii_case("foo.tar.gz",   "oo.tar.gz"));
///
/// // Only the full extension (with or without a leading `.`) counts
/// assert!( has_extension_ignore_ascii_case("foo.tar.gz", ".tar.gz"));
/// assert!( has_extension_ignore_ascii_case("foo.tar.gz",  "tar.gz"));
/// assert!(!has_extension_ignore_ascii_case("foo.tar.gz",   "ar.gz"));
///
/// // Case insensitive
/// assert!( has_extension_ignore_ascii_case("foo.tAr.gZ", ".tar.gz"));
/// assert!( has_extension_ignore_ascii_case("foo.tAr.gZ",  "tar.gz"));
/// assert!(!has_extension_ignore_ascii_case("foo.tAr.gZ",   "ar.gz"));
///
/// assert!( has_extension_ignore_ascii_case("foo.tar.gz", ".gz"));
/// assert!( has_extension_ignore_ascii_case("foo.tar.gz",  "gz"));
/// assert!(!has_extension_ignore_ascii_case("foo.tar.gz",   "z"));
/// ```
pub fn has_extension_ignore_ascii_case(path: impl AsRef<Path>, ext: impl AsRef<OsStr>) -> bool {
    has_extension_impl(path, ext, |a,b| a.eq_ignore_ascii_case(b))
}

fn has_extension_impl(path: impl AsRef<Path>, ext: impl AsRef<OsStr>, eq: impl FnOnce(&[u8], &[u8]) -> bool) -> bool {
    let ext = os_str_to_likely_wtf8_bytes(ext.as_ref());
    if ext.is_empty() { return true }
    let ext = ext.strip_prefix(b".").unwrap_or(ext);
    // ext = "tar.gz"

    let file_name = match path.as_ref().file_name() {
        None => return false,
        Some(p) => os_str_to_likely_wtf8_bytes(p),
    };
    // file_name = "foo.tar.gz"

    if file_name.len() <= ext.len() {
        // file_name is either smaller than ext, or doesn't have enough space for the leading `.`
        // (might be exactly equal to `ext`, but that's not the same thing as *having* `ext` as an extension IMO)
        false
    } else if let Some(file_ext) = file_name[file_name.len() - ext.len() - 1..].strip_prefix(b".") {
        eq(file_ext, ext)
    } else { // `ext` was not preceeded by `.`
        false
    }
}

#[cfg(windows)] fn is_os_str_to_bytes_transmute_safe_probably() -> bool {
    #[repr(C)] struct Sliceish { ptr: usize, len: usize }

    let astr = "a";
    let a = OsStr::new(astr);
    if a.len() != 1 { return false }
    // SAFETY:
    //  ✔️ all bit patterns of Sliceish should be valid, no invariants to violate
    //  ⚠️ if &OsStr ever has padding, this would be an unsound read of potentially uninitialized data
    let a : Sliceish = unsafe { std::mem::transmute(a) };
    if a.len != 1 { return false }
    if a.ptr != astr.as_ptr() as usize { return false }

    true
}

#[cfg(windows)] #[test] fn check_transmute_safe() {
    assert!(is_os_str_to_bytes_transmute_safe_probably());
}

fn os_str_to_likely_wtf8_bytes<'s>(os: &'s (impl AsRef<OsStr> + ?Sized)) -> &'s [u8] {
    return imp(os.as_ref());

    #[cfg(unix)]
    fn imp(os: &OsStr) -> &[u8] {
        use std::os::unix::ffi::*;
        os.as_bytes()
    }

    #[cfg(windows)]
    fn imp(os: &OsStr) -> &[u8] {
        assert!(is_os_str_to_bytes_transmute_safe_probably());
        // UNSAFETY: ❌ technically super unsound?
        unsafe { std::mem::transmute(os) }
    }
}
