use std::path::PathBuf;



/// An error or warning
#[derive(Debug)]
#[non_exhaustive]
pub struct Diagnostic {
    pub path:       Option<PathBuf>,
    pub message:    String,
    pub kind:       DiagKind,
}

/// Diagnostic type information / additional payload
#[derive(Debug)]
#[non_exhaustive]
pub enum DiagKind {
    Warning,
    Malformed,
    Bug,
    Io(std::io::Error),
    Toml(toml::de::Error),
}

impl From<std::io::Error>   for DiagKind { fn from(err: std::io::Error)    -> DiagKind { DiagKind::Io(err) } }
impl From<toml::de::Error>  for DiagKind { fn from(err: toml::de::Error)   -> DiagKind { DiagKind::Toml(err) } }
