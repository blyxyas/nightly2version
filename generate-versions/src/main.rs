use anyhow::Result;
use git2::{self, Repository};
use std::{
    fmt::write,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
    str::FromStr,
};
use version::Version;

// I KNOW THIS IS A MESS
fn main() -> Result<()> {
    let rust_path = Path::new("rust");
    let repo;
    if !rust_path.exists() {
        repo = Repository::clone("https://github.com/rust-lang/rust", rust_path)?;
    } else {
        repo = Repository::open(rust_path)?;
    }

    let mut generated_rs = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("../src/generated.rs")
        .unwrap();

    generated_rs.set_len(0)?;

    let mut arms = Vec::with_capacity(120);
    generated_rs.write_all(b"#[inline]\npub(crate) fn correlations_dates(minor: u16, patch: u16) -> anyhow::Result<i64> {\nmatch minor {\n")?;

    repo.tag_foreach(|oid, bnames| {
        let tag = std::str::from_utf8(&bnames[10..]).unwrap();
        if tag.split('.').count() != 3 {
            return true;
        }
        let version_parse = match Version::from_str(tag) {
            Ok(s) if s.minor != 0 => s,
            _ => return true,
        };

        let find_commit_time = repo.find_tag(oid).unwrap().tagger().unwrap().when();

        if version_parse.patch != 0 {
            arms.push(format!(
                "{} if patch == {} => {{Ok({})}},",
                version_parse.minor,
                version_parse.patch,
                find_commit_time.seconds()
            ));
        } else {
            arms.push(format!(
                "{} => {{Ok({})}},",
                version_parse.minor,
                find_commit_time.seconds()
            ));
        }
        true
    })?;

    arms.reverse();
    writeln!(generated_rs, "{}", arms.join("\n")).unwrap();
    arms.clear();

    writeln!(
        generated_rs,
        "_ => anyhow::bail!(\"Version {{}}.{{}}not found\", minor, patch)"
    )
    .unwrap();
    writeln!(generated_rs, "}} }}\n").unwrap();
    writeln!(generated_rs, "#[inline]\npub(crate) fn correlations_commits(minor: u16, patch: u16) -> anyhow::Result<&'static str> {{\nmatch minor {{\n")?;

    repo.tag_foreach(|oid, bnames| {
        let tag = std::str::from_utf8(&bnames[10..]).unwrap();
        if tag.split('.').count() != 3 {
            return true;
        }
        let version_parse = match Version::from_str(tag) {
            Ok(s) if s.minor != 0 => s,
            _ => return true,
        };

        if version_parse.patch != 0 {
            arms.push(format!(
                "{} if patch == {} => {{Ok(\"{}\")}},",
                version_parse.minor,
                version_parse.patch,
                oid.to_string()
            ));
        } else {
            arms.push(format!(
                "{} => {{Ok(\"{}\")}},",
                version_parse.minor,
                oid.to_string()
            ));
        }
        true
    })?;

    arms.reverse();
    writeln!(generated_rs, "{}", arms.join("\n")).unwrap();
    arms.clear();

    writeln!(
        generated_rs,
        "_ => anyhow::bail!(\"Version {{}}.{{}}not found\", minor, patch)"
    )
    .unwrap();

    writeln!(generated_rs, "}} }}").unwrap();

    writeln!(
        generated_rs,
        "#[inline]\npub(crate) fn version_exists(minor: u16) -> bool {{\nmatch minor {{\n"
    )?;

    repo.tag_foreach(|oid, bnames| {
        let tag = std::str::from_utf8(&bnames[10..]).unwrap();
        if tag.split('.').count() != 3 {
            return true;
        }
        let version_parse = match Version::from_str(tag) {
            Ok(s) if s.minor != 0 => s,
            _ => return true,
        };
        arms.push(format!(" | {}", version_parse.minor));
        true
    })?;
    arms.reverse();
    arms.dedup();
    arms.push("=> true,\n_ => false".into());
    writeln!(generated_rs, "{}", arms.join("\n")).unwrap();
    writeln!(generated_rs, "}} }}").unwrap();

    format_with_fmt()?;
    Ok(())
}

fn format_with_fmt() -> Result<()> {
    Command::new("rustfmt")
        .arg("../src/generated.rs")
        .status()?;

    Ok(())
}
