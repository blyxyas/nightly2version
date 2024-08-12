#![allow(internal_features)]
#![feature(core_intrinsics)]

use core::str;

pub use chrono::DateTime;
use chrono::TimeZone;

mod generated;

trait ToVersion {
    fn to_version(&self) -> (u16, u16, u16);
}


impl ToVersion for &str {
    fn to_version(&self) -> (u16, u16, u16) {
        let mut split = self.split('.');
        // https://stackoverflow.com/questions/16826422/c-most-efficient-way-to-convert-string-to-int-faster-than-atoi
        (
            if core::intrinsics::likely(split.next().unwrap() == "1") { 1 } else {0}, // For now const
            unsafe {split.next().unwrap_unchecked().parse().unwrap() },
            unsafe { split.next().unwrap_unchecked().parse().unwrap() },
        )
    }
}

impl ToVersion for (u16, u16, u16) {
    #[inline(always)]
    fn to_version(&self) -> (u16, u16, u16) {
        *self
    }
}

impl ToVersion for String {
    #[inline(always)]
    fn to_version(&self) -> (u16, u16, u16) {
        self.as_str().to_version()
    }
}

impl ToVersion for &[u8] {
    #[inline]
    fn to_version(&self) -> (u16, u16, u16) {
        std::str::from_utf8(self).expect("Couldn't create String from").to_version()
    }
}

// const fn to_version(a: &str) {
// }

pub fn nightly_to_version<V: AsRef<str>>(nightly: V) -> () {
    let s = nightly.as_ref();
    let (major, minor, patch) = s.to_version();
    
    if core::intrinsics::unlikely(major == 0) {
        unimplemented!("Betas (< 1.0.0) are not supported versions for now");
    }

    let mut prove = 60;
    let mut lower_prove = 61;
    while generated::correlations_generated_rs(minor, patch)
}

#[inline]
pub fn version_to_commits(version: (u16, u16, u16)) -> &'static str {
    generated::correlations_commits(version.1, version.2)
}

#[test]
fn test() {
    dbg!("1.80.0".to_version());
}

// #[cfg(feature = "version_crate")]
// impl From<Version> for
