#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(let_chains)]

use core::str;

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
        splits.all(|s| { count +=1; s.chars().all(|c| c.is_digit(10))}) && count == 3
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

struct RustVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl RustVersion {
    pub fn new<V: ToVersion>(version: V) -> Self {
        let (major, minor, patch) = version.to_version();
        Self {
            major,
            minor,
            patch
        }
    }

    pub fn to_timestamp<V: ToVersion>(&self) -> Result<i64> {
        if core::intrinsics::unlikely(self.major == 0) {
            unimplemented!("Betas (< 1.0.0) are not supported versions for now");
        }

        generated::correlations_dates(self.minor, 0)
    }

    // pub fn version_to_nightly<V: ToVersion>(version: V) -> Result<NaiveDate> {
    //     Ok()
    // }

    #[inline]
    pub fn to_commit_id<V: ToVersion>(&self) -> Result<&'static str> {
        let version = version.to_version();
        generated::correlations_commits(version.1, version.2)
    }

    pub fn exists(&self) -> bool {
        generated::exists(minor, patch);
    }
}

#[test]
fn test() {
    dbg!("1.80.0".to_version());
}

// #[cfg(feature = "version_crate")]
// impl From<Version> for
