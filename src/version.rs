use std::io;



#[derive(Clone)]
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

    /// Check if the version is at least the given version
    ///
    /// ```rust
    /// # use mmrbi::Version;
    /// let cargo_1_47_0_stable     = Version::parse_rusty_version("cargo 1.47.0").unwrap();
    /// assert_eq!(false, cargo_1_47_0_stable   .is_at_least(1, 48, 0));
    /// assert_eq!(true,  cargo_1_47_0_stable   .is_at_least(1, 47, 0));
    /// assert_eq!(true,  cargo_1_47_0_stable   .is_at_least(1, 46, 0));
    ///
    /// let cargo_1_47_0_beta       = Version::parse_rusty_version("cargo 1.47.0-beta").unwrap();
    /// assert_eq!(false, cargo_1_47_0_beta     .is_at_least(1, 48, 0));
    /// assert_eq!(false, cargo_1_47_0_beta     .is_at_least(1, 47, 0));
    /// assert_eq!(true,  cargo_1_47_0_beta     .is_at_least(1, 46, 0));
    ///
    /// let cargo_1_47_0_nightly    = Version::parse_rusty_version("cargo 1.47.0-nightly").unwrap();
    /// assert_eq!(false, cargo_1_47_0_nightly  .is_at_least(1, 48, 0));
    /// assert_eq!(false, cargo_1_47_0_nightly  .is_at_least(1, 47, 0));
    /// assert_eq!(true,  cargo_1_47_0_nightly  .is_at_least(1, 46, 0));
    /// ```
    pub fn is_at_least(&self, major: u64, minor: u64, patch: u64) -> bool {
        let self_ver = (self.version.major, self.version.minor, self.version.patch);
        let check_ver = (major, minor, patch);

        if self.version.is_prerelease() {
            self_ver > check_ver
        } else {
            self_ver >= check_ver
        }
    }

    /// Check if the version is after the given version
    ///
    /// ```rust
    /// # use mmrbi::Version;
    /// let cargo_1_47_0_stable     = Version::parse_rusty_version("cargo 1.47.0").unwrap();
    /// assert_eq!(false, cargo_1_47_0_stable   .is_after(1, 48, 0));
    /// assert_eq!(false, cargo_1_47_0_stable   .is_after(1, 47, 0));
    /// assert_eq!(true,  cargo_1_47_0_stable   .is_after(1, 46, 0));
    ///
    /// let cargo_1_47_0_beta       = Version::parse_rusty_version("cargo 1.47.0-beta").unwrap();
    /// assert_eq!(false, cargo_1_47_0_beta     .is_after(1, 48, 0));
    /// assert_eq!(false, cargo_1_47_0_beta     .is_after(1, 47, 0));
    /// assert_eq!(true,  cargo_1_47_0_beta     .is_after(1, 46, 0));
    ///
    /// let cargo_1_47_0_nightly    = Version::parse_rusty_version("cargo 1.47.0-nightly").unwrap();
    /// assert_eq!(false, cargo_1_47_0_nightly  .is_after(1, 48, 0));
    /// assert_eq!(false, cargo_1_47_0_nightly  .is_after(1, 47, 0));
    /// assert_eq!(true,  cargo_1_47_0_nightly  .is_after(1, 46, 0));
    /// ```
    pub fn is_after(&self, major: u64, minor: u64, patch: u64) -> bool {
        let self_ver = (self.version.major, self.version.minor, self.version.patch);
        let check_ver = (major, minor, patch);
        self_ver > check_ver
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
