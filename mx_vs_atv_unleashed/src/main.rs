#![allow(unused)]

use std::path::PathBuf;

mod exts {
    pub mod pak;
    pub mod res;
    pub mod utils;
    pub mod xbr;
}
mod ext;

use exts::utils::read_file;
use exts::{pak::parse_pak, res::parse_res};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let p = match args.len() {
        1 => "/Users/timmeh/Desktop/",
        2 => &args[1],
        _ => panic!(),
    };
    let (out_path, path) = (PathBuf::from(p.replace(".", "_")), PathBuf::from(p));

    let mut buf: Vec<u8> = vec![];
    let _ = read_file(&mut buf, path.clone());
    let clean_input = buf.as_slice();

    // parse_pak(path.with_file_name(""), clean_input);
    parse_res(out_path, clean_input);

    println!("Hello, world!");
}
