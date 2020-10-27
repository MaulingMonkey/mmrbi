//! Wrappers to manipulate [`rustc`](https://doc.rust-lang.org/rustc/command-line-arguments.html).

use crate::{Command, CommandExt, Version};

/// Parse `rustc --version`
///
/// # Examples
///
/// ```rust
/// # use mmrbi::rustc;
/// let v = rustc::version().unwrap();
/// assert_eq!(v.tool_name, "rustc");
/// ```
pub fn version() -> std::io::Result<Version> {
    Command::new("rustc").arg("--version").stdout0()?.parse()
}
