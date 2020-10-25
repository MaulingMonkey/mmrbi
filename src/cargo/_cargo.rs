//! Wrappers to manipulate [`cargo`] and [`Cargo.toml`] files.
//! 
//! [`cargo`]:          https://doc.rust-lang.org/cargo/
//! [`Cargo.toml`]:     https://doc.rust-lang.org/cargo/reference/manifest.html

#[path = "metadata/_metadata.rs"]   pub mod metadata; pub use metadata::Metadata;
#[path = "script/_script.rs"]       pub mod script;
#[path = "toml/_toml.rs"]           pub mod toml;
