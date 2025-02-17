use anyhow::Result;
use git2::{self, Repository};
use std::{
    collections::HashMap, fs::OpenOptions, io::Write, path::Path, process::Command, str::FromStr,
};
use version::Version;

// I KNOW THIS IS A MESS
// THIS MAY BE THE WORST CODE EVER CONCEIVED BY ANY HUMAN EVER
// IF YOURE A CONTRIBUTOR DONT EVEN TRY TO LOOK AT THIS, IT *JUST WORKS*
// SO WHY BOTHER, IM "COMMITTING" (haha) SEVERAL DEADLY SINS HERE
fn main() -> Result<()> {
    let rust_path = Path::new("rust");
    let repo;
    if !rust_path.exists() {
        repo = Repository::clone("https://github.com/rust-lang/rust", rust_path)?;
    } else {
        repo = Repository::open(rust_path)?;
    }

    repo.find_remote("origin")?.fetch(&["master"], None, None)?;

    let mut generated_rs = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("../src/generated.rs")
        .unwrap();

    generated_rs.set_len(0)?;

    let mut arms = Vec::with_capacity(120);

    // TIMESTAMP

    generated_rs
        .write_all(b"#![allow(unreachable_patterns)]")
        .unwrap();
    writeln!(generated_rs, "#[inline]\npub(crate) fn correlations_dates(minor: u16, patch: u16) -> Result<i64, &'static str> {{\nmatch minor {{\n")?;

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
            let find_commit_time = repo.find_tag(oid).unwrap().tagger().unwrap().when();
            arms.push(format!(
                "{} if patch == {} => {{Ok({})}},",
                version_parse.minor,
                version_parse.patch,
                find_commit_time.seconds()
            ));
        } else {
            let find_commit_time = repo.find_tag(oid).unwrap().tagger().unwrap().when();
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

    writeln!(generated_rs, "_ => Err(\"Version not found\")").unwrap();
    writeln!(generated_rs, "}} }}\n").unwrap();

    // COMMITS

    writeln!(generated_rs, "#[inline]\npub(crate) fn correlations_commits(major: u16, minor: u16, patch: u16) -> Result<&'static str, &'static str> {{\nmatch minor {{\n")?;

    repo.tag_foreach(|oid, bnames| {
        let tag = std::str::from_utf8(&bnames[10..]).unwrap();
        if tag.split('.').count() != 3 {
            return true;
        }
        let version_parse = match Version::from_str(tag) {
            Ok(s) if s.minor != 0 => s,
            _ => return true,
        };

        let mut meow = String::new();
        if version_parse.patch != 0 {
            meow.push_str(&format!("if patch == {}", version_parse.patch))
        }
        if version_parse.major == 0 {
            writeln!(
                generated_rs,
                "{} if major == 0 => {{Ok(\"{}\")}},",
                version_parse.minor,
                // Meow not necessary
                oid.to_string()
            )
            .unwrap();
        } else {
            arms.push(format!(
                "{} {} => {{Ok(\"{}\")}},",
                version_parse.minor,
                meow,
                oid.to_string()
            ));
        }
        true
    })?;

    arms.reverse();
    writeln!(generated_rs, "{}", arms.join("\n")).unwrap();
    arms.clear();

    writeln!(generated_rs, "_ => Err(\"Version not found\")").unwrap();
    writeln!(generated_rs, "}} }}").unwrap();

    // VERSIONS EXIST

    writeln!(generated_rs, "#[inline]\npub(crate) fn version_exists(minor: u16, patch: u16) -> bool {{\nmatch minor {{\n")?;

    let mut how_many = 0;
    repo.tag_foreach(|_, bnames| {
        let tag = std::str::from_utf8(&bnames[10..]).unwrap();
        if tag.split('.').count() != 3 {
            return true;
        }
        let version_parse = match Version::from_str(tag) {
            Ok(s) if s.minor != 0 => s,
            _ => return true,
        };

        how_many += 1;
        arms.push(format!(
            "{} if patch == {} => true,",
            version_parse.minor, version_parse.patch,
        ));

        true
    })?;
    arms.reverse();
    arms.dedup();
    arms.push("_ => false".into());
    writeln!(generated_rs, "{}", arms.join("\n")).unwrap();
    writeln!(generated_rs, "}} }}").unwrap();
    arms.clear();

    // VERSIONS ARRAY

    writeln!(
        generated_rs,
        "#[inline]\npub(crate) fn all_versions() -> [((u16, u16, u16), i64);{how_many}] {{\n["
    )?;

    let mut meowmeowvec: Vec<((u32, u32, u32), i64)> = Vec::new();

    repo.tag_foreach(|oid, bnames| {
        let tag = std::str::from_utf8(&bnames[10..]).unwrap();
        if tag.split('.').count() != 3 {
            return true;
        }

        let version_parse = match Version::from_str(tag) {
            Ok(s) if s.minor != 0 => s,
            _ => return true,
        };

        let find_commit_time = match repo.find_tag(oid) {
            Ok(t) => t.tagger().unwrap().when(),
            _ => return true,
        };

        meowmeowvec.push((
            (
                version_parse.major,
                version_parse.minor,
                version_parse.patch,
            ),
            find_commit_time.seconds(),
        ));

        true
    })?;

    meowmeowvec.sort_by(|x, y| {
        x.0 .0
            .cmp(&y.0 .0)
            .then(x.0 .1.cmp(&y.0 .1))
            .then(x.0 .2.cmp(&y.0 .2))
    });

    meowmeowvec
        .iter()
        .for_each(|m| arms.push(format!("(({}, {}, {}), {})", m.0 .0, m.0 .1, m.0 .2, m.1)));

    arms.reverse();
    arms.dedup();
    writeln!(generated_rs, "{}", arms.join("\n,")).unwrap();
    writeln!(generated_rs, "] }}").unwrap();
    arms.clear();

    // HASHMAP RANGES

    writeln!(generated_rs, "#[inline]\npub(crate) fn timestamp_ranges(timestamp: i64) -> Result<(u16, u16, u16), &'static str> {{\nmatch timestamp - 1 {{\n")?;

    let mut hashmap = HashMap::new();

    repo.tag_foreach(|oid, bnames| {
        let tag = std::str::from_utf8(&bnames[10..]).unwrap();
        if tag.split('.').count() != 3 {
            return true;
        }

        let find_commit_time = match repo.find_tag(oid) {
            Ok(t) => t.tagger().unwrap().when(),
            _ => return true,
        };

        let version_parse = match Version::from_str(tag) {
            Ok(s) if s.minor != 0 => s,
            _ => return true,
        };

        hashmap.insert(
            (
                version_parse.major,
                version_parse.minor,
                version_parse.patch,
            ),
            find_commit_time.seconds(),
        );

        // arms.push(format!(
        //     "{}..{} => Ok(({}, {}, {})),",
        //     last_time.seconds(),
        //     find_commit_time.seconds(),
        //     version_parse.major,
        //     version_parse.minor,
        //     version_parse.patch,
        // ));

        true
    })?;

    let mut timesvec = hashmap.into_iter().collect::<Vec<((u32, u32, u32), i64)>>();
    timesvec.sort_by(|x, y| {
        x.0 .0
            .cmp(&y.0 .0)
            .then(x.0 .1.cmp(&y.0 .1))
            .then(x.0 .2.cmp(&y.0 .2))
    });

    // For the first one
    arms.push(format!(
        "..{} => Ok(({}, {}, {})),",
        timesvec[0].1, timesvec[0].0 .0, timesvec[0].0 .1, timesvec[0].0 .2,
    ));

    let mut last_ts = &timesvec[0].1;

    for (version, timestamp) in &timesvec[1..] {
        arms.push(format!(
            "{}..{} => Ok(({}, {}, {})),",
            last_ts, timestamp, version.0, version.1, version.2,
        ));

        last_ts = timestamp;
    }
    arms.reverse();
    arms.push(
        "_ => Err(\"Timestamp is not a version's release date, maybe it is the current version?\")"
            .into(),
    );
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
