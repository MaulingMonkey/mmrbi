//! Wrappers to manipulate [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

use crate::{Command, CommandExt};

use std::io;



/// Parse `wasm-pack --version`
///
/// # Examples
///
/// ```rust
/// # use mmrbi::wasm_pack;
/// let v = wasm_pack::version().unwrap();
/// assert_eq!(v.tool_name, "wasm-pack");
/// ```
#[cfg(feature = "version")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "version")))]
pub fn version() -> io::Result<crate::Version> {
    Command::new("wasm-pack").arg("--version").stdout0()?.parse()
}

/// Install `wasm-pack` and friends if `wasm-pack --version` < `v`
///
/// # Examples
///
/// ```rust,no_run
/// # use mmrbi::wasm_pack;
/// wasm_pack::install_at_least("0.9.1").unwrap();
/// ```
#[cfg(feature = "version")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "version")))]
pub fn install_at_least(v: &str) -> io::Result<()> {
    let v = semver::Version::parse(v).map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    if let Ok(installed) = version() {
        if v >= installed.version { return Ok(()); }
    }
    Command::new("cargo").arg("install").arg("--version").arg(format!("^{}", v)).arg("wasm-pack").status0()
}
