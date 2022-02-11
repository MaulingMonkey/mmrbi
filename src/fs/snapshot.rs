//! Create in-memory snapshots of directory trees etc.

use std::borrow::Cow;
use std::collections::*;
use std::ffi::{OsString, OsStr};
use std::fmt::{self, Formatter};
use std::fs::{Metadata, FileType};
use std::io::{self, Read as _};
use std::path::{PathBuf, Path};
use std::str::Utf8Error;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;



#[derive(Debug)]
pub struct File {
    name:       OsString,
    path:       PathBuf,
    metadata:   Metadata,
    data:       Arc<[u8]>,
}

#[derive(Debug)]
pub struct Dir {
    name:       OsString,
    path:       PathBuf,
    metadata:   Metadata,
    dirs:       BTreeMap<OsString, Arc<Dir>>,
    files:      BTreeMap<OsString, Arc<File>>,
}

#[derive(Debug)]
pub struct Entry<'a> {
    name: &'a OsStr,
    path: &'a Path,
    ty: &'a FileType,
}

impl File {
    /// Read a file.
    ///
    /// ### Example
    /// ```
    /// # use mmrbi::fs::snapshot::*;
    /// let file = File::read("Cargo.toml").unwrap();
    /// assert!(file.to_utf8_lossy().starts_with("# https://doc.rust-lang.org/cargo/reference/manifest.html"));
    /// ```
    pub fn read(path: impl AsRef<Path> + Into<PathBuf>) -> Result<Arc<Self>, Error> {
        let p = path.as_ref();
        if !p.is_file() {
            if !p.exists() {
                Err(Error::new(ErrorKind::DoesNotExist, io::ErrorKind::NotFound, path))
            } else if p.is_dir() {
                Err(Error::new(ErrorKind::ReadFileIsDir, io::ErrorKind::Other, path))
            } else {
                Err(Error::new(ErrorKind::NeitherFileNorDirectory, io::Error::new(io::ErrorKind::Other, "is not a file"), path))
            }
        } else if let Some(file_name) = p.file_name() {
            let file_name = OsString::from(file_name); // Fix borrowing of path
            CACHE.lock().unwrap().read_file(path, file_name)
        } else {
            Err(Error::new(ErrorKind::FileMissingFileName, io::ErrorKind::Other, path))
        }
    }

    pub fn name(&self)          -> &OsStr                   { &self.name }
    pub fn path(&self)          -> &Path                    { &self.path }
    pub fn metadata(&self)      -> &Metadata                { &self.metadata }
    pub fn as_bytes(&self)      -> &[u8]                    { &self.data }
    pub fn to_utf8(&self)       -> Result<&str, Utf8Error>  { std::str::from_utf8(self.as_bytes()) }
    pub fn to_utf8_lossy(&self) -> Cow<'_, str>             { String::from_utf8_lossy(self.as_bytes()) }
}

impl Dir {
    /// Read a directory.
    ///
    /// ### Example
    /// ```
    /// # use mmrbi::fs::snapshot::*;
    /// # use std::ffi::*;
    /// let src = Dir::read("src", |e| e.is_dir() || e.path().extension() == Some(OsStr::new("rs"))).unwrap();
    /// for (actual, expected) in src.dirs().into_iter().zip(["cargo", "fs"]) {
    ///     assert!(actual.name() == expected);
    /// }
    /// ```
    pub fn read(path: impl AsRef<Path> + Into<PathBuf>, filter: impl Fn(&Entry) -> bool) -> Result<Arc<Self>, Error> {
        let p = path.as_ref();
        if !p.is_dir() {
            if !p.exists() {
                Err(Error::new(ErrorKind::DoesNotExist, io::ErrorKind::NotFound, path))
            } else if p.is_file() {
                Err(Error::new(ErrorKind::ReadDirIsFile, io::ErrorKind::Other, path))
            } else {
                Err(Error::new(ErrorKind::NeitherFileNorDirectory, io::Error::new(io::ErrorKind::Other, "is not a directory"), path))
            }
        } else {
            CACHE.lock().unwrap().read_dir(path, &filter)
        }
    }

    pub fn name(&self)      -> &OsStr       { &self.name }
    pub fn path(&self)      -> &Path        { &self.path }
    pub fn metadata(&self)  -> &Metadata    { &self.metadata }
    pub fn dirs (&self) -> impl IntoIterator<Item = &Dir > + '_ { self.dirs .values().map(|dir | &**dir ) }
    pub fn files(&self) -> impl IntoIterator<Item = &File> + '_ { self.files.values().map(|file| &**file) }
}

impl Entry<'_> {
    pub fn name(&self) -> &OsStr { self.name }
    pub fn path(&self) -> &Path  { self.path }
    pub fn ty(&self) -> &FileType { self.ty }

    pub fn is_file      (&self) -> bool { self.ty.is_file()     }
    pub fn is_dir       (&self) -> bool { self.ty.is_dir()      }
    pub fn is_symlink   (&self) -> bool { self.ty.is_symlink()  }
}



lazy_static::lazy_static! {
    static ref CACHE : Mutex<Cache> = Default::default();
}

#[derive(Default)]
struct Cache {
    //data:   HashMap<Sha256?, Arc<[u8]>>,
    files:  HashMap<CacheFileKey, Arc<File>>,
    //dirs:   HashMap<CacheDirKey,  Arc<Dir>>,
}

