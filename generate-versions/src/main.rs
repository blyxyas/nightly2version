use std::{fs::{File, OpenOptions}, io::Write, path::Path, str::FromStr};
use git2::{self, Repository};
use anyhow::Result;
use version::Version;

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

    generated_rs.write_all(b"#[inline]\npub(crate) fn correlations_generated_rs(minor: u16, patch: u16) -> Result<u128> {\nmatch minor {\n")?;

    repo.tag_foreach(|oid, bnames| {
        let tag = std::str::from_utf8(&bnames[10..]).unwrap();
        dbg!(tag);
        if tag.split('.').count() != 3 {
            return true
        }
        let version_parse = match Version::from_str(tag) {
            Ok(s) if s.minor != 0 => s,
            _ => return true
        };

        if version_parse.patch != 0 {
            let find_commit_time = repo.find_tag(oid).unwrap().tagger().unwrap().when();
            writeln!(generated_rs, "{} if patch == {} => {{{}}},", version_parse.minor, version_parse.patch, find_commit_time.seconds()).unwrap();
        } else {
            let find_commit_time = repo.find_tag(oid).unwrap().tagger().unwrap().when();
            writeln!(generated_rs, "{} => {{{}}},", version_parse.minor, find_commit_time.seconds()).unwrap();
        }

        
        // let tag_parsed = Version::from_str(tag).unwrap();
        // // if tag_names.get(i + 1).is_some_and(|new_version| Version::from_str(new_version).unwrap().patch - tag_parsed.patch == 1) {
        //     // We're in a patched version
        //     format!("{} if patch == {} =>", tag_parsed.minor, tag_parsed.patch);
        //     let commit = repo.tag_foreach(cb)
        // }
        // format!("{} => ")
        true
    })?;
    
    writeln!(generated_rs, "_ => bail!(\"Version {{}}.{{}}not found\", minor, patch)").unwrap();
    writeln!(generated_rs, "}} }}\n").unwrap();
    writeln!(generated_rs, "#[inline]\npub(crate) fn correlations_commits(minor: u16, patch: u16) -> anyhow::Result<&'static str> {{\nmatch minor {{\n")?;


    repo.tag_foreach(|oid, bnames| {
        let tag = std::str::from_utf8(&bnames[10..]).unwrap();
        dbg!(tag);
        if tag.split('.').count() != 3 {
            return true
        }
        let version_parse = match Version::from_str(tag) {
            Ok(s) if s.minor != 0 => s,
            _ => return true
        };

        if version_parse.patch != 0 {
            writeln!(generated_rs, "{} if patch == {} => {{\"{}\"}},", version_parse.minor, version_parse.patch, oid.to_string()).unwrap();
        } else {
            writeln!(generated_rs, "{} => {{\"{}\"}},", version_parse.minor, oid.to_string()).unwrap();   
        }
        
        // let tag_parsed = Version::from_str(tag).unwrap();
        // // if tag_names.get(i + 1).is_some_and(|new_version| Version::from_str(new_version).unwrap().patch - tag_parsed.patch == 1) {
        //     // We're in a patched version
        //     format!("{} if patch == {} =>", tag_parsed.minor, tag_parsed.patch);
        //     let commit = repo.tag_foreach(cb)
        // }
        // format!("{} => ")
        true
    })?;
    writeln!(generated_rs, "_ => anyhow::bail!(\"Version {{}}.{{}}not found\", minor, patch)").unwrap();
    writeln!(generated_rs, "}} }}").unwrap();
    Ok(())
}
