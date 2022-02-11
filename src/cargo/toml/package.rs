//! [`[package]`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section) types —
//! [Category], [Edition], [License], [Name], [Pattern], [Publish], [Url], and [Version]

use serde::*;

use std::borrow::{Borrow, Cow};
use std::fmt::{self, Debug, Display, Formatter};
use std::path::PathBuf;
use std::ops::Deref;



/// [`[package]`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section)
/// — Defines a package.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
#[serde(rename_all="kebab-case")]
pub struct Package<Metadata = toml::value::Table> {
    pub name:           Name,
    pub version:        Version,
    #[serde(default)] pub authors:        Vec<String>,
    #[serde(default)] pub edition:        Edition,
    #[serde(default)] pub description:    Option<String>,
    #[serde(default)] pub documentation:  Option<Url>,
    #[serde(default)] pub readme:         Option<PathBuf>,
    #[serde(default)] pub homepage:       Option<Url>,
    #[serde(default)] pub repository:     Option<Url>,
    #[serde(default)] pub license:        Option<License>,
    #[serde(default)] pub license_file:   Option<PathBuf>,
    #[serde(default)] pub keywords:       Vec<String>,
    #[serde(default)] pub categories:     Vec<Category>,
    #[serde(default)] pub workspace:      Option<PathBuf>,
    #[serde(default)] pub build:          Option<PathBuf>,
    #[serde(default)] pub links:          Option<String>,
    #[serde(default)] pub exclude:        Vec<Pattern>,
    #[serde(default)] pub include:        Vec<Pattern>,
    #[serde(default)] pub publish:        Publish,
    #[serde(default)] pub metadata:       Metadata,
    #[serde(default)] pub default_run:    Option<String>,
    #[serde(default)] pub autobins:       Option<bool>, // default: true unless (edition == 2015 && manual targets > 0)
    #[serde(default)] pub autoexamples:   Option<bool>, // default: true unless (edition == 2015 && manual targets > 0)
    #[serde(default)] pub autotests:      Option<bool>, // default: true unless (edition == 2015 && manual targets > 0)
    #[serde(default)] pub autobenches:    Option<bool>, // default: true unless (edition == 2015 && manual targets > 0)
    #[serde(flatten)] rest:               toml::value::Table
}

/// A Cargo.toml [`package.categories`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-categories-field)
/// crates\.io [category slug](https://crates.io/category_slugs)
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category(String);
// TODO: impl tons of category slugs?

/// A Cargo.toml [`package.edition`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-edition-field),
/// typically "2015", "2018", or "2021"
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edition(Cow<'static, str>);
impl Edition {
    pub const V2015 : Edition = Edition(Cow::Borrowed("2015"));
    pub const V2018 : Edition = Edition(Cow::Borrowed("2018"));
    pub const V2021 : Edition = Edition(Cow::Borrowed("2021"));
}

/// A Cargo.toml [`package.license`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields),
/// which should be a valid [SPDX 2.1 license expression](https://spdx.org/spdx-specification-21-web-version#h.jxpfx0ykyb60).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct License(String);

/// A Cargo.toml [`package.name`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field)
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(String);

/// A Cargo.toml [`package.include`/`.exclude`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields) or
/// [`workspace.members`/`.exclude`](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-workspace-section) pattern
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pattern(String);

/// A Cargo.toml [`package.publish`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field)
/// — prevent publishing, or limit publishing to a list of known registries
#[derive(Deserialize, Serialize, Clone, PartialEq)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Publish {
    Enabled(bool),
    Registries(Vec<String>),
}

/// A Cargo.toml url such as
/// [`package.documentation`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-documentation-field),
/// [`package.homepage`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-homepage-field), or
/// [`package.repository`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-repository-field).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Url(String);

/// A Cargo.toml [`package.version`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(String);



impl Category   { pub fn new(value: impl Into<String>) -> Self { Self(value.into().into()) } }
impl Edition    { pub fn new(value: impl Into<String>) -> Self { Self(value.into().into()) } }
impl License    { pub fn new(value: impl Into<String>) -> Self { Self(value.into().into()) } }
impl Name       { pub fn new(value: impl Into<String>) -> Self { Self(value.into().into()) } }
impl Pattern    { pub fn new(value: impl Into<String>) -> Self { Self(value.into().into()) } }
impl Url        { pub fn new(value: impl Into<String>) -> Self { Self(value.into().into()) } }
impl Version    { pub fn new(value: impl Into<String>) -> Self { Self(value.into().into()) } }

