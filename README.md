# `nightly2version`

This is a very lightweight, **very fast**, `#[no_std]`-compatible Rust crate destined to converting from a Rust version to a timestamp and vice-versa (Along other kinds of version-checking shenanigans)

```rust
use nightly2version::RustVersion;

fn main() {

    assert_eq!(RustVersion::new("1.80.1").exists_in_stable(), false); // Version does not exist
    assert_eq!(RustVersion::new("1.80.0").exists_in_stable(), true); // Version does exist

    let timestamp = RustVersion::new("1.80.0").to_timestamp().unwrap();
    assert_eq!(timestamp, 1721908957);

    let version = RustVersion::timestamp_to_version(timestamp).unwrap();
    assert_eq!(version.exists_in_stable(), true);
    assert_eq!(
        version,
        RustVersion {
            major: 1,
            minor: 80,
            patch: 0
        }
    );
}
```

You can convert from a timestamp to a [`RustVersion`], change the minor, check if the mutated version exists **and then** get a timestamp from that, in just a few method calls. It's really great!

## Versioning

This crate doesn't follow normal crate versioning conventions. `nightly2version` gets updated on a 6-week schedule, just after Rust gets a new version. Sometimes a change in the crate gets included in that update. Compatibility is a priority and will be maintained. For new `nightly2version` versions that needs to get released before the 6-week schedule, you can find those in the last number of the version number, just after the dash.

```rust,no_compile
   "1.80.0-1" ;
// <RUST MAJOR>.<RUST MINOR>.<RUST PATCH>-<CRATE REVISION>
```