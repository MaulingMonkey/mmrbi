//! Similar to [std::fs::*](std::fs)

use std::io::{self, Cursor};
use std::path::Path;

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
