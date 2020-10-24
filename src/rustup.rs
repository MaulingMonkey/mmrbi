//! Wrappers to manipulate [`rustup`](https://rustup.rs/)
//!
//! # Examples
//!
//! ```rust
//! # use mmrbi::*;
//! let rustup = Rustup::default().or_die();
//!
//! // should only fail if rustup was moved/deleted after Rustup was constructed
//! assert!(rustup.is_available());
//!
//! // If you're running these tests, surely you have a toolchain
//! assert!(rustup.toolchains().active().is_some());
//! assert!(rustup.toolchains().default().is_some());
//! assert!(rustup.toolchains().installed().len() > 0);
//! assert!(rustup.toolchains().get("nonexistant").is_none());
//!
//! let active = rustup.toolchains().active().unwrap();
//! assert!(rustup.toolchains().get(active.as_str()).is_some());
//! assert!(active.targets().installed().len() > 0);
//! assert!(active.targets().all().len() > active.targets().installed().len());
//! active.cargo().arg("--version").status0().unwrap();
//! active.rustc().arg("--version").status0().unwrap();
//! ```

// TODO: WSL support

use crate::{Command, CommandExt, ResultExt};

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ffi::*;
use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io;
use std::sync::Arc;



/// Wrapper around a `rustup` executable that existed at least at one point.
pub struct Rustup { rustup: Arc<OsString> }

impl Rustup {
    /// Get `rustup` from `${PATH}`, if available.
    ///
    /// Returns `Err(...)` if `rustup --version` fails.
    pub fn default() -> io::Result<Self> { Self::new("rustup") }

    /// Get `rustup` from a specific path, if available.
    ///
    /// Returns `Err(...)` if `{rustup} --version` fails.
    pub fn new(rustup: impl AsRef<OsStr> + Into<OsString>) -> io::Result<Self> {
        let rustup = rustup.into();
        Command::new(&rustup).arg("--version").status0()?;
        Ok(Self { rustup: Arc::new(rustup.into()) })
    }

    /// Get `rustup` from a specific path
    pub fn new_unchecked(rustup: impl AsRef<OsStr> + Into<OsString>) -> Self {
        let rustup = rustup.into();
        Self { rustup: Arc::new(rustup.into()) }
    }

    /// Returns `true` if `rustup --version` still succeeds
    pub fn is_available(&self) -> bool {
        Command::new(self.rustup.as_os_str()).arg("--version").status().map_or(false, |c| c.code() == Some(0))
    }

    /// Toolchains rustup is aware of
    pub fn toolchains(&self) -> RustupToolchains {
        RustupToolchains { rustup: &self.rustup }
    }
}



/// Result of `rustup.toolchains()`
pub struct RustupToolchains<'r> { rustup: &'r Arc<OsString> }

impl<'r> RustupToolchains<'r> {
    /// Gets the default toolchain by parsing `rustup show active-toolchain`
    pub fn active(&self) -> Option<Toolchain> {
        let o = self.rustup(&["show", "active-toolchain"]).stdout0_no_stderr().ok()?;
        // stable-x86_64-pc-windows-msvc (default)
        let o = o.trim().split(' ').next()?;
        Some(Toolchain { rustup: self.rustup.clone(), toolchain: Arc::new(o.into()) })
    }

    /// Gets the default toolchain by parsing `rustup default`
    pub fn default(&self) -> Option<Toolchain> {
        let o = self.rustup(&["default"]).stdout0_no_stderr().or_die();
        let o = o.trim().split(' ').next()?;
        Some(Toolchain { rustup: self.rustup.clone(), toolchain: Arc::new(o.into()) })
    }

    /// Gets all installed toolchains by parsing `rustup toolchain list`
    pub fn installed(&self) -> BTreeSet<Toolchain> {
        let o = self.rustup(&["toolchain", "list"]).stdout0().or_die();
        let mut r = BTreeSet::new();
        for line in o.lines() {
            let line = line.trim().split(' ').next().unwrap_or("");
            if line.is_empty() { continue }
            r.extend(Some(Toolchain { rustup: self.rustup.clone(), toolchain: Arc::new(line.into()) }));
        }
        r
    }

    /// Gets an installed toolchain by parsing `rustup +{toolchain} show active-toolchain`
    pub fn get(&self, toolchain: impl AsRef<str>) -> Option<Toolchain> {
        let toolchain = toolchain.as_ref();
        // Despite https://rust-lang.github.io/rustup/overrides.html implying +nonexistant should take priority over
        // ${RUSTUP_TOOLCHAIN}, it looks like we need to clear RUSTUP_TOOLCHAIN on `rustup 1.22.1 (b01adbbc3 2020-07-08)`
        // for accurate failures
        let o = self.rustup(&[&format!("+{}", toolchain), "show", "active-toolchain"]).env_remove("RUSTUP_TOOLCHAIN").stdout0_no_stderr().ok()?;
        // stable-x86_64-pc-windows-msvc (overridden by +toolchain on the command line)
        let o = o.trim().split(' ').next()?;
        Some(Toolchain { rustup: self.rustup.clone(), toolchain: Arc::new(o.into()) })
    }

