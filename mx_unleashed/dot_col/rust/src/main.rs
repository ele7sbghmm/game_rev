#![allow(unused)]
use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions, create_dir_all};
use std::path::PathBuf;
use nom::{
    IResult,
    bytes::complete::take,
    multi::count,
    number::complete::{le_u32, le_i16, le_u16, le_f32},
    sequence::{tuple},
};

#[derive(Debug)]
struct Vec3f { x: f32, y: f32, z: f32 }
impl Vec3f {
    fn parse<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec3f, ()> {
        let (input, (x, y, z)) =
            tuple((le_f32, le_f32, le_f32))(input)?;
        Ok((input, Vec3f {x, y, z}))
    }
}

type T1 = Vec3f;
type T2 = (f32, f32, f32, f32, f32, f32, f32, u16, u16);
fn t2_from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], T2, ()> {
    tuple((le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_u16, le_u16))(input)
}
type T3 = (f32, Vec3f, u16, u16, u16);
fn t3_from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], T3, ()> {
    tuple((le_f32, Vec3f::parse, le_u16, le_u16, le_u16))(input)
}
fn header<'a>(input: &'a [u8]) -> IResult<&'a [u8], (u32, u32, u32, u32)> {
    let (input, v) =
        count(le_u32::<&'a [u8], ()>, 22_usize)(input).unwrap();
    Ok((input, (v[18], v[19], v[20], v[21])))
}
fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let in_path: PathBuf = PathBuf::from(&args[1]);

    let mut v: Vec<u8> = vec![];
    _ = OpenOptions::new()
        .read(true)
        .open(in_path)?
        .read_to_end(&mut v);
    let input = v.as_slice();

    let (input, (t1_count, t2_count, t3_count, t4_count)) =
        header(input).unwrap();

    // let (input, _null) = le_u32::<&[u8], ()>(input).unwrap();

    let (input, t1) =
        count::<&[u8], T1, (), for<'a> fn(&'a [u8]) -> IResult<&'a [u8], T1, ()>>(
            T1::parse,
            t1_count as usize
        )(input).unwrap();

    let (input, _null) = le_u32::<&[u8], ()>(input).unwrap();

    let (input, t2) =
        count::<&[u8], T2, (), for<'a> fn(&'a [u8]) -> IResult<&'a [u8], T2, ()>>(
            t2_from_bytes,
            t2_count as usize,
        )(input).unwrap();

    let (input, t3) =
        count::<&[u8], T3, (), for<'a> fn(&'a [u8]) -> IResult<&'a [u8], T3, ()>>(
            t3_from_bytes,
            t3_count as usize,
        )(input).unwrap();

    // dbg!(t1);
    // dbg!(t2);
    // dbg!(t3);

    for i in t1 { println!("{:?}", i); }
    for i in t2 { println!("{:?}", i); }
    for i in t3 { println!("{:?}", i); }

    Ok(())
}
