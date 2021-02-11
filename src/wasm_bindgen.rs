//! Wrappers to manipulate [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen).

use crate::{Command, CommandExt};

use std::io;



/// Parse `wasm-bindgen --version`
///
/// # Examples
///
/// ```rust
/// # use mmrbi::wasm_bindgen;
/// let v = wasm_bindgen::version().unwrap();
/// assert_eq!(v.tool_name, "wasm-bindgen");
/// ```
#[cfg(feature = "version")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "version")))]
pub fn version() -> io::Result<crate::Version> {
    Command::new("wasm-bindgen").arg("--version").stdout0()?.parse()
}

/// Install `wasm-bindgen` and friends if `wasm-bindgen --version` < `v`
///
/// # Examples
///
/// ```rust,no_run
/// # use mmrbi::wasm_bindgen;
/// wasm_bindgen::install_at_least("0.2.70").unwrap();
/// ```
#[cfg(feature = "version")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "version")))]
pub fn install_at_least(v: &str) -> io::Result<()> {
    let v = semver::Version::parse(v).map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    if let Ok(installed) = version() {
        if v >= installed.version { return Ok(()); }
    }
    Command::new("cargo").arg("install").arg("--version").arg(format!("^{}", v)).arg("wasm-bindgen-cli").status0()
}
