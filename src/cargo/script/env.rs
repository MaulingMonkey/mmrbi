use crate::{env, ResultExt};

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::path::PathBuf;



/// [Environment variables Cargo sets for build scripts](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)
#[derive(Clone, Debug)]
pub struct Env {
    /// Path to the `cargo` binary performing the build.
    pub cargo:                          PathBuf,

    /// The directory containing the manifest for the package being built (the package containing the build script).
    /// Also note that this is the value of the current working directory of the build script when it starts.
    pub cargo_manifest_dir:             PathBuf,

    /// The manifest `links` value.
    pub cargo_manifest_links:           Option<OsString>,

    /// Set on [unix-like platforms](https://doc.rust-lang.org/reference/conditional-compilation.html#unix-and-windows).
    pub cargo_cfg_unix:                 bool,

    /// Set on [windows-like platforms](https://doc.rust-lang.org/reference/conditional-compilation.html#unix-and-windows).
    pub cargo_cfg_windows:              bool,

    /// The [target family](https://doc.rust-lang.org/reference/conditional-compilation.html#target_family)
    /// (windows, unix)
    pub cargo_cfg_target_family:        String,

    /// The [target operating system](https://doc.rust-lang.org/reference/conditional-compilation.html#target_os)
    /// (windows, macos, ios, linux, android, freebsd, dragonfly, openbsd, netbsd, ...)
    pub cargo_cfg_target_os:            String,

    /// The CPU [target architecture](https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch)
    /// (x86, x86_64, mips, powerpc, powerpc64, arm, aarch64, ...)
    pub cargo_cfg_target_arch:          String,

    /// The [target vendor](https://doc.rust-lang.org/reference/conditional-compilation.html#target_vendor)
    /// (apple, fortanix, pc, unknown, ...)
    pub cargo_cfg_target_vendor:        String,

    /// The [target environment](https://doc.rust-lang.org/reference/conditional-compilation.html#target_env) ABI
    /// (blank, gnu, msvc, musl, sgx, ...)
    pub cargo_cfg_target_env:           String,

    /// The CPU [pointer width](https://doc.rust-lang.org/reference/conditional-compilation.html#target_pointer_width)
    /// (16, 32, 64, ...)
    pub cargo_cfg_target_pointer_width: String, // XXX

    /// The CPU [target endianness](https://doc.rust-lang.org/reference/conditional-compilation.html#target_endian)
    /// (little, big, ...)
    pub cargo_cfg_target_endian:        String,

    /// List of CPU [target features](https://doc.rust-lang.org/reference/conditional-compilation.html#target_feature) enabled
    /// (avx, avx2, crt-static, rdrand, sse, sse2, sse4.1, ...)
    pub cargo_cfg_target_features:      BTreeSet<String>,

    /// The folder in which all output should be placed
    pub out_dir:                        PathBuf,

    /// The [target triple](https://doc.rust-lang.org/cargo/appendix/glossary.html#target) that is being compiled for
    /// (x86_64-pc-windows-msvc, ...)
    pub target:                         String,

    /// The [host triple](https://doc.rust-lang.org/cargo/appendix/glossary.html#target) of the rust compiler / build scripts
    /// (x86_64-pc-windows-msvc, ...)
    pub host:                           String,
    
    /// The parallelism specified as the top-level parallelism.
    pub num_jobs:                       String, // XXX

    /// Values of the corresponding variables for the profile currently being built
    /// (DEBUG, ...)
    pub opt_level:                      String, // ???

    /// `release` for release builds, `debug` for debug builds, custom profiles may add more values
    pub profile:                        String,

    /// The compiler cargo has resolved to use
    pub rustc:                          PathBuf,

    /// The documentation generator cargo has resolved to use
    pub rustdoc:                        Option<PathBuf>,

    /// The linker binary that Cargo has resolved to use for the current target, **if specified**
    pub rustc_linker:                   Option<PathBuf>,
}

impl Env {
    pub fn get() -> Result<Self, crate::env::Error> {
        use env::*;
        Ok(Self {
            cargo:                          var_path("CARGO")?,
            cargo_manifest_dir:             var_path("CARGO_MANIFEST_DIR")?,
            cargo_manifest_links:           opt_var_os("CARGO_MANIFEST_LINKS"),

            cargo_cfg_unix:                 has_var("CARGO_CFG_UNIX"),
            cargo_cfg_windows:              has_var("CARGO_CFG_WINDOWS"),
            cargo_cfg_target_family:        var_str("CARGO_CFG_TARGET_FAMILY")?,
            cargo_cfg_target_os:            var_str("CARGO_CFG_TARGET_OS")?,
            cargo_cfg_target_arch:          var_str("CARGO_CFG_TARGET_ARCH")?,
            cargo_cfg_target_vendor:        var_str("CARGO_CFG_TARGET_VENDOR")?,
            cargo_cfg_target_env:           opt_var_str("CARGO_CFG_TARGET_ENV")?.unwrap_or(String::new()),
            cargo_cfg_target_pointer_width: var_str("CARGO_CFG_TARGET_POINTER_WIDTH")?,
            cargo_cfg_target_endian:        var_str("CARGO_CFG_TARGET_ENDIAN")?,
            cargo_cfg_target_features:      var_str("CARGO_CFG_TARGET_FEATURE")?.split(',').map(String::from).collect(),

            out_dir:                        var_path("OUT_DIR")?,
            target:                         var_str("TARGET")?,
            host:                           var_str("HOST")?,
            num_jobs:                       var_str("NUM_JOBS")?,
            opt_level:                      var_str("OPT_LEVEL")?,
            profile:                        var_str("PROFILE")?,
            rustc:                          var_path("RUSTC")?,
            rustdoc:                        opt_var_path("RUSTDOC"),
            rustc_linker:                   opt_var_path("RUSTC_LINKER"),
        })
    }
}

impl Env {
    /// Is the given feature activated for the package being built?
    pub fn cargo_feature(&self, name: impl AsRef<str>) -> bool {
        let mut name = name.as_ref().replace('-', "_");
        name.make_ascii_uppercase();
        let name = format!("CARGO_FEATURE_{}", name);
        env::has_var(name)
    }

    pub fn cargo_cfg(&self, name: impl AsRef<str>) -> bool {
        let mut name = name.as_ref().replace('-', "_");
        name.make_ascii_uppercase();
        let name = format!("CARGO_CFG_{}", name);
        env::has_var(name)
    }

    pub fn cargo_cfg_str(&self, name: impl AsRef<str>) -> Option<String> {
        let mut name = name.as_ref().replace('-', "_");
        name.make_ascii_uppercase();
        let name = format!("CARGO_CFG_{}", name);
        env::opt_var_str(name).or_die() // XXX
    }

    pub fn cargo_cfg_vec_str(&self, name: impl AsRef<str>) -> Vec<String> {
        let mut name = name.as_ref().replace('-', "_");
        name.make_ascii_uppercase();
        let name = format!("CARGO_CFG_{}", name);
        env::opt_var_str(name).or_die().map_or(Vec::new(), |s| s.split(',').map(String::from).collect()) // XXX
    }

    pub fn dep(&self, name: impl AsRef<str>, key: impl AsRef<str>) -> Option<String> {
        let name = format!("DEP_{}_{}", name.as_ref(), key.as_ref());
        env::opt_var_str(name).or_die() // XXX
    }
}
