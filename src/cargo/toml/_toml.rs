//! serde structures for parsing [The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
//!
//! Why this over [cargo_metadata]?
//!
//! 1.  `cargo metadata` requires a valid workspace - missing crates etc. kill the whole command
//! 2.  `metadata` fields in [cargo_metadata] are just tables, I want them parameterized for easier usage
//! 3.  I also want spans and non-fatal errors for better error reporting and stuff
//!
//! [cargo_metadata]:       https://docs.rs/cargo_metadata/

mod cargo;          pub use cargo::*;
mod package;        pub use package::{Package, Publish};
mod target;         pub use target::Target;
mod workspace;      pub use workspace::Workspace;
