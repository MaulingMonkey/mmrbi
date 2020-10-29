use std::collections::*;
use std::fmt::{self, Display, Debug, Formatter};
use std::ffi::*;
use std::io::{self, BufRead, BufReader};
use std::path::*;
use std::process::{Child, ExitStatus, Output, Stdio};
use std::sync::Arc;
use std::thread;


/// A [Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)able, [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)able clone of [std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html)
#[derive(Clone)]
pub struct Command {
    program:    OsString,
    dir:        Option<PathBuf>,
    args:       Vec<OsString>,
    env_clear:  bool,
    env:        BTreeMap<OsString, OsString>,

    stdin:      Option<Arc<dyn Fn() -> Stdio>>,
    stdout:     Option<Arc<dyn Fn() -> Stdio>>,
    stderr:     Option<Arc<dyn Fn() -> Stdio>>,
}

impl Display for Command {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "`{}", Path::new(&self.program).display())?;
        for arg in self.args.iter() {
            let arg = arg.to_string_lossy();
            if arg.contains(|ch| ch == ' ' || ch == '\"') {
                write!(fmt, " \"{}\"", arg.replace("\"", "\\\""))?;
            } else {
                write!(fmt, " {}", arg)?;
            }
        }
        write!(fmt, "`")
    }
}

impl Debug for Command {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "`{}", Path::new(&self.program).display())?;
        for arg in self.args.iter() {
            let arg = arg.to_string_lossy();
            if arg.contains(|ch| ch == ' ' || ch == '\"') {
                write!(fmt, " \"{}\"", arg.replace("\"", "\\\""))?;
            } else {
                write!(fmt, " {}", arg)?;
            }
        }
        write!(fmt, "`")?;
        if let Some(dir) = self.dir.as_ref() {
            write!(fmt, ", in `{}`", dir.display())?;
        }
        if self.env_clear {
            write!(fmt, ", with env cleared")?;
        }
        if !self.env.is_empty() {
            write!(fmt, ", with env = {{")?;
            for (k, v) in self.env.iter() {
                write!(fmt, " {:?} = {:?},", k, v)?;
            }
            write!(fmt, "}}")?;
        }
        Ok(())
    }
}

impl Command {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            program:    program.as_ref().into(),
            dir:        None,
            args:       Default::default(),
            env_clear:  false,
            env:        Default::default(),

            stdin:      None,
            stdout:     None,
            stderr:     None,
        }
    }

    pub fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
        self.args.push(arg.as_ref().into());
        self
    }

    pub fn args<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(&mut self, args: I) -> &mut Self {
        self.args.extend(args.into_iter().map(|a| a.as_ref().into()));
        self
    }

    pub fn env(&mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> &mut Self {
        self.env.insert(key.as_ref().into(), val.as_ref().into());
        self
    }

    pub fn envs<I: IntoIterator<Item = (K, V)>, K: AsRef<OsStr>, V: AsRef<OsStr>>(&mut self, vars: I) -> &mut Self {
        self.env.extend(vars.into_iter().map(|(k, v)| (k.as_ref().into(), v.as_ref().into())));
        self
    }

    pub fn env_remove(&mut self, key: impl AsRef<OsStr>) -> &mut Self {
        self.env.insert(key.as_ref().into(), OsString::new());
        self
    }

    pub fn env_clear(&mut self) -> &mut Self {
        self.env.clear();
        self.env_clear = true;
        self
    }

    pub fn current_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self {
        self.dir = Some(dir.as_ref().into());
        self
    }

    pub fn stdin(&mut self, f: impl Fn() -> Stdio + 'static) -> &mut Self {
        self.stdin = Some(Arc::new(f));
        self
    }

    pub fn stdout(&mut self, f: impl Fn() -> Stdio + 'static) -> &mut Self {
        self.stdout = Some(Arc::new(f));
        self
    }

    pub fn stderr(&mut self, f: impl Fn() -> Stdio + 'static) -> &mut Self {
        self.stderr = Some(Arc::new(f));
        self
    }

    pub fn to_command(&self) -> std::process::Command {
        let mut c = std::process::Command::new(&self.program);
        if let Some(dir) = self.dir.as_ref() { c.current_dir(dir); }
        c.args(self.args.iter());
        if self.env_clear { c.env_clear(); }
        c.envs(self.env.iter());
        if let Some(stdin ) = self.stdin .as_ref() { c.stdin (stdin ()); }
        if let Some(stdout) = self.stdout.as_ref() { c.stdout(stdout()); }
        if let Some(stderr) = self.stderr.as_ref() { c.stderr(stderr()); }
        c
    }

    pub fn spawn (&self) -> io::Result<Child>       { self.to_command().spawn() .map_err(|err| io::Error::new(err.kind(), format!("{} failed: {}", self, err))) }
    pub fn output(&self) -> io::Result<Output>      { self.to_command().output().map_err(|err| io::Error::new(err.kind(), format!("{} failed: {}", self, err))) }
    pub fn status(&self) -> io::Result<ExitStatus>  { self.to_command().status().map_err(|err| io::Error::new(err.kind(), format!("{} failed: {}", self, err))) }
}

