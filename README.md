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
