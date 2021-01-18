//! Wrappers to manipulate [`rustc`](https://doc.rust-lang.org/rustc/command-line-arguments.html).

use crate::{Command, CommandExt};

/// Parse `rustc --version`
///
/// # Examples
///
/// ```rust
/// # use mmrbi::rustc;
/// let v = rustc::version().unwrap();
/// assert_eq!(v.tool_name, "rustc");
/// ```
#[cfg(feature = "version")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "version")))]
pub fn version() -> std::io::Result<crate::Version> {
    Command::new("rustc").arg("--version").stdout0()?.parse()
}
