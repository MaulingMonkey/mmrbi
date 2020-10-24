use std::io;
use std::process::{Command, Stdio, Output};



/// Utility methods for [std::process::Command]
pub trait CommandExt {
    /// [Command::status], but returns an error if the process didn't have a zero exit code
    fn status0(&mut self) -> io::Result<()>;

    /// [Command::output], but returns an error if the process didn't have a zero exit code
    fn output0(&mut self) -> io::Result<Output>;

    /// [Command::output], but:
    ///
    /// * Returns an error if the process didn't have a zero exit code
    /// * Returns an error if stdout wasn't valid unicode
    /// * Returns *only* stdout
    /// * Stderr is inherited instead of redirected
    fn stdout0(&mut self) -> io::Result<String>;

    /// [Command::output], but:
    ///
    /// * Returns an error if the process didn't have a zero exit code
    /// * Returns an error if stdout wasn't valid unicode
    /// * Returns *only* stdout
    /// * Stderr is nulled instead of redirected
    fn stdout0_no_stderr(&mut self) -> io::Result<String>;
}

impl CommandExt for Command {
    fn status0(&mut self) -> io::Result<()> {
        let status = self.status()?;
        match status.code() {
            Some(0) => Ok(()),
            Some(n) => Err(io::Error::new(io::ErrorKind::Other, format!("exit code {}", n))),
            None    => Err(io::Error::new(io::ErrorKind::Other, "terminated by signal")),
        }
    }

    fn output0(&mut self) -> io::Result<Output> {
        let output = self.output()?;
        match output.status.code() {
            Some(0) => Ok(output),
            Some(n) => Err(io::Error::new(io::ErrorKind::Other, format!("exit code {}", n))),
            None    => Err(io::Error::new(io::ErrorKind::Other, "terminated by signal")),
        }
    }

    fn stdout0(&mut self) -> io::Result<String> {
        let output = self.stderr(Stdio::inherit()).output()?;
        match output.status.code() {
            Some(0) => {},
            Some(n) => return Err(io::Error::new(io::ErrorKind::Other, format!("exit code {}", n))),
            None    => return Err(io::Error::new(io::ErrorKind::Other, "terminated by signal")),
        }
        String::from_utf8(output.stdout).map_err(|_err| io::Error::new(io::ErrorKind::InvalidData, "stdout contained invalid unicode"))
    }

    fn stdout0_no_stderr(&mut self) -> io::Result<String> {
        let output = self.stderr(Stdio::null()).output()?;
        match output.status.code() {
            Some(0) => {},
            Some(n) => return Err(io::Error::new(io::ErrorKind::Other, format!("exit code {}", n))),
            None    => return Err(io::Error::new(io::ErrorKind::Other, "terminated by signal")),
        }
        String::from_utf8(output.stdout).map_err(|_err| io::Error::new(io::ErrorKind::InvalidData, "stdout contained invalid unicode"))
    }
}