impl crate::CommandExt for Command {
    fn status0(&mut self) -> io::Result<()> {
        let status = self.status()?;
        match status.code() {
            Some(0) => Ok(()),
            Some(n) => Err(io::Error::new(io::ErrorKind::Other, format!("{} failed: exit code {}", self, n))),
            None    => Err(io::Error::new(io::ErrorKind::Other, format!("{} failed: terminated by signal", self))),
        }
    }

    fn output0(&mut self) -> io::Result<Output> {
        let output = self.output()?;
        match output.status.code() {
            Some(0) => Ok(output),
            Some(n) => Err(io::Error::new(io::ErrorKind::Other, format!("{} failed: exit code {}", self, n))),
            None    => Err(io::Error::new(io::ErrorKind::Other, format!("{} failed: terminated by signal", self))),
        }
    }

    fn stdout0(&mut self) -> io::Result<String> {
        let output = self.to_command().stderr(Stdio::inherit()).output().map_err(|err| io::Error::new(err.kind(), format!("{} failed: {}", self, err)))?;
        match output.status.code() {
            Some(0) => {},
            Some(n) => return Err(io::Error::new(io::ErrorKind::Other, format!("{} failed: exit code {}", self, n))),
            None    => return Err(io::Error::new(io::ErrorKind::Other, format!("{} failed: terminated by signal", self))),
        }
        String::from_utf8(output.stdout).map_err(|_err| io::Error::new(io::ErrorKind::InvalidData, format!("{} failed: stdout contained invalid unicode", self)))
    }

    fn stdout0_no_stderr(&mut self) -> io::Result<String> {
        let output = self.to_command().stderr(Stdio::null()).output().map_err(|err| io::Error::new(err.kind(), format!("{} failed: {}", self, err)))?;
        match output.status.code() {
            Some(0) => {},
            Some(n) => return Err(io::Error::new(io::ErrorKind::Other, format!("{} failed: exit code {}", self, n))),
            None    => return Err(io::Error::new(io::ErrorKind::Other, format!("{} failed: terminated by signal", self))),
        }
        String::from_utf8(output.stdout).map_err(|_err| io::Error::new(io::ErrorKind::InvalidData, format!("{} failed: stdout contained invalid unicode", self)))
    }

    fn io(&mut self, on_out: impl Fn(&str) + Send + Sync + 'static, on_err: impl Fn(&str) + Send + Sync + 'static) -> io::Result<ExitStatus> {
        let mut child = self.to_command().stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().map_err(|err| io::Error::new(err.kind(), format!("{} failed: {}", self, err)))?;

        let stdout = child.stdout.take().map(|stdout| thread::spawn(move ||{
            for line in BufReader::new(stdout).lines() {
                on_out(&line.unwrap());
            }
        }));
        let stderr = child.stderr.take().map(|stderr| thread::spawn(move ||{
            for line in BufReader::new(stderr).lines() {
                on_err(&line.unwrap());
            }
        }));
        let es = child.wait().map_err(|err| io::Error::new(err.kind(), format!("{} failed: {}", self, err)))?;
        stdout.map(|t| t.join().unwrap());
        stderr.map(|t| t.join().unwrap());
        Ok(es)
    }

    fn io0(&mut self, on_out: impl Fn(&str) + Send + Sync + 'static, on_err: impl Fn(&str) + Send + Sync + 'static) -> io::Result<()> {
        let status = self.io(on_out, on_err)?;
        match status.code() {
            Some(0) => Ok(()),
            Some(n) => Err(io::Error::new(io::ErrorKind::Other, format!("{} failed: exit code {}", self, n))),
            None    => Err(io::Error::new(io::ErrorKind::Other, format!("{} failed: terminated by signal", self))),
        }
    }
}



pub struct IoLine<'s> {
    pub(crate) line:   &'s str,
    pub(crate) err:    bool
}

impl IoLine<'_> {
    pub fn as_str(&self) -> &str { self.line }
    pub fn is_stdout(&self) -> bool { !self.err }
    pub fn is_stderr(&self) -> bool {  self.err }
}

impl std::convert::AsRef<str>           for IoLine<'_>  { fn as_ref(&self) -> &str { self.line } }
impl std::borrow::Borrow<str>           for IoLine<'_>  { fn borrow(&self) -> &str { self.line } }
impl std::ops::Deref                    for IoLine<'_>  { fn deref(&self) -> &Self::Target { self.line } type Target = str;  }
impl std::fmt::Debug                    for IoLine<'_>  { fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result { std::fmt::Debug::fmt(&self.line, fmt) } }
impl std::fmt::Display                  for IoLine<'_>  { fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result { std::fmt::Display::fmt(&self.line, fmt) } }
impl std::cmp::PartialEq< str>          for IoLine<'_>  { fn eq(&self, other: &str      ) -> bool { &**self == other } }
impl std::cmp::PartialEq<&str>          for IoLine<'_>  { fn eq(&self, other: &&str     ) -> bool { &**self == *other } }
impl std::cmp::PartialEq<IoLine<'_>>    for  str        { fn eq(&self, other: &IoLine   ) -> bool { self == &**other } }
impl std::cmp::PartialEq<IoLine<'_>>    for &str        { fn eq(&self, other: &IoLine   ) -> bool { *self == &**other } }
