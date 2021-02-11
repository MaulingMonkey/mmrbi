//! Wrappers to manipulate [`cargo-web`](https://github.com/koute/cargo-web).

use crate::{Command, CommandExt};

use std::io;



/// Parse `cargo-web --version`
///
/// # Examples
///
/// ```rust
/// # use mmrbi::cargo_web;
/// let v = cargo_web::version().unwrap();
/// assert_eq!(v.tool_name, "cargo-web");
/// ```
#[cfg(feature = "version")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "version")))]
pub fn version() -> io::Result<crate::Version> {
    Command::new("cargo-web").arg("--version").stdout0()?.parse()
}

/// Install `cargo-web` and friends if `cargo-web --version` < `v`
///
/// # Examples
///
/// ```rust,no_run
/// # use mmrbi::cargo_web;
/// cargo_web::install_at_least("0.6.26").unwrap();
/// ```
#[cfg(feature = "version")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "version")))]
pub fn install_at_least(v: &str) -> io::Result<()> {
    let v = semver::Version::parse(v).map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    if let Ok(installed) = version() {
        if v >= installed.version { return Ok(()); }
    }
    Command::new("cargo").arg("install").arg("--version").arg(format!("^{}", v)).arg("cargo-web-cli").status0()
}