    /// Install a toolchain via `rustup toolchain install ${toolchain}`
    pub fn install(&self, toolchain: impl AsRef<str>) -> io::Result<Toolchain> {
        // TODO: --allow-downgrade, --force, --no-self-update, --component, --profile, --target
        self.rustup(&["toolchain", "install", toolchain.as_ref()]).stdout0()?;
        self.get(toolchain.as_ref()).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("unable to find toolchain {}", toolchain.as_ref())))
    }

    /// Uninstall a toolchain via `rustup toolchain install ${toolchain}`
    pub fn uninstall(&self, toolchain: impl AsRef<str>) -> io::Result<()> {
        self.rustup(&["toolchain", "install", toolchain.as_ref()]).status0()
    }

    fn rustup<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(&self, args: I) -> Command {
        let mut c = Command::new(self.rustup.as_os_str());
        c.args(args);
        c
    }
}



/// A rustup-installed rust toolchain
#[derive(Clone, Debug)] pub struct Toolchain { rustup: Arc<OsString>, toolchain: Arc<String> }

impl AsRef<str>     for Toolchain { fn as_ref(&self) -> &str { &self.toolchain } }
impl Borrow<str>    for Toolchain { fn borrow(&self) -> &str { &self.toolchain } }
impl Display        for Toolchain { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.toolchain, fmt) } }
impl Eq             for Toolchain {}
impl Hash           for Toolchain { fn hash<H: Hasher>(&self, state: &mut H) { self.toolchain.hash(state) } }
impl Ord            for Toolchain { fn cmp(&self, other: &Self) -> Ordering { self.toolchain.cmp(&other.toolchain) } }
impl PartialEq      for Toolchain { fn eq(&self, other: &Self) -> bool { self.toolchain == other.toolchain } }
impl PartialOrd     for Toolchain { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.toolchain.partial_cmp(&other.toolchain) } }

impl Toolchain {
    /// The entire toolchain name as a string
    pub fn as_str(&self) -> &str { &self.toolchain }

    /// [Target]s for this toolchain
    pub fn targets(&self) -> ToolchainTargets { ToolchainTargets { toolchain: self } }

    /// Creates a [Command] to run cargo for a specific rustup toolchain
    pub fn cargo(&self) -> Command { self.run("cargo") }

    /// Creates a [Command] to run rustc for a specific rustup toolchain
    pub fn rustc(&self) -> Command { self.run("rustc") }

    fn run(&self, command: &str) -> Command {
        let mut c = Command::new(self.rustup.as_os_str());
        c.arg("run");
        c.arg(self.toolchain.as_str());
        c.arg(command);
        c
    }
}



/// Result of `toolchain.targets()`
pub struct ToolchainTargets<'t> { toolchain: &'t Toolchain }

impl<'t> ToolchainTargets<'t> {
    /// `rustup target list --toolchain {toolchain}` - gets all targets known by this toolchain, installed or not
    pub fn all(&self) -> BTreeSet<Target> {
        let o = self.rustup(&["target", "list"]).stdout0().or_die();
        let mut r = BTreeSet::new();
        for line in o.lines() {
            let line = line.trim().split(' ').next().unwrap_or("");
            if line.is_empty() { continue }
            r.extend(Some(Target(line.into())));
        }
        r
    }

    /// `rustup target list --installed --toolchain {toolchain}` - gets all installed targets for this toolchain
    pub fn installed(&self) -> BTreeSet<Target> {
        let o = self.rustup(&["target", "list", "--installed"]).stdout0().or_die();
        let mut r = BTreeSet::new();
        for line in o.lines() {
            let line = line.trim().split(' ').next().unwrap_or("");
            if line.is_empty() { continue }
            r.extend(Some(Target(line.into())));
        }
        r
    }

    /// Verifies target exists within `rustup target list --installed --toolchain {toolchain}`
    pub fn get(&self, target: impl AsRef<str>) -> Option<Target> {
        self.installed().take(target.as_ref())
    }

    /// `rustup target add {target} --toolchain {toolchain}` - adds a target
    pub fn add(&self, target: impl AsRef<str>) -> io::Result<()> {
        self.rustup(&["target", "add", target.as_ref()]).status0()
    }

    /// `rustup target remove {target} --toolchain {toolchain}` - removes a target
    pub fn remove(&self, target: impl AsRef<str>) -> io::Result<()> {
        self.rustup(&["target", "remove", target.as_ref()]).status0()
    }

    fn rustup<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(&self, args: I) -> Command {
        let mut c = Command::new(self.toolchain.rustup.as_os_str());
        c.args(args);
        c.arg("--toolchain");
        c.arg(self.toolchain.toolchain.as_str());
        c
    }
}



/// A `rustup` (and `cargo`/`rustc`) target
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Target(String);

impl AsRef<str>   for Target { fn as_ref(&self) -> &str { &self.0 } }
impl Borrow<str>  for Target { fn borrow(&self) -> &str { &self.0 } }
//impl From<String> for Target { fn from(value: String ) -> Self { Self(value) } }
//impl From<&str>   for Target { fn from(value: &str   ) -> Self { Self(value.into()) } }
impl Display      for Target { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.0, fmt) } }

impl Target {
    /// Construct a new target from a known tuple
    pub fn new(src: impl Into<Target>) -> Self { src.into() }

    /// The entire target name as a string
    pub fn as_str(&self) -> &str { &self.0 }
}
