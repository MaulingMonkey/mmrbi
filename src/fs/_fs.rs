//! Similar to [std::fs::*](std::fs)

mod entries;        pub use entries::*;
pub mod snapshot;
pub mod windows;



use crate::io::EolRewriter;

use std::convert::TryFrom;
use std::fs::DirEntry;
use std::io::{self, Cursor};
use std::path::Path;



/// Write the output of `io(&mut o)?` to `path` unless unchanged.
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

/// Write the output of `io(&mut o)?` (replacing `\n` with `\r\n` on windows) to `path` unless unchanged.
pub fn write_text_if_modified_with(path: impl AsRef<Path>, io: impl FnOnce(EolRewriter<&mut Cursor<Vec<u8>>>) -> io::Result<()>) -> io::Result<bool> {
    write_if_modified_with(path, |w| io(EolRewriter(w)))
}

/// Collect directories (like [std::fs::read_dir], but only one io::Result to untangle, and allocates)
pub fn collect_dir<C: Default + Extend<D>, D: TryFrom<DirEntry, Error=io::Error>>(path: impl AsRef<Path>) -> io::Result<C> {
    let mut c = C::default();
    for dir in std::fs::read_dir(path)? {
        c.extend(Some(D::try_from(dir?)?));
    }
    Ok(c)
}

#[test] fn platforms() {
    if let Ok(platforms) = DirNameType::read_by_alphanumeric(r"C:\Program Files (x86)\Android\android-sdk\platforms") {
        for e in platforms {
            if !e.is_dir() { continue }
            eprintln!("{}", Path::new(&e.file_name).display());
        }
    }
}
