#![allow(unused)]
use std::fs::{File, OpenOptions, rename, read_dir, create_dir_all};
use std::io::{self, prelude::*, Read, Write};

struct Rebuh {
    signature: [u8; 4], // "REBU"
    len: u32,           // 0x48  72
    name: [u8; 36],     // "agent"
    values: [u32; 8],   // 00 16 48 A8  DC 00 00 00
}
struct Mina {
    signature: [u8; 4], // "MINA"
    values: [u32; 5],   // 0x60  96
    vecs: [f32; 4],     // 1. 
}
struct Gnht {signature: [u8; 4], len: u32, values: [u32; 16]}
struct Naxt {
    signature: [u8; 4], // "naxt"
    size: u32,          // 0x1001  266
    name: [u8; 0x28],   // "_dummy"
    name_2: [u8; 0x20], // ""
    name_3: [u8; 0x24], // ""
    neg1s: [i32; 2],
}
struct X4C {
    small: u32,
    float_10001: [f32; 5],
    x21: u32,
    name: [u8; 0x30],
}
struct X34 {
    null: u32,
    x34: u32,
    two: u32,
    one: u32,
    null_2: u32,
    floats: [f32; 6],
    x300: u32,
    x100: u32,
}
struct X98 {
    null: u32,
    x98: u32,
    null_2: u32,
    x6003: u32,

    null_3: u32,
    name_1: [u8; 0x26],
    null_4: u32,
    name_2: [u8; 0x26],
}
struct Nuos {
    name: [u8; 32], 
    x2c_count: u32, // 0x149
    x2cs: Vec<X2c>,
}
struct X2c {
    hmm: u32,
    small: u32,
    float: f32, // 120f
    nulls: [u32; 6],
    sometimes_4: u32,
}
struct Btvl {
    sig: [u8; 4], // "BTVL"
    size: u32,
    name: [u8; 0x20], // "level_0_water"
    nullnullnegone: [u32; 3],
    name_2: [u8; 0x20], // "Test Level"
    nullx3negone: [u32; 4],
}
struct Vec3f {x: f32, y: f32, z: f32}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    dbg!(&args[1]);

    Ok(())
}
