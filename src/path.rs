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
    has_extension_impl(path, ext, |a,b| a == b)
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
    has_extension_impl(path, ext, |a,b| a.eq_ignore_ascii_case(&b))
}

#[cfg(unix)]
fn has_extension_impl(path: impl AsRef<Path>, ext: impl AsRef<OsStr>, eq: impl FnOnce(&[u8], &[u8]) -> bool) -> bool {
    use std::os::unix::ffi::*;

    let ext = ext.as_ref().as_bytes();
    if ext.is_empty() { return true }
    let ext = ext.strip_prefix(b".").unwrap_or(ext);
    // ext = "tar.gz"

    let file_name = match path.as_ref().file_name() {
        None => return false,
        Some(p) => p.as_bytes(),
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

#[cfg(windows)]
fn has_extension_impl(path: impl AsRef<Path>, ext: impl AsRef<OsStr>, eq: impl Fn(u8, u8) -> bool) -> bool {
    use std::os::windows::ffi::*;

    let ext_os = ext.as_ref();
    if ext_os.is_empty() { return true }
    let mut ext = ext_os.encode_wide();
    if ext.next() != Some(b'.' as u16) { ext = ext_os.encode_wide() } // ~strip_prefix(b".")
    let ext_len = ext.clone().count();
    // ext = "tar.gz"

    let mut file_name = match path.as_ref().file_name() {
        None => return false,
        Some(p) => p.encode_wide(),
    };
    let file_name_len = file_name.clone().count();
    // file_name = "foo.tar.gz"

    if file_name_len <= ext_len {
        // file_name is either smaller than ext, or doesn't have enough space for the leading `.`
        // (might be exactly equal to `ext`, but that's not the same thing as *having* `ext` as an extension IMO)
        false
    } else {
        for _ in ext_len+1 .. file_name_len { let _ = file_name.next(); }
        if file_name.next() != Some(b'.' as u16) { return false }

        loop {
            match (file_name.next(), ext.next()) {
                (None, None) => return true,
                (Some(file_name_16), Some(ext_16)) => {
                    match (u8::try_from(file_name_16), u8::try_from(ext_16)) {
                        (Ok(file_name_8), Ok(ext_8)) => {
                            if !eq(file_name_8, ext_8) { return false }
                        },
                        (Err(_), Err(_)) => {
                            if file_name_16 != ext_16 { return false }
                        },
                        _mixed => return false,
                    }
                },
                _other => panic!("has_extension_impl bug: file_name and ext should've had the same length in loop"),
            }
        }
    }
}
