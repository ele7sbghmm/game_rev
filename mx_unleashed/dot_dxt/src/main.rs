#![allow(unused)]

mod utils;
use utils::{read_file, write_file};
mod dxt;
use dxt::Dxt;
mod color;
mod texture;

use nom::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path: String = match args.len() {
        2 => args[2].clone(),
        _ => "/Users/timmeh/Desktop/d_dxt_xbox/sx14_d.dxt".to_string(),
    };
    dbg!(&path);

    let mut buf: Vec<u8> = vec![];
    let _ = read_file(&mut buf, path);
    let clean_input = buf.as_slice();

    let (input, dxt) = Dxt::from_bytes().parse(clean_input).unwrap();
    let last = dxt.textures[dxt.textures.len() - 1].clone();
    let third = dxt.textures[5].clone();
    let _ = third.to_ppm(format!("/tmp/sx14_d.dxt_texture_100_100.ppm"));

    println!("Hello, world!");
}
