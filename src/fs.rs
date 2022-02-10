//! Similar to [std::fs::*](std::fs)

use std::convert::TryFrom;
use std::ffi::OsString;
use std::fs::{DirEntry, FileType};
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

/// Write the output of `io(&mut output)?` to `path` if it differed from what already existed there (if anything.)
pub fn write_if_modified_with(path: impl AsRef<Path>, io: impl FnOnce(&mut Cursor<Vec<u8>>) -> io::Result<()>) -> io::Result<bool> {
    let path = path.as_ref();
    let mut c = Cursor::new(Vec::new());
    io(&mut c)?;
    let v = c.into_inner();
    match std::fs::read(path) {
        Ok(bytes) if bytes == v                             => Ok(false),
        Ok(_orig)                                           => std::fs::write(path, v).map(|()| true).map_err(|err| io::Error::new(err.kind(), format!("{}: {}", path.display(), err))),
        Err(err) if err.kind() == io::ErrorKind::NotFound   => std::fs::write(path, v).map(|()| true).map_err(|err| io::Error::new(err.kind(), format!("{}: {}", path.display(), err))),
        Err(err)                                            => Err(err),
    }
}

/// A directory [OsString] + [FileType]
#[derive(Clone, Debug)]
pub struct DirNameType {
    pub file_name:  OsString,
    pub file_type:  FileType,
}

/// A directory [PathBuf] + [FileType]
#[derive(Clone, Debug)]
pub struct DirPathType {
    pub path:       PathBuf,
    pub file_type:  FileType,
}

impl DirNameType {
    pub fn is_dir       (&self) -> bool { self.file_type.is_dir() }
    pub fn is_file      (&self) -> bool { self.file_type.is_file() }
    pub fn is_symlink   (&self) -> bool { self.file_type.is_symlink() }

    pub fn read_by_alphanumeric(path: impl AsRef<Path>) -> io::Result<impl Iterator<Item = Self>> {
        let mut dirs : Vec<Self> = collect_dir(path)?;
        alphanumeric_sort::sort_slice_by_os_str_key(&mut dirs[..], |d| d.file_name.as_os_str());
        Ok(dirs.into_iter())
    }
}

impl DirPathType {
    pub fn is_dir       (&self) -> bool { self.file_type.is_dir() }
    pub fn is_file      (&self) -> bool { self.file_type.is_file() }
    pub fn is_symlink   (&self) -> bool { self.file_type.is_symlink() }

    pub fn read_by_alphanumeric(path: impl AsRef<Path>) -> io::Result<impl Iterator<Item = Self>> {
        let mut dirs : Vec<Self> = collect_dir(path)?;
        alphanumeric_sort::sort_slice_by_os_str_key(&mut dirs[..], |d| d.path.as_path());
        Ok(dirs.into_iter())
    }
}

impl TryFrom<DirEntry> for DirNameType {
    type Error = io::Error;
    fn try_from(de: DirEntry) -> Result<Self, Self::Error> {
        Ok(Self {
            file_name:  de.file_name(),
            file_type:  de.file_type()?,
        })
    }
}

impl TryFrom<DirEntry> for DirPathType {
    type Error = io::Error;
    fn try_from(de: DirEntry) -> Result<Self, Self::Error> {
        Ok(Self {
            path:       de.path(),
            file_type:  de.file_type()?,
        })
    }
}

/// Collect directories (like [std::fs::read_dir], but only one io::Result to untangle, and allocates)
pub fn collect_dir<C: Default + Extend<D>, D: TryFrom<DirEntry, Error=io::Error>>(path: impl AsRef<Path>) -> io::Result<C> {
    let mut c = C::default();
    for dir in std::fs::read_dir(path)? {
        c.extend(Some(D::try_from(dir?)?));
    }
    Ok(c)
}

/// Windows specific paths
#[cfg_attr(doc_cfg, doc(cfg(windows)))]
#[cfg(windows)]
pub mod windows {
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
}

#[test] fn platforms() {
    if let Ok(platforms) = DirNameType::read_by_alphanumeric(r"C:\Program Files (x86)\Android\android-sdk\platforms") {
        for e in platforms {
            if !e.is_dir() { continue }
            eprintln!("{}", Path::new(&e.file_name).display());
        }
    }
}
