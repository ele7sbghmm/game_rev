use std::{
    fs,
    io::{Read, Write},
};

pub fn read_file(mut buf: &mut Vec<u8>, path: String) {
    let _ = fs::OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap()
        .read_to_end(&mut buf);
}

pub fn write_file(buffers: Vec<Vec<u8>>, path: String) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    // let _ = buffers.iter().map(|buffer| {
    //     let _ = file.write(&buffer);
    // });
    for buffer in buffers.iter() {
        let _ = file.write(buffer);
    }
}
