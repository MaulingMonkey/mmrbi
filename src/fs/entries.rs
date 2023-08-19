use std::ffi::OsString;
use std::fs::{FileType, DirEntry};
use std::io;
use std::path::{Path, PathBuf};



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
        let mut dirs : Vec<Self> = super::collect_dir(path)?;
        alphanumeric_sort::sort_slice_by_os_str_key(&mut dirs[..], |d| d.file_name.as_os_str());
        Ok(dirs.into_iter())
    }
}

impl DirPathType {
    pub fn is_dir       (&self) -> bool { self.file_type.is_dir() }
    pub fn is_file      (&self) -> bool { self.file_type.is_file() }
    pub fn is_symlink   (&self) -> bool { self.file_type.is_symlink() }

    pub fn read_by_alphanumeric(path: impl AsRef<Path>) -> io::Result<impl Iterator<Item = Self>> {
        let mut dirs : Vec<Self> = super::collect_dir(path)?;
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
