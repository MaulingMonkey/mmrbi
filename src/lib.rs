//! MaulingMonkey's Rust Build Infrastructure

mod macros;

#[path="cargo/_cargo.rs"] pub mod cargo;
#[doc(hidden)] pub mod log; // macro implementation details

mod command_ext;    pub use command_ext::CommandExt;
mod command;        pub use command::Command;
pub mod env;
pub mod fs;
mod path_ext;       pub use path_ext::PathExt;
pub mod path;
pub mod rustc;
pub mod rustup;     pub use rustup::Rustup;
mod result_ext;     pub use result_ext::ResultExt;
mod version;        pub use version::Version;
