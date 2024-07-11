#![allow(unused)]
use std::fs::{File, read_dir, create_dir_all, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use nom::{
    IResult, error::ParseError,
    bytes::complete::{tag, take},
    multi::{count, length_data},
    number::complete::{float, le_u32},
    sequence::tuple,
};

fn vec3u<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], (u32, u32, u32), E> {
    tuple((le_u32::<&'a [u8], E>, le_u32, le_u32))(input)
}
fn header<'a, E: ParseError<&'a [u8]> + std::fmt::Debug>(input: &'a [u8]) -> IResult<&'a [u8], u32, E> {
    let (input, _radcore) = take::<usize, &'a [u8], E>(32)(input).unwrap();
    let (input, _something) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, starts_offset) = le_u32::<&'a [u8], E>(input).unwrap();
    // rest of header can be skipped
    // using offsets to target rest of data

    Ok((b"", starts_offset))
}
fn parse_starts<'a, E: ParseError<&'a [u8]> + std::fmt::Debug>(input: &'a [u8], offset: u32) -> (u32, Vec<(u32, u32, u32)>) {
    let (input, _) = take::<u32, &'a [u8], E>(offset)(input).unwrap();
    let (input, num) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, dir_offset) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, _) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, _) = le_u32::<&'a [u8], E>(input).unwrap();
    let (_, starts) =
        count::<&[u8], (u32, u32, u32), E, fn(&'a [u8]) -> IResult<&'a [u8], (u32, u32, u32), E>>(
            vec3u, num as usize,
        )(input).unwrap();
    (dir_offset, starts)
}
fn dir<'a, E: ParseError<&'a [u8]> + std::fmt::Debug>(input: &'a [u8]) -> IResult<&'a [u8], String, E> {
    let (input, path_slice) =
        length_data::<&'a [u8], u32, E, fn(&'a [u8]) -> IResult<&'a [u8], u32, E>>(le_u32)(input).unwrap();
    let (input, idk) = le_u32::<&'a [u8], E>(input).unwrap();

    let s: String = String::from_utf8_lossy(path_slice.strip_suffix(b"\x00").unwrap()).to_string();
    let posix_path = s.replace("\\", "/");

    Ok((input, posix_path))
}
fn parse_dirs<'a, E: ParseError<&'a [u8]> + std::fmt::Debug>(input: &'a [u8], offset: u32) -> Vec<String> {
    let (input, _skip) = take::<u32, &'a [u8], E>(offset)(input).unwrap();
    let (input, num) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, _null) = le_u32::<&'a [u8], E>(input).unwrap();
    count::<&'a [u8], String, E, fn(&'a [u8]) -> IResult<&'a [u8], String, E>>(
        dir, num as usize,
    )(input).unwrap().1
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let out_path = PathBuf::from(&args[1]);

    let mut v: Vec<u8> = vec![]; 
    _ = OpenOptions::new().read(true).open(&args[2]).unwrap().read_to_end(&mut v);
    let input = v.as_slice();

    let (_, starts_offsets) = header::<()>(input).unwrap();
    let (dir_offset, starts) = parse_starts::<()>(input, starts_offsets);
    let dirs = parse_dirs::<()>(input, dir_offset);

    for (i, (_, start, size)) in starts.iter().enumerate() {
        let out_dir = out_path.join(&dirs[i]);
        _ = create_dir_all(out_dir.with_file_name(""));

        // let mut file: File = OpenOptions::new().write(true).create(true).open(&out_dir).unwrap();
        // file.write(&input[*start as usize..*start as usize + *size as usize]);
        println!(" GOOD {:?}", out_dir);
    }

    // println!("starts {:?} dirs {:?}", starts.len(), starts);
    assert_eq!(starts.len(), dirs.len());

    Ok(())
}