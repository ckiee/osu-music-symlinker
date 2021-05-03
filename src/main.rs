use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use pbr::ProgressBar;
use regex::Regex;
use std::convert::TryInto;
use std::{env, fs, fs::File, io::prelude::*, os::unix::fs::symlink, path::Path};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        bail!(
            r"Usage: osu-music-symlinker INPUT-FOLDER OUTPUT-FOLDER

There are no options."
        );
    }

    let songs_path = Path::new(&args[1]);
    let new_root = Path::new(&args[2]);

    assert!(songs_path.is_dir());
    assert!(new_root.is_dir());

    let songs_entries = fs::read_dir(songs_path)?.collect::<Vec<_>>();
    let mut pb = ProgressBar::new(songs_entries.len().try_into()?);
    for entry in songs_entries {
        let entry = entry?;
        handle_mapset(&entry.path(), &new_root).ok();
        pb.inc();
    }

    Ok(())
}

fn handle_mapset(dir_path: &Path, new_root: &Path) -> Result<()> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r#"AudioFilename:\s*(?P<filename>.+)"#).unwrap();
    }
    assert!(dir_path.is_dir());
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        if entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow!("into_string fail"))?
            .ends_with(".osu")
        {
            let mut file = File::open(&entry.path())?;
            // All the beatmap parsing crates I've tried didn't fit this use case well. (failed to parse some beatmaps, slow, missing attrs, etc)
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let cap = REGEX.captures(&content).ok_or(anyhow!("REGEX.captures"))?;
            let name = &cap["filename"].trim();
            let mut audio_path = entry.path().clone();
            audio_path.set_file_name(name);
            symlink(
                &audio_path,
                new_root.join(dir_path.file_name().ok_or(anyhow!("file_name"))?),
            )?;
        }
    }
    Ok(())
}