impl Cache {
    fn read_file(&mut self, path: impl AsRef<Path> + Into<PathBuf>, name: impl AsRef<OsStr> + Into<OsString>) -> Result<Arc<File>, Error> {
        macro_rules! ok_or { ( $kind:expr, $io:expr ) => { match $io { Err(e) => return Err(Error::new($kind, e, path)), Ok(r) => r } }}

        let mut file    = ok_or!(ErrorKind::ReadFileOpen, std::fs::File::open(path.as_ref()));
        let metadata    = ok_or!(ErrorKind::ReadFileMeta, file.metadata());
        let modified    = ok_or!(ErrorKind::ReadFileMeta, metadata.modified());
        let len         = metadata.len();

        use hash_map::Entry::*;
        match self.files.entry(CacheFileKey { path: path.into(), modified, len }) {
            Occupied(e) => Ok(e.get().clone()),
            Vacant(e) => {
                let path = &e.key().path;
                macro_rules! ok_or { ( $kind:expr, $io:expr ) => { match $io { Err(e) => return Err(Error::new($kind, e, path)), Ok(r) => r } }}

                let mut data = Vec::new();
                data.reserve(metadata.len() as _);
                ok_or!(ErrorKind::ReadFileData, file.read_to_end(&mut data));
                // TODO: intern data by sha3 or blake2?

                let file = Arc::new(File {
                    name: name.into(),
                    path: path.into(),
                    data: data.into(),
                    metadata,
                });
                e.insert(file.clone());
                Ok(file)
            },
        }
    }

    fn read_dir(&mut self, path: impl AsRef<Path> + Into<PathBuf>, filter: &impl Fn(&Entry) -> bool) -> Result<Arc<Dir>, Error> {
        macro_rules! ok_or { ( $kind:expr, $io:expr ) => { match $io { Err(e) => return Err(Error::new($kind, e, path)), Ok(r) => r } }}

        let p           = path.as_ref();
        let metadata    = ok_or!(ErrorKind::ReadDirMeta, p.metadata());
        let mut dirs    = BTreeMap::<OsString, Arc<Dir>>::new();
        let mut files   = BTreeMap::<OsString, Arc<File>>::new();

        for e in ok_or!(ErrorKind::ReadDirOpen, std::fs::read_dir(p)) {
            let e = ok_or!(ErrorKind::ReadDirEntry, e);
            let e_path = e.path();
            macro_rules! ok_or { ( $kind:expr, $io:expr ) => { match $io { Err(e) => return Err(Error::new($kind, e, e_path)), Ok(r) => r } }}
            let e_name = e.file_name();
            let e_ty   = ok_or!(ErrorKind::ReadDirMeta, e.file_type());
            if !filter(&Entry{ name: &e_name, path: &e_path, ty: &e_ty }) {
                // do nothing: filtered out
            } else if e_ty.is_dir() {
                let dir = self.read_dir(e_path, filter)?;
                dirs.insert(dir.name.clone(), dir);
            } else if e_ty.is_file() {
                let file = self.read_file(e_path, e_name)?;
                files.insert(file.name.clone(), file);
            } else {
                // do nothing: neither a dir nor a file
            }
        }

        Ok(Arc::new(Dir{
            name: p.file_name().unwrap_or(OsStr::new(".")).into(),
            path: path.into(),
            metadata,
            files,
            dirs,
        }))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] struct CacheFileKey {
    path:       PathBuf,
    modified:   SystemTime,
    len:        u64,
}



#[derive(Debug)]
pub struct Error {
    kind:   ErrorKind,
    io:     io::Error,
    path:   PathBuf,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, io: impl Into<io::Error>, path: impl Into<PathBuf>) -> Self { Self { kind, io: io.into(), path: path.into() } }
    pub fn kind(&self) -> ErrorKind { self.kind }
    pub fn io_kind(&self) -> io::ErrorKind { self.io.kind() }
    pub fn path(&self) -> &Path { self.path.as_ref() }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let path = self.path.display();
        let io = &self.io;
        match self.kind {
            ErrorKind::DoesNotExist             => write!(fmt, "{path} does not exist"),
            ErrorKind::NeitherFileNorDirectory  => write!(fmt, "{path} is neither a file, nor a directory"),
            ErrorKind::FileMissingFileName      => write!(fmt, "{path} is supposely a file, but somehow also missing a filename"),

            ErrorKind::ReadDirIsFile            => write!(fmt, "{path} could not be opened as a directory: it is a file"),
            ErrorKind::ReadDirOpen              => write!(fmt, "{path} could not be opened as a directory: {io}"),
            ErrorKind::ReadDirEntry             => write!(fmt, "{path} could not be fully enumerated: {io}"),
            ErrorKind::ReadDirMeta              => write!(fmt, "{path} could not have directory metadata read: {io}"),

            ErrorKind::ReadFileIsDir            => write!(fmt, "{path} could not be opened as a file: it is a directory"),
            ErrorKind::ReadFileOpen             => write!(fmt, "{path} could not be opened as a file: {io}"),
            ErrorKind::ReadFileData             => write!(fmt, "{path} could not be fully read: {io}"),
            ErrorKind::ReadFileMeta             => write!(fmt, "{path} could not have file metadata read: {io}"),
        }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error { io::Error::new(err.io.kind(), err) }
}

impl std::error::Error for Error {}



#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {
    DoesNotExist,
    NeitherFileNorDirectory,
    FileMissingFileName,

    ReadDirIsFile,
    ReadDirOpen,
    ReadDirEntry,
    ReadDirMeta,

    ReadFileIsDir,
    ReadFileOpen,
    ReadFileData,
    ReadFileMeta,
}
