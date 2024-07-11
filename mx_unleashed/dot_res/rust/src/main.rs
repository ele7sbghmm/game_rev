#![allow(unused)]
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use nom::{
    bytes::complete::take,
    multi::{count, length_data, many0},
    number::complete::le_u32,
    sequence::tuple,
    IResult,
};
use zune_inflate::DeflateDecoder;

#[derive(Debug)]
struct Dir {
    file_path: String,
    file_start: u32,
    file_size: u32,
}
impl Dir {
    fn parse<'a>(input: &'a [u8]) -> IResult<&'a [u8], Dir, ()> {
        let (input, file_path_bytes) = length_data(le_u32)(input)?;
        let (input, file_start) = le_u32(input)?;
        let (input, file_size) = le_u32(input)?;
        let file_path = String::from_utf8_lossy(file_path_bytes).to_string();

        Ok((
            input,
            Dir {
                file_path,
                file_start,
                file_size,
            },
        ))
    }
}
fn parse_header<'a>(input: &'a [u8]) -> IResult<&'a [u8], (u32, u32)> {
    let (input, (_1, weird_bytes_size)) = tuple((le_u32, le_u32))(input)?;
    let (input, _weird_bytes) = take(weird_bytes_size)(input)?;
    let (input, (maybe_num_of_dirs, dir_data_size)) = tuple((le_u32, le_u32))(input)?;
    // dbg!(_1, maybe_num_of_dirs, weird_bytes_size);

    Ok((input, (maybe_num_of_dirs, dir_data_size)))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let in_path: PathBuf = PathBuf::from(&args[1]);
    dbg!(&in_path);

    let mut v: Vec<u8> = vec![];
    _ = File::open(in_path)?.read_to_end(&mut v);
    let input: &[u8] = v.as_slice();

    let (input, (maybe_num_of_dirs, dir_data_size)) = parse_header(input).unwrap();
    // let (input, dirs) =
    //     count::<&[u8], Dir, (), for<'a> fn(&'a [u8]) -> IResult<&'a [u8], Dir, ()>>(
    //         Dir::parse, maybe_num_of_dirs as usize
    //     )(input).unwrap();
    // println!("{:?}", maybe_num_of_dirs);
    // println!("{:?}", dir_data_size);
    let (input, dir_data) = take::<usize, &[u8], ()>(dir_data_size as usize)(input).unwrap();
    let (_, dirs) = many0::<&[u8], Dir, (), for<'a> fn(&'a [u8]) -> IResult<&'a [u8], Dir, ()>>(
        Dir::parse,
    )(dir_data)
    .unwrap();
    let mut decompressed: Vec<u8> = vec![];

    for (idx, _byte) in input.iter().enumerate() {
        if input[idx] == 0x78 {
            // if input[idx+1] != 0x9c { () };
            match &mut DeflateDecoder::new(&input[idx..]).decode_zlib() {
                Ok(z) => decompressed.append(z),
                Err(e) => (),
            }
        };
    }

    // println!("{:?}", dirs[0]);
    // println!("{:?}", dirs[1]);
    // println!("{:?}", dirs[dirs.len()-1]);

    for Dir {
        file_path,
        file_start,
        file_size,
    } in dirs
    {
        let mut out_path: PathBuf = PathBuf::from(&args[2]);

        let mut file_path_ = file_path.replace("\\", "/");
        out_path = out_path.join(file_path_);
        println!(" {:?}", out_path);
        _ = create_dir_all(out_path.with_file_name(""));

        let mut v: Vec<u8> = vec![];

        let mut file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&out_path)
            .unwrap();
        file.write(
            &decompressed.as_slice()[file_start as usize..file_start as usize + file_size as usize],
        );
        println!("success");
    }

    Ok(())
}
