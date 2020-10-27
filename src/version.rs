use std::io;



pub struct Version {
    pub tool_name:  String,
    pub version:    semver::Version,
    pub hash:       String,
    pub date:       String
}

impl Version {
    /// Parse input in the format `"{tool_name} {version}"` or `"{tool_name} {version} ({hash} {date})"`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use mmrbi::Version;
    /// Version::parse_rusty_version("rustup 1.22.1 (b01adbbc3 2020-07-08)").unwrap();
    /// Version::parse_rusty_version("cargo 1.47.0 (f3c7e066a 2020-08-28)").unwrap();
    /// Version::parse_rusty_version("rustc 1.47.0 (18bf6b4f0 2020-10-07)").unwrap();
    ///
    /// Version::parse_rusty_version("rustup 1.22.1").unwrap();
    /// Version::parse_rusty_version("cargo 1.47.0").unwrap();
    /// Version::parse_rusty_version("rustc 1.47.0").unwrap();
    /// ```
    pub fn parse_rusty_version(line: &str) -> io::Result<Self> {
        let mut line    = line.trim_end_matches(|ch| "\r\n".contains(ch)).splitn(4, " ");
        let tool_name   = line.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing tool name"))?;
        let semver      = line.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing semver"))?;
        let hash        = line.next().map_or(Ok(""), parse_hash)?;
        let date        = line.next().map_or(Ok(""), parse_date)?;

        let semver      = semver::Version::parse(semver).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, format!("invalid version: {}", err)))?;

        Ok(Self{
            tool_name:  tool_name.into(),
            version:    semver,
            hash:       hash.into(),
            date:       date.into(),
        })
    }
}

impl std::str::FromStr for Version {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Self::parse_rusty_version(s) }
}

fn parse_hash(hash: &str) -> io::Result<&str> {
    if !hash.starts_with("(") {
        Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected hash {:?} to be enclosed in a parenthesis", hash)))
    } else if hash[1..].chars().any(|ch| !ch.is_ascii_hexdigit()) {
        Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected hash {:?} to be exclusively ASCII hexidecimal digits", &hash[1..])))
    } else {
        Ok(&hash[1..])
    }
}

fn parse_date(date: &str) -> io::Result<&str> {
    if !date.ends_with(")") {
        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected date {:?} to be enclosed in a parenthesis", date)));
    }
    let date = &date[..(date.len()-1)];
    if date.chars().any(|ch| !(ch.is_ascii_digit() || ch == '-')) {
        Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected date {:?} to be exclusively ASCII digits or '-' separators", date)))
    } else {
        Ok(date)
    }
}



pub use semver::VersionReq;
