#![doc = include_str!("../README.md")]
#![allow(internal_features)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::{
    cmp::{self, Ordering},
    fmt, str,
};

mod generated;

pub trait ToVersion {
    /// Returns the parsed version of the input without taking the input. The
    /// return type `(u16, u16, u16)` is formatted like `(MAJOR, MINOR, PATCH)`.
    fn to_version(&self) -> (u16, u16, u16);
    /// Validates if a string would be a valid version. This method does not
    /// check if the version exists, checkout [`RustVersion`] for checking
    /// if a version exists.
    fn is_valid_version(&self) -> bool;
}

impl ToVersion for &str {
    fn to_version(&self) -> (u16, u16, u16) {
        let mut split = self.split('.');
        // https://stackoverflow.com/questions/16826422/c-most-efficient-way-to-convert-string-to-int-faster-than-atoi
        (
            if split
                .next()
                .expect("The split had length 0, are you sure you inputted a version (e.g. 1.0.0)?")
                == "1"
            {
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

#[cfg(feature = "std")]
impl ToVersion for std::string::String {
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
        core::str::from_utf8(self)
            .expect("Couldn't create String from")
            .to_version()
    }
    fn is_valid_version(&self) -> bool {
        core::str::from_utf8(self)
            .expect("Couldn't create String from")
            .is_valid_version()
    }
}

// FIXME: Remember to change that `125` to 126 when the next version comes around.
/// Return an static array of length `125` with the type structure `((MAJOR, MINOR, PATCH),
/// timestamp)`
///
/// As always, the timestamp is in `i64` and starts from the [Unix Epoch](https://doc.rust-lang.org/stable/std/time/constant.UNIX_EPOCH.html)
pub fn all_versions() -> [((u16, u16, u16), i64); 125] {
    generated::all_versions()
}

#[derive(PartialEq, Eq, Debug)]
pub struct RustVersion {
    /// The MAJOR number in [SemVer snytax](https://semver.org/) (e.g. **MAJOR**.MINOR.PATCH).
    pub major: u16,
    /// The MINOR number in [SemVer snytax](https://semver.org/) (e.g. MAJOR.**MINOR**.PATCH).
    pub minor: u16,
    /// The PATCH number in [SemVer snytax](https://semver.org/) (e.g. MAJOR.MINOR.**PATCH**).
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

    /// Returns the associated timestamp with the version. This is done by getting what commit.
    pub fn to_timestamp(&self) -> Result<i64, &'static str> {
        if self.major == 0 {
            unimplemented!("Betas (< 1.0.0) are not supported versions for now");
        }

        generated::correlations_dates(self.minor, 0)
    }

    /// Returns the commit id for the Rust Version, if it exists. Returns an
    /// error if it doesn't exist (likely because the version doesn't exist,
    /// or these isn't a commit tagged as the release; unlikely).
    #[inline(always)]
    pub fn to_commit_id(&self) -> Result<&'static str, &'static str> {
        generated::correlations_commits(self.major, self.minor, self.patch)
    }

    #[inline(always)]
    /// Returns true if the version has ever been been declared.
    /// For example, the version 1.77.2 would return `true` while the version
    /// 1.77.9 would return `false`, because there never was a version 1.77.9.
    ///
    /// Note that the nightly or beta version would output `false`, as it's not in stable yet.
    ///
    /// ```rust
    /// # fn main() {
    /// # use nightly2version::RustVersion;
    /// assert_eq!(RustVersion::new("1.77.2").exists_in_stable(), true);
    /// assert_eq!(RustVersion::new("1.101.9000").exists_in_stable(), false)
    /// # }
    /// ```
    pub fn exists_in_stable(&self) -> bool {
        generated::version_exists(self.minor, self.patch)
    }

    /// Converts a timestamp to a version. This method can return a
    /// [`RustVersion`] up to two MINOR versions highers than the current
    /// stable one to account for beta and nightly versions.
    #[inline]
    pub fn timestamp_to_version(timestamp: i64) -> Result<Self, &'static str> {
        match generated::timestamp_ranges(timestamp) {
            Ok(s) => Ok(RustVersion::new(s.to_version())),
            Err(e) => {
                // Calculate nightly
                let latest = generated::all_versions()[0];
                // Check if three weeks ago we were stable
                if timestamp - (60 * 60 * 24 * 21) <= latest.1 {
                    // We're in beta
                    return Ok(RustVersion {
                        major: latest.0 .0,
                        minor: latest.0 .1 + 1,
                        patch: latest.0 .2,
                    });
                } else if timestamp - (60 * 60 * 24 * 42) <= latest.1 {
                    // Were we 6 weeks ago or more recently?
                    // Then we're in nightly
                    return Ok(RustVersion {
                        major: latest.0 .0,
                        minor: latest.0 .1 + 2,
                        patch: latest.0 .2,
                    });
                }

                Err(e)
            }
        }
    }
}

impl fmt::Display for RustVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl cmp::PartialOrd for RustVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for RustVersion {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.major.cmp(&other.major).then(
            self.minor
                .cmp(&other.minor)
                .then(self.patch.cmp(&other.patch)),
        )
    }
}

#[cfg(test)]
mod tests {
    use ::core::ops::Not;

    use crate::*;
    #[test]
    fn test() {
        assert!(RustVersion::new("1.80.999").exists_in_stable().not());
        assert!(RustVersion::new("1.80.0").exists_in_stable());

        let timestamp = RustVersion::new("1.80.0").to_timestamp().unwrap();
        assert_eq!(timestamp, 1721908957);

        let version = RustVersion::timestamp_to_version(timestamp).unwrap();
        assert!(version.exists_in_stable());
        assert_eq!(
            version,
            RustVersion {
                major: 1,
                minor: 80,
                patch: 0
            }
        );
    }

    #[test]
    fn impls() {
        assert!(RustVersion::new("1.80.1") < RustVersion::new("1.81.0"));
        assert!(RustVersion::new("1.80.1") == RustVersion::new("1.80.1"));
        assert!(RustVersion::new("1.80.1") != RustVersion::new("1.80.9"));
        assert!(RustVersion::new("1.80.1") != RustVersion::new("2.80.1"));
        assert!(RustVersion::new("1.1.0") >= RustVersion::new("1.0.0"));
    }
}