impl Category   { pub fn as_str(&self) -> &str { &self.0 } }
impl Edition    { pub fn as_str(&self) -> &str { &self.0 } }
impl License    { pub fn as_str(&self) -> &str { &self.0 } }
impl Name       { pub fn as_str(&self) -> &str { &self.0 } }
impl Pattern    { pub fn as_str(&self) -> &str { &self.0 } }
impl Url        { pub fn as_str(&self) -> &str { &self.0 } }
impl Version    { pub fn as_str(&self) -> &str { &self.0 } }

impl<'de> Deserialize<'de> for Category { fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> { String::deserialize(d).map(|s| Self(s.into())) } }
impl<'de> Deserialize<'de> for Edition  { fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> { String::deserialize(d).map(|s| Self(s.into())) } }
impl<'de> Deserialize<'de> for License  { fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> { String::deserialize(d).map(|s| Self(s.into())) } }
impl<'de> Deserialize<'de> for Name     { fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> { String::deserialize(d).map(|s| Self(s.into())) } }
impl<'de> Deserialize<'de> for Pattern  { fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> { String::deserialize(d).map(|s| Self(s.into())) } }
impl<'de> Deserialize<'de> for Url      { fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> { String::deserialize(d).map(|s| Self(s.into())) } }
impl<'de> Deserialize<'de> for Version  { fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> { String::deserialize(d).map(|s| Self(s.into())) } }

impl Serialize for Category { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { self.0.serialize(s) } }
impl Serialize for Edition  { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { self.0.serialize(s) } }
impl Serialize for License  { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { self.0.serialize(s) } }
impl Serialize for Name     { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { self.0.serialize(s) } }
impl Serialize for Pattern  { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { self.0.serialize(s) } }
impl Serialize for Url      { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { self.0.serialize(s) } }
impl Serialize for Version  { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { self.0.serialize(s) } }

impl AsRef<str> for Category    { fn as_ref(&self) -> &str { self.as_str() } }
impl AsRef<str> for Edition     { fn as_ref(&self) -> &str { self.as_str() } }
impl AsRef<str> for License     { fn as_ref(&self) -> &str { self.as_str() } }
impl AsRef<str> for Name        { fn as_ref(&self) -> &str { self.as_str() } }
impl AsRef<str> for Pattern     { fn as_ref(&self) -> &str { self.as_str() } }
impl AsRef<str> for Url         { fn as_ref(&self) -> &str { self.as_str() } }
impl AsRef<str> for Version     { fn as_ref(&self) -> &str { self.as_str() } }

impl Borrow<str> for Category   { fn borrow(&self) -> &str { self.as_str() } }
impl Borrow<str> for Edition    { fn borrow(&self) -> &str { self.as_str() } }
impl Borrow<str> for License    { fn borrow(&self) -> &str { self.as_str() } }
impl Borrow<str> for Name       { fn borrow(&self) -> &str { self.as_str() } }
impl Borrow<str> for Pattern    { fn borrow(&self) -> &str { self.as_str() } }
impl Borrow<str> for Url        { fn borrow(&self) -> &str { self.as_str() } }
impl Borrow<str> for Version    { fn borrow(&self) -> &str { self.as_str() } }

impl Debug for Category { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.as_str(), fmt) } }
impl Debug for Edition  { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.as_str(), fmt) } }
impl Debug for License  { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.as_str(), fmt) } }
impl Debug for Name     { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.as_str(), fmt) } }
impl Debug for Pattern  { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.as_str(), fmt) } }
impl Debug for Publish  { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { match self { Publish::Enabled(enabled) => Debug::fmt(enabled, fmt), Publish::Registries(reg) => Debug::fmt(reg, fmt) } } }
impl Debug for Url      { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.as_str(), fmt) } }
impl Debug for Version  { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.as_str(), fmt) } }

impl Deref for Category { fn deref(&self) -> &str { &self.0 } type Target = str; }
impl Deref for Edition  { fn deref(&self) -> &str { &self.0 } type Target = str; }
impl Deref for License  { fn deref(&self) -> &str { &self.0 } type Target = str; }
impl Deref for Name     { fn deref(&self) -> &str { &self.0 } type Target = str; }
impl Deref for Pattern  { fn deref(&self) -> &str { &self.0 } type Target = str; }
impl Deref for Url      { fn deref(&self) -> &str { &self.0 } type Target = str; }
impl Deref for Version  { fn deref(&self) -> &str { &self.0 } type Target = str; }

