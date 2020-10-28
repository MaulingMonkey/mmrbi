use std::io::{self, BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio, Output};
use std::thread;



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

    /// [Command::status], but provides a callback for stdout/stderr
    fn io(&mut self, on_out: impl Fn(&str) + Send + Sync + 'static, on_err: impl Fn(&str) + Send + Sync + 'static) -> io::Result<ExitStatus>;

    /// [Command::status], but provides a callback for stdout/stderr and returns an error if the process didn't have a zero exit code
    fn io0(&mut self, on_out: impl Fn(&str) + Send + Sync + 'static, on_err: impl Fn(&str) + Send + Sync + 'static) -> io::Result<()>;
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

    fn io(&mut self, on_out: impl Fn(&str) + Send + Sync + 'static, on_err: impl Fn(&str) + Send + Sync + 'static) -> io::Result<ExitStatus> {
        let mut child = self.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

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
        let es = child.wait()?;
        stdout.map(|t| t.join().unwrap());
        stderr.map(|t| t.join().unwrap());
        Ok(es)
    }

    fn io0(&mut self, on_out: impl Fn(&str) + Send + Sync + 'static, on_err: impl Fn(&str) + Send + Sync + 'static) -> io::Result<()> {
        let status = self.io(on_out, on_err)?;
        match status.code() {
            Some(0) => Ok(()),
            Some(n) => Err(io::Error::new(io::ErrorKind::Other, format!("exit code {}", n))),
            None    => Err(io::Error::new(io::ErrorKind::Other, "terminated by signal")),
        }
    }
}
