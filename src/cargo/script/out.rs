//! [Outputs of the Build Script](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script)

use std::ffi::OsStr;
use std::path::Path;



/// [`cargo:rerun-if-changed=PATH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
/// — Tells Cargo when to re-run the script.
pub fn rerun_if_changed(path: impl AsRef<Path>) {
    println!("cargo:rerun-if-changed={}", path.as_ref().display());
}

/// [`cargo:rerun-if-env-changed=VAR`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed)
/// — Tells Cargo when to re-run the script.
pub fn rerun_if_env_changed(var: impl AsRef<OsStr>) {
    println!("cargo:rerun-if-env-changed={}", Path::new(var.as_ref()).display());
}

/// [`cargo:rustc-link-lib=[KIND=]NAME`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib)
/// — Adds a library to link.
pub fn rustc_link_lib(kind: Option<&str>, name: impl AsRef<OsStr>) {
    match kind {
        Some(kind)  => println!("cargo:rustc-link-lib={kind}={name}", kind=kind, name=Path::new(name.as_ref()).display()),
        None        => println!("cargo:rustc-link-lib={name}", name=Path::new(name.as_ref()).display()),
    }
}

/// [`cargo:rustc-link-search=[KIND=]PATH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search)
/// — Adds to the library search path.
pub fn rustc_link_search(kind: Option<&str>, path: impl AsRef<Path>) {
    match kind {
        Some(kind)  => println!("cargo:rustc-link-search={kind}={path}", kind=kind, path=path.as_ref().display()),
        None        => println!("cargo:rustc-link-search={path}", path=path.as_ref().display()),
    }
}

/// [`cargo:rustc-flags=FLAGS`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-flags)
/// — Passes certain flags to the compiler.
pub fn rustc_flags(flags: impl AsRef<OsStr>) {
    println!("cargo:rustc-flags={}", Path::new(flags.as_ref()).display());
}

/// [`cargo:rustc-cfg=KEY`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg)
/// — Enables compile-time cfg settings.
pub fn rustc_cfg(key: impl AsRef<str>) {
    println!("cargo:rustc-cfg={}", key.as_ref());
}

/// [`cargo:rustc-cfg=KEY="VALUE"`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg)
/// — Enables compile-time cfg settings.
pub fn rustc_cfg_val(key: impl AsRef<str>, val: impl AsRef<str>) {
    println!("cargo:rustc-cfg={}={:?}", key.as_ref(), val.as_ref());
}

/// [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env)
/// — Sets an environment variable.
pub fn rustc_env(env: impl AsRef<str>, val: impl AsRef<str>) {
    println!("cargo:rustc-env={}={}", env.as_ref(), val.as_ref());
}

/// [`cargo:rustc-cdylib-link-arg=FLAG`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cdylib-link-arg)
/// — Passes custom flags to a linker for cdylib crates.
pub fn rustc_cdylib_link_arg(flag: impl AsRef<str>) {
    println!("cargo:rustc-cdylib-link-arg={}", flag.as_ref());
}

/// [`cargo:warning=MESSAGE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning)
/// — Displays a warning on the terminal.
pub fn warning(message: impl AsRef<str>) {
    for line in message.as_ref().lines() {
        println!("cargo:warning={}", line.trim_end());
    }
}

/// [`cargo:KEY=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key)
/// — Metadata, used by links scripts.
pub fn metadata(key: impl AsRef<str>, value: impl AsRef<str>) {
    println!("cargo:{}={}", key.as_ref(), value.as_ref());
}
