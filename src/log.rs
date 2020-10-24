use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering::AcqRel};



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Severity {
    Error,
    Warning,
    Info,
}



#[derive(Clone)]
#[non_exhaustive]
pub struct Stats {
    pub errors:     usize,
    pub warnings:   usize,
}

impl Stats {
    pub fn get() -> Self {
        Self {
            errors:     ERRORS.load(AcqRel),
            warnings:   WARNINGS.load(AcqRel),
        }
    }
}


pub struct Context<'c> {
    pub severity:   Severity,
    pub code:       &'c str,
    pub at:         Option<&'c Path>,
    pub line:       usize,
    pub col:        usize,
}



pub fn write(ctx: Context, f: impl FnOnce(&mut std::io::StderrLock) -> io::Result<()>) {
    let pre = match ctx.severity {
        Severity::Error     => "\u{001B}[31;1merror",
        Severity::Warning   => "\u{001B}[33;1mwarning",
        Severity::Info      => "\u{001B}[36;1minfo",
    };

    use std::io::Write;
    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();
    let _ = write!(&mut stderr, "{}{}\u{001B}[37m:\u{001B}[0m ", pre, ctx.code);
    let _ = f(&mut stderr);

    if let Some(at) = ctx.at {
        let _ = writeln!(&mut stderr, "  \u{001B}[36;1m-->\u{001B}[0m {}:{}:{}", at.display(), ctx.line, ctx.col);
    }
}

static ERRORS   : AtomicUsize = AtomicUsize::new(0);
static WARNINGS : AtomicUsize = AtomicUsize::new(0);
