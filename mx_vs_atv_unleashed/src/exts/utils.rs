use std::{
    fs,
    io::{Read, Write},
    //ffi::OsStr,
    path::PathBuf,
};

pub fn read_file(mut buf: &mut Vec<u8>, path: PathBuf) {
    let _ = fs::OpenOptions::new()
        .read(true)
        .open(path)
        .expect("!!! read failed")
        .read_to_end(&mut buf);
}
pub fn write_file(buffers: Vec<Vec<u8>>, path: PathBuf) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("!!! write failed");

    for buf in buffers.iter() {
        let _ = file.write(buf.as_slice());
    }
}

pub fn path_windows_to_posix(path: PathBuf) -> PathBuf {
    PathBuf::from(path.to_string_lossy().replace("\\", "/"))
}
