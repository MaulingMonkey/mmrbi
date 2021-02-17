//! MaulingMonkey's Rust Build Infrastructure

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(not(feature = "all"), allow(unused_imports))]


mod macros;

#[path="cargo/_cargo.rs"] pub mod cargo;
#[doc(hidden)] pub mod log; // macro implementation details

pub mod cargo_web;
mod command_ext;    pub use command_ext::CommandExt;
mod command;        pub use command::Command;
pub mod env;
pub mod fs;
mod path_ext;       pub use path_ext::PathExt;
pub mod path;
pub mod rustc;
pub mod rustup;     pub use rustup::Rustup;
mod result_ext;     pub use result_ext::ResultExt;
mod version;        #[cfg(feature = "version")] pub use version::Version;
pub mod vscode;
pub mod wasm_bindgen;
pub mod wasm_pack;
