#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(let_chains)]

use core::str;
use std::ops::Not;

use anyhow::Result;
pub use chrono::DateTime;
use chrono::{NaiveDate, TimeZone};

mod generated;

pub trait ToVersion {
    fn to_version(&self) -> (u16, u16, u16);
    fn is_valid_version(&self) -> bool;
}

impl ToVersion for &str {
    fn to_version(&self) -> (u16, u16, u16) {
        let mut split = self.split('.');
        // https://stackoverflow.com/questions/16826422/c-most-efficient-way-to-convert-string-to-int-faster-than-atoi
        (
            if core::intrinsics::likely(
                split.next().expect(
                    "The split had length 0, are you sure you inputted a version (e.g. 1.0.0)?",
                ) == "1",
            ) {
                1
            } else {
                0
            },
            split
                .next()
                .expect("Expected 3 componenents, got only 1")
                .parse()
                .unwrap(),
            split
                .next()
                .expect("Expected 3 components, got only 2")
                .parse()
                .unwrap(),
        )
    }

    fn is_valid_version(&self) -> bool {
        let mut splits = self.split('.')/*.collect::<Vec<&str>>()*/;
        let mut count = 0;
        splits.all(|s| {
            count += 1;
            s.chars().all(|c| c.is_digit(10))
        }) && count == 3
    }
}

impl ToVersion for (u16, u16, u16) {
    #[inline(always)]
    fn to_version(&self) -> (u16, u16, u16) {
        *self
    }

    #[inline(always)]
    fn is_valid_version(&self) -> bool {
        true
    }
}

impl ToVersion for String {
    #[inline(always)]
    fn to_version(&self) -> (u16, u16, u16) {
        self.as_str().to_version()
    }

    fn is_valid_version(&self) -> bool {
        self.as_str().is_valid_version()
    }
}

impl ToVersion for &[u8] {
    #[inline]
    fn to_version(&self) -> (u16, u16, u16) {
        std::str::from_utf8(self)
            .expect("Couldn't create String from")
            .to_version()
    }
    fn is_valid_version(&self) -> bool {
        std::str::from_utf8(self)
            .expect("Couldn't create String from")
            .is_valid_version()
    }
}

// const fn to_version(a: &str) {
// }

#[derive(PartialEq, Debug)]
pub struct RustVersion {
    /// The MAJOR number in [SemVer snytax](https://semver.org/) (e.g. **MAJOR**.MINOR.PATCH)
    pub major: u16,
    /// The MINOR number in [SemVer snytax](https://semver.org/) (e.g. MAJOR.**MINOR**.PATCH)
    pub minor: u16,
    /// The PATCH number in [SemVer snytax](https://semver.org/) (e.g. MAJOR.MINOR.**PATCH**)
    pub patch: u16,
}

impl RustVersion {
    #[inline]
    pub fn new<V: ToVersion>(version: V) -> Self {
        let (major, minor, patch) = version.to_version();
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn to_timestamp(&self) -> Result<i64> {
        if core::intrinsics::unlikely(self.major == 0) {
            unimplemented!("Betas (< 1.0.0) are not supported versions for now");
        }

        generated::correlations_dates(self.minor, 0)
    }

    #[inline(always)]
    pub fn to_commit_id(&self) -> Result<&'static str> {
        generated::correlations_commits(self.major, self.minor, self.patch)
    }

    #[inline(always)]
    pub fn exists(&self) -> bool {
        generated::version_exists(self.minor, self.patch)
    }

    #[inline]
    pub fn timestamp_to_version(timestamp: i64) -> Result<Self> {
        Ok(Self::new(generated::timestamp_ranges(timestamp)?))
    }
}

#[test]
fn test() {
    assert!(RustVersion::new("1.80.1").exists().not());
    assert!(RustVersion::new("1.80.0").exists());
    
    let timestamp = RustVersion::new("1.80.0").to_timestamp().unwrap();
    assert_eq!(timestamp, 1721908957);
    
    let version = RustVersion::timestamp_to_version(timestamp - 10).unwrap();
    assert!(version.exists());
    assert_eq!(version, RustVersion {
        major: 1,
        minor: 80,
        patch: 0
    });
}

// #[cfg(feature = "version_crate")]
// impl From<Version> for
