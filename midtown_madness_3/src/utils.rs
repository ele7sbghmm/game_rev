use std::{fs, io::Read};

pub fn read_file(mut buf: &mut Vec<u8>, path: String) {
    let _ = fs::OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap()
        .read_to_end(&mut buf);
}
