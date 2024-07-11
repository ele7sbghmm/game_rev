#![allow(unused)]

use std::fs;
use std::io::{Read, Write};
use std::fmt::Debug;

use nom::{
    IResult,
    multi::count,
    number::complete::{le_f32, le_u8},
    sequence::tuple,
};

fn main() {
    let mut buf: Vec<u8> = vec![];
    _ = fs::OpenOptions::new()
        .read(true).open("/tmp/mx/trn/free10.trn").unwrap()
        .read_to_end(&mut buf);
    let input = buf.as_slice();

    let (input, v) =
        count(from_bytes, 2311)(input).unwrap();
    
    dbg!(&input);
    let mut file = fs::OpenOptions::new()
        .create(true).write(true).truncate(true)
        .open("/tmp/mx/trn/free10_xyzw.trn.obj").unwrap();

    let mut obj = String::new();
    for vert in v.iter() {
        obj += &vert.to_obj();
    }
    file.write(obj.as_bytes());
}

#[derive(Debug)]
struct Vec3f {x: f32, y: f32, z: f32}
impl Vec3f {
    fn to_obj(&self) -> String {
        format!("v {} {} {}\n", self.x, self.y, self.z)
    }
}

fn from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec3f> {
    let (input, (x, y, z)) =
        tuple((le_f32, le_f32, le_f32))(input)?;
    let (input, float) = le_f32(input)?;
    let (input, ffs) =
        tuple((
            le_f32, le_f32, le_f32, le_f32,
            le_f32, le_f32, le_f32, le_f32,
            le_f32, le_f32, le_f32, le_f32,
            le_f32, le_f32, le_f32, le_f32,
            le_f32, le_f32, le_f32, le_f32, le_u8
        ))(input)?;
    Ok((input, Vec3f {x, y, z}))
}
