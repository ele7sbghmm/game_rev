#![allow(unused)]
use std::cmp::Ord;
use std::fs::{File, read_dir, create_dir_all, OpenOptions};
use std::io::{self, Read, Write};
use std::ops::Index;
use std::path::PathBuf;
use nom::{
    IResult, error::ParseError,
    bytes::complete::{tag, take},
    multi::{count, length_data},
    number::complete::{float, le_u32, be_u32, le_u8, be_u64, le_u64},
    sequence::tuple,
};

fn lets_sort(v: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut todo: Vec<u64> =
        v
            .iter()
            .map(|(h, l)| {
                ((*h as u64) << 32) + (*l as u64)
            }).collect();
    todo.sort();
    todo.iter().map(|&b| {((b >> 32) as u32, b as u32)}).collect()
}
fn vec3u32<'a>(input: &'a [u8]) -> IResult<&'a [u8], (u32, u32), ()> {
    let (input, (_, u, v)) =
        tuple((le_u32::<&'a [u8], ()>, le_u32, le_u32))(input).unwrap();
    Ok((input, (u, v)))
}
fn header<'a, E: ParseError<&'a [u8]> + std::fmt::Debug>(input: &'a [u8]) -> IResult<&'a [u8], (u32, u32), E> {
    let (input, _radcore) = take::<usize, &'a [u8], E>(32)(input).unwrap();
    let (input, _something) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, _starts_offset) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, _starts_size) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, dir_offset) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, _fake_file) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, _null) = le_u32::<&'a [u8], E>(input).unwrap();
    let (input, starts_num) = le_u32::<&'a [u8], E>(input).unwrap();

    Ok((input, (starts_num, dir_offset)))
}

fn dir<'a>(input: &'a [u8]) -> IResult<&'a [u8], String, ()> {
    let (input, _idk) = take::<u32, &'a [u8], ()>(0xf)(input).unwrap();
    let (_, path_len) = le_u32::<&'a [u8], ()>(input).unwrap();
    
    let (input, path_slice) =
        length_data::<&'a [u8], u32, (), fn(&'a [u8]) -> IResult<&'a [u8], u32, ()>>(le_u32)(input).unwrap();

    let s: String = String::from_utf8_lossy(path_slice.strip_suffix(b"\x00").unwrap()).to_string();
    let posix_path = s.replace("\\", "/");

    Ok((input, posix_path))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let out_path = PathBuf::from(&args[1]);

    let mut v: Vec<u8> = vec![]; 
    _ = OpenOptions::new().read(true).open(&args[2]).unwrap().read_to_end(&mut v);
    let untouched_input = v.as_slice();

//// STARTS
    let (input, (starts_num, dirs_offset)) = header::<()>(untouched_input).unwrap();
    dbg!(starts_num, dirs_offset);
    let (_, starts) =
        count::<&[u8], (u32, u32), (), fn(&[u8]) -> IResult<&[u8], (u32, u32), ()>>(
            vec3u32, starts_num as usize,
        )(input).unwrap();
    // let starts_sorted = lets_sort(starts);
    let mut starts_sorted = lets_sort(starts);
    
//// DIRS
    let (input, _skip) = take::<u32, &[u8], ()>(dirs_offset)(untouched_input).unwrap();
    let (input, _) = le_u32::<&[u8], ()>(input).unwrap();
    let (input, _creepy_nullbyte) = le_u8::<&[u8], ()>(input).unwrap();
    let (_, dirs) = count::<&[u8], String, (), fn(&[u8]) -> IResult<&[u8], String, ()>>(
            dir, starts_num as usize,
        )(input).unwrap();
////

    for (i, (start, size)) in starts_sorted.iter().enumerate() {
        let out_dir = out_path.join(&dirs[i]);

        _ = create_dir_all(out_dir.with_file_name(""));
        let mut file: File = OpenOptions::new().write(true).create(true).open(&out_dir).unwrap();
        file.write(&untouched_input[*start as usize..*start as usize + *size as usize]);

        println!(" GOOD {:?}", out_dir);
    }

    Ok(())
}
