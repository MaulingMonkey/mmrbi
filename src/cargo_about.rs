//! Wrappers to manipulate [`cargo-about`](https://github.com/EmbarkStudios/cargo-about).

use crate::{Command, CommandExt};

use std::io;



/// Parse `cargo about --version`
///
/// # Examples
///
/// ```rust
/// # use mmrbi::cargo_about;
/// let v = cargo_about::version().unwrap();
/// assert_eq!(v.tool_name, "cargo-about");
/// ```
#[cfg(feature = "version")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "version")))]
pub fn version() -> io::Result<crate::Version> {
    Command::new("cargo").arg("about").arg("--version").stdout0()?.parse()
}

/// Install `cargo-about` and friends if `cargo about --version` < `requested`
///
/// # Examples
///
/// ```rust,no_run
/// # use mmrbi::cargo_about;
/// cargo_about::install_at_least("0.2.3").unwrap();
/// ```
#[cfg(feature = "version")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "version")))]
pub fn install_at_least(requested: &str) -> io::Result<()> {
    let requested = semver::Version::parse(requested).map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    if let Ok(installed) = version() {
        if requested >= installed.version { return Ok(()); }
    }
    Command::new("cargo").arg("install").arg("--version").arg(format!("^{}", requested)).arg("cargo-about").status0()
}
