//! Wrappers to manipulate [Visual Studio Code](https://code.visualstudio.com/)

use crate::{Command, CommandExt};

use std::collections::BTreeSet;



/// Parse `code --list-extensions`
///
/// # Examples
///
/// ```rust
/// # use mmrbi::vscode;
/// # if std::env::var_os("CI").is_none() {
/// assert!( vscode::list_extensions().contains("ms-vscode.cpptools"));
/// assert!(!vscode::list_extensions().contains("nonexistent"));
/// # }
/// ```
pub fn list_extensions() -> BTreeSet<String> {
    if cfg!(windows) {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C").arg("call code --list-extensions");
        cmd
    } else {
        let mut cmd = Command::new("code");
        cmd.arg("--list-extensions");
        cmd
    }.stdout0().unwrap_or(String::new()).lines().map(String::from).collect::<BTreeSet<_>>()
}
