use crate::PathExt;
use super::{Diagnostic, DiagKind};

use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::{Component, Components, Path, PathBuf};
use std::io;



pub(super) fn pop1(path: impl Into<PathBuf>) -> PathBuf {
    let mut path = path.into();
    assert!(path.pop());
    path
}

pub(super) fn enum_manifest_pattern(
    insert:             bool,
    out_paths:          &mut BTreeSet<PathBuf>,
    path:               &mut PathBuf,
    mut components:     Components,
    original_pattern:   &Path,
    out_errors:         &mut Vec<Diagnostic>,
) {
    if let Some(c) = components.next() {
        match c {
            c @ Component::Prefix(_) => {
                let mut path = PathBuf::from(c.as_os_str());
                let _root = components.next();
                enum_manifest_pattern(insert, out_paths, &mut path, components, original_pattern, out_errors);
            },
            c @ Component::RootDir => {
                let mut path = PathBuf::from(c.as_os_str());
                enum_manifest_pattern(insert, out_paths, &mut path, components, original_pattern, out_errors);
            },
            Component::CurDir => {
                enum_manifest_pattern(insert, out_paths, path, components, original_pattern, out_errors);
            },
            Component::ParentDir => {
                let mut path = path.join("..").cleanup();
                enum_manifest_pattern(insert, out_paths, &mut path, components, original_pattern, out_errors);
            },
            Component::Normal(c) => {
                if c == OsStr::new("*") {
                    match path.read_dir() {
                        Ok(r) => {
                            for entry in r {
                                match entry {
                                    Ok(entry) => {
                                        path.push(entry.file_name());
                                        match entry.metadata() {
                                            Ok(metadata) if !metadata.is_dir() => {} // noop
                                            Ok(_) => enum_manifest_pattern(insert, out_paths, path, components.clone(), original_pattern, out_errors),
                                            Err(err) => out_errors.push(Diagnostic {
                                                path:       Some(path.clone()),
                                                message:    format!("error during enumeration (expected by pattern `{}`)", original_pattern.display()),
                                                kind:       DiagKind::Io(err)
                                            }),
                                        }
                                        assert!(path.pop());
                                    },
                                    Err(err) => out_errors.push(Diagnostic {
                                        path:       Some(path.clone()),
                                        message:    format!("error during enumeration (expected by pattern `{}`)", original_pattern.display()),
                                        kind:       DiagKind::Io(err)
                                    }),
                                }
                            }
                        },
                        Err(err) => out_errors.push(Diagnostic {
                            path:       Some(path.clone()),
                            message:    format!("unable to enumerate (expected by pattern `{}`)", original_pattern.display()),
                            kind:       DiagKind::Io(err)
                        }),
                    };
                } else {
                    path.push(c);
                    enum_manifest_pattern(insert, out_paths, path, components, original_pattern, out_errors);
                    assert!(path.pop());
                }
            },
        }
    } else {
        path.push("Cargo.toml");
        if !insert {
            out_paths.remove(path.as_path());
        } else if path.exists() {
            if let Some(prev) = out_paths.replace(path.clone()) {
                out_errors.push(Diagnostic {
                    path:       Some(prev),
                    message:    format!("multiple matches (repeat pattern: `{}`)", original_pattern.display()),
                    kind:       DiagKind::Warning
                });
            }
        } else { // TODO: make this conditional on path.parent() being a directory?
            out_errors.push(Diagnostic {
                path:       Some(path.clone()),
                message:    format!("expected by pattern `{}`", original_pattern.display()),
                kind:       DiagKind::Io(io::Error::from(io::ErrorKind::NotFound))
            });
        }
        assert!(path.pop());
    }
}
