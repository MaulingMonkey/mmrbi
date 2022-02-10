//! Windows specific paths

#![cfg_attr(doc_cfg, doc(cfg(windows)))]
#![cfg(windows)]

use std::ffi::OsStr;
use std::path::PathBuf;
use std::io;



/// Retrieve `C:\Program Files\` (64-bit) by reading `%ProgramW6432%`
pub fn program_files_x64() -> io::Result<PathBuf> {
    if let Ok(path) = crate::env::var_path("ProgramW6432") {
        Ok(path)
    } else {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}

/// Retrieve `C:\Program Files (x86)\` (32-bit) by reading `%ProgramFiles(x86)%` / `%ProgramFiles%`
pub fn program_files_x86() -> io::Result<PathBuf> {
    if let Ok(path) = crate::env::var_path("ProgramFiles(x86)") {
        Ok(path)
    } else if let Ok(path) = crate::env::var_path("ProgramFiles") {
        Ok(path)
    } else {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}

/// Enumerate over `C:\Program Files\` / `C:\Program Files (x86)\` (in that order)
pub fn program_files() -> impl Iterator<Item = PathBuf> {
    program_files_x64().into_iter().chain(program_files_x86())
}

/// Try to find `dxc.exe`, the SM 6.0 and later HLSL shader compiler
pub fn find_dxc_exe() -> io::Result<PathBuf> { find_win_kit_10_bin("dxc.exe") }

/// Try to find `fxc.exe`, the SM 5.0 and earlier HLSL shader compiler
pub fn find_fxc_exe() -> io::Result<PathBuf> { find_win_kit_10_bin("fxc.exe") }

fn find_win_kit_10_bin(bin: &str) -> io::Result<PathBuf> {
    let x64 = program_files_x64().is_ok();
    let mut dir = program_files_x86()?;
    dir.push(r"Windows Kits\10\bin");
    let dir : Vec<crate::fs::DirPathType> = crate::fs::collect_dir(&dir)?;
    for dir in dir.iter().rev() {
        if ["arm", "arm64", "x64", "x86"].iter().any(|name| dir.path.file_name() == Some(OsStr::new(name))) { continue }

        #[cfg(any(target_arch = "i586", target_arch = "i686", target_arch = "x86_64"))]
        if x64 {
            let bin = dir.path.join("x64").join(bin);
            if bin.exists() { return Ok(bin); }
        }

        #[cfg(any(target_arch = "i586", target_arch = "i686", target_arch = "x86_64"))] {
            let bin = dir.path.join("x86").join(bin);
            if bin.exists() { return Ok(bin); }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

#[test] fn find_exes() {
    assert!(find_dxc_exe().is_ok());
    assert!(find_fxc_exe().is_ok());
}