impl Default for Edition { fn default() -> Self { Edition::new("2015") } }
impl Default for Publish { fn default() -> Self { Publish::Enabled(true) } }

impl Display for Category   { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(self.as_str(), fmt) } }
impl Display for Edition    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(self.as_str(), fmt) } }
impl Display for License    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(self.as_str(), fmt) } }
impl Display for Name       { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(self.as_str(), fmt) } }
impl Display for Pattern    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(self.as_str(), fmt) } }
impl Display for Publish    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { match self { Publish::Enabled(enabled) => Display::fmt(enabled, fmt), Publish::Registries(reg) => Debug::fmt(reg, fmt) } } }
impl Display for Url        { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(self.as_str(), fmt) } }
impl Display for Version    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(self.as_str(), fmt) } }

impl PartialEq<bool> for Publish { fn eq(&self, other: &bool   ) -> bool { match self { Publish::Enabled(enabled) => *enabled == *other, _ => false } } }
impl PartialEq<Publish> for bool { fn eq(&self, other: &Publish) -> bool { match other { Publish::Enabled(enabled) => *enabled == *self, _ => false } } }

impl PartialEq<str> for Category    { fn eq(&self, other: &str) -> bool { &**self == other } }
impl PartialEq<str> for Edition     { fn eq(&self, other: &str) -> bool { &**self == other } }
impl PartialEq<str> for License     { fn eq(&self, other: &str) -> bool { &**self == other } }
impl PartialEq<str> for Name        { fn eq(&self, other: &str) -> bool { &**self == other } }
impl PartialEq<str> for Pattern     { fn eq(&self, other: &str) -> bool { &**self == other } }
impl PartialEq<str> for Url         { fn eq(&self, other: &str) -> bool { &**self == other } }
impl PartialEq<str> for Version     { fn eq(&self, other: &str) -> bool { &**self == other } }

impl PartialEq<&str> for Category   { fn eq(&self, other: &&str) -> bool { &**self == *other } }
impl PartialEq<&str> for Edition    { fn eq(&self, other: &&str) -> bool { &**self == *other } }
impl PartialEq<&str> for License    { fn eq(&self, other: &&str) -> bool { &**self == *other } }
impl PartialEq<&str> for Name       { fn eq(&self, other: &&str) -> bool { &**self == *other } }
impl PartialEq<&str> for Pattern    { fn eq(&self, other: &&str) -> bool { &**self == *other } }
impl PartialEq<&str> for Url        { fn eq(&self, other: &&str) -> bool { &**self == *other } }
impl PartialEq<&str> for Version    { fn eq(&self, other: &&str) -> bool { &**self == *other } }

impl PartialEq<Category > for str { fn eq(&self, other: &Category  ) -> bool { self == &**other } }
impl PartialEq<Edition  > for str { fn eq(&self, other: &Edition   ) -> bool { self == &**other } }
impl PartialEq<License  > for str { fn eq(&self, other: &License   ) -> bool { self == &**other } }
impl PartialEq<Name     > for str { fn eq(&self, other: &Name      ) -> bool { self == &**other } }
impl PartialEq<Pattern  > for str { fn eq(&self, other: &Pattern   ) -> bool { self == &**other } }
impl PartialEq<Url      > for str { fn eq(&self, other: &Url       ) -> bool { self == &**other } }
impl PartialEq<Version  > for str { fn eq(&self, other: &Version   ) -> bool { self == &**other } }

impl PartialEq<Category > for &str { fn eq(&self, other: &Category  ) -> bool { *self == &**other } }
impl PartialEq<Edition  > for &str { fn eq(&self, other: &Edition   ) -> bool { *self == &**other } }
impl PartialEq<License  > for &str { fn eq(&self, other: &License   ) -> bool { *self == &**other } }
impl PartialEq<Name     > for &str { fn eq(&self, other: &Name      ) -> bool { *self == &**other } }
impl PartialEq<Pattern  > for &str { fn eq(&self, other: &Pattern   ) -> bool { *self == &**other } }
impl PartialEq<Url      > for &str { fn eq(&self, other: &Url       ) -> bool { *self == &**other } }
impl PartialEq<Version  > for &str { fn eq(&self, other: &Version   ) -> bool { *self == &**other } }
