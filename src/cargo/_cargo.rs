//! Wrappers to manipulate [`cargo`] and [`Cargo.toml`] files.
//! 
//! [`cargo`]:          https://doc.rust-lang.org/cargo/
//! [`Cargo.toml`]:     https://doc.rust-lang.org/cargo/reference/manifest.html

#[path = "metadata/_metadata.rs"]   pub mod metadata; pub use metadata::Metadata;
#[path = "script/_script.rs"]       pub mod script;
#[path = "toml/_toml.rs"]           pub mod toml;

/// Parse `cargo --version`
///
/// # Examples
///
/// ```rust
/// # use mmrbi::cargo;
/// let v = cargo::version().unwrap();
/// assert_eq!(v.tool_name, "cargo");
/// ```
pub fn version() -> std::io::Result<crate::Version> {
    use crate::CommandExt;
    crate::Command::new("cargo").arg("--version").stdout0_no_stderr()?.parse()
}
