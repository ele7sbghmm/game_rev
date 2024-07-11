#![allow(unused)]
use std::collections;
use std::fs::{File, OpenOptions, rename, read_dir, create_dir_all};
use std::ops::{Index, IndexMut};
use std::io::{self, prelude::*, Read, Write};
use std::path::{Path, PathBuf};

fn get_hashes<'a>(path: String) -> Option<collections::HashMap<String, String>> {
    let mut hashes = collections::HashMap::new();

    let mut file: File = OpenOptions::new().read(true).open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    for line_res in lines {
        if let Ok(line) = line_res {
            let mut b = line.split_whitespace();
            let path = b.next().unwrap().to_string();
            let hash = b.next().unwrap().to_string();
            hashes.insert(hash, path);
        } else if let Err(e) = line_res {
            panic!(" line result {} ", e);
        }
    }
    Some(hashes)
}


fn visit_dirs(dir: &Path, og: &Path, hash_map: &collections::HashMap<String, String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, og, hash_map)?;
            } else {
                let lets = swap_hash_filename(hash_map, path.to_path_buf());
                if let Some(p) = lets {
                    let new = og.join(p);
                    _ = create_dir_all(new.with_file_name(""));
                    _ = rename(path, new);
                }
            }
        }
    }
    Ok(())
}

fn swap_hash_filename(
    hash_map: &collections::HashMap<String, String>,
    path: PathBuf,
) -> Option<PathBuf> {
    if let Some(stem) = path.file_stem().unwrap().to_str() {
        println!(" {:?}", stem);
        if hash_map.contains_key(stem) {
            println!(" ^^^^^^^^^^^^^^ ");
            return Some(PathBuf::from(&hash_map[stem]));
        };
    };
    None
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let hashes = get_hashes(args[2].clone()).unwrap();
    let path: &Path = &Path::new(&args[1]);
    _ = visit_dirs(path, path, &hashes);

    Ok(())
}
