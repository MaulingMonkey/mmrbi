//! Similar to [std::env::*](mod@std::env), but optimized for better error messages.
//!
//! | Prefix        | Set           | Unset     | Set (Invalid Unicode) |
//! | ------------- | ------------- | --------- | --------------------- |
//! | var_str       | Ok(value)     | Err       | Err
//! | var_lossy     | Ok(value)     | Err       | Ok(**lossy**)
//! | var_os        | Ok(value)     | Err       | Ok(value)
//! | var_path      | Ok(value)     | Err       | Ok(value)
//! | req_var_str   | value         | <span style="color: red; font-weight: bold">exit</span>      | <span style="color: red; font-weight: bold">exit</span>
//! | req_var_lossy | value         | <span style="color: red; font-weight: bold">exit</span>      | **lossy**
//! | req_var_os    | value         | <span style="color: red; font-weight: bold">exit</span>      | value
//! | req_var_path  | value         | <span style="color: red; font-weight: bold">exit</span>      | value
//! | opt_var_str   | Some(value)   | None      | <span style="color: red; font-weight: bold">exit</span>
//! | opt_var_lossy | Some(value)   | None      | Some(**lossy**)
//! | opt_var_os    | Some(value)   | None      | Some(value)
//! | opt_var_path  | Some(value)   | None      | Some(value)

use crate::*;

use std::ffi::{OsStr, OsString};
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};



pub type Result<T> = std::result::Result<T, Error>;

/// Contextual env var error.  Examples:
/// <code style="display: block; padding: 0.25em; margin: 0.5em 0;">%NONEXISTANT% is not set   <span style="color: #888">(windows)</span>
/// ${NONEXISTANT} is not set  <span style="color: #888">(linux)</span></code>

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    NotSet(OsString),
    InvalidUnicode(OsString),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fn display<'a>(var: &'a OsStr) -> impl Display + 'a { Path::new(var).display() }

        if cfg!(windows) {
            match self {
                Error::NotSet(var)          => write!(fmt, "%{}% is not set",               display(var)),
                Error::InvalidUnicode(var)  => write!(fmt, "%{}% contains invalid unicode", display(var)),
            }
        } else {
            match self {
                Error::NotSet(var)          => write!(fmt, "${{{}}} is not set",               display(var)),
                Error::InvalidUnicode(var)  => write!(fmt, "${{{}}} contains invalid unicode", display(var)),
            }
        }
    }
}



pub fn var_str(name: impl AsRef<OsStr> + Into<OsString>) -> Result<String> {
    match std::env::var(name.as_ref()) {
        Ok(v) => Ok(v),
        Err(std::env::VarError::NotPresent)     => Err(Error::NotSet(name.into())),
        Err(std::env::VarError::NotUnicode(_))  => Err(Error::InvalidUnicode(name.into())),
    }
}

pub fn var_lossy(name: impl AsRef<OsStr> + Into<OsString>) -> Result<String> {
    match std::env::var_os(name.as_ref()) {
        Some(v) => Ok(into_string_lossy(v)),
        None    => Err(Error::NotSet(name.into())),
    }
}

pub fn var_os(name: impl AsRef<OsStr> + Into<OsString>) -> Result<OsString> {
    match std::env::var_os(name.as_ref()) {
        Some(v) => Ok(v),
        None    => Err(Error::NotSet(name.into())),
    }
}

pub fn var_path(name: impl AsRef<OsStr> + Into<OsString>) -> Result<PathBuf> {
    match std::env::var_os(name.as_ref()) {
        Some(v) => Ok(PathBuf::from(v)),
        None    => Err(Error::NotSet(name.into())),
    }
}



pub fn req_var_str  (name: impl AsRef<OsStr> + Into<OsString>) -> String    { var_str(name).or_die() }
pub fn req_var_lossy(name: impl AsRef<OsStr> + Into<OsString>) -> String    { var_lossy(name).or_die() }
pub fn req_var_os   (name: impl AsRef<OsStr> + Into<OsString>) -> OsString  { var_os(name).or_die() }
pub fn req_var_path (name: impl AsRef<OsStr> + Into<OsString>) -> PathBuf   { var_path(name).or_die() }



pub fn opt_var_str(name: impl AsRef<OsStr> + Into<OsString>) -> Option<String> {
    match std::env::var(name.as_ref()) {
        Ok(v) => Some(v),
        Err(std::env::VarError::NotPresent)     => None,
        Err(std::env::VarError::NotUnicode(_))  => fatal!("{}", Error::InvalidUnicode(name.into())),
    }
}

pub fn opt_var_lossy(name: impl AsRef<OsStr> + Into<OsString>) -> Option<String> {
    std::env::var_os(name.as_ref()).map(into_string_lossy)
}

pub fn opt_var_os(name: impl AsRef<OsStr> + Into<OsString>) -> Option<OsString> {
    std::env::var_os(name.as_ref())
}

pub fn opt_var_path(name: impl AsRef<OsStr> + Into<OsString>) -> Option<PathBuf> {
    std::env::var_os(name.as_ref()).map(PathBuf::from)
}



fn into_string_lossy(os: OsString) -> String {
    // Optimized for the common case where OsString is valid UTF8
    os.into_string().map_or_else(
        |os| os.to_string_lossy().into_owned(), // could be optimized to not revalidate early unicode
        |s| s,
    )
}
