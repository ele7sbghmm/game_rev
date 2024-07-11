use std::fs::OpenOptions;
use std::io::{self, Read};
// use nom::{sequence::Tuple, IResult};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path: String = args[0].clone();

    let mut buf: Vec<u8> = vec![];
    let _ = OpenOptions::new()
        .read(true)
        .open(&path)
        .expect(&format!("Couldn't open {}", &path))
        .read_to_end(&mut buf);
    println!("Hello, world!");
}
