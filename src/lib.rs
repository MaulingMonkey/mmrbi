//! MaulingMonkey's Rust Build Infrastructure

mod macros;

mod command_ext;    pub use command_ext::CommandExt;
mod command;        pub use command::Command;
pub mod env;
#[doc(hidden)] pub mod log; // macro implementation details
pub mod rustup;     pub use rustup::Rustup;
mod result_ext; pub use result_ext::ResultExt;
