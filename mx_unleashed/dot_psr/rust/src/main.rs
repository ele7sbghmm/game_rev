#![allow(unused)]
use std::io::{self, Read, Write};
use std::fs;
use std::path::{Path, PathBuf};

use nom::{
    self, IResult, Parser, error::{ParseError},
    bytes::complete::{take},
    multi::{count, length_data, many1},
    number::complete::{le_f32, le_u32, le_i32},
    sequence::{tuple},
};

#[derive(Debug)]
struct Dir {
    file_path: String,
    size: u32,
    unknown: u32,
}
impl Dir {
    fn from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], Dir, ()> {
        let (input, file_path_bytes) = length_data(le_u32)(input)?;
        let (input, (size, unknown)) = tuple((le_u32, le_u32))(input)?;
        let file_path_windows = String::from_utf8_lossy(file_path_bytes).to_string();
        let file_path = file_path_windows.replace("\\", "/");
        Ok((input, Dir {file_path, size, unknown}))
    }
}

const FILE_STRIDE: usize = 0x800;
fn calc_padding(file_size: usize) -> usize {
    let padding_size: usize = FILE_STRIDE - file_size % FILE_STRIDE;
    if padding_size == FILE_STRIDE { 0 }
    else { padding_size }
}
fn take_x_and_padding<'a>(input: &'a [u8], size: usize) -> IResult<&'a [u8], &'a [u8]> {
    let padding_size = calc_padding(size);
    let (input, output) = take(size)(input)?;
    let (input, _padding) = take(padding_size)(input)?;
    Ok((input, output))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let psr_path = Path::new(&args[1]);
    // let psr_path: &Path = Path::new("/tmp/mx_ps2/MX.PSR_mod");
    let out_path = Path::new(&args[2]).join("mx_psr");

    let mut buf: Vec<u8> = vec![];
    _ = fs::File::open(psr_path)?.read_to_end(&mut buf);
    let mut input: &[u8] = buf.as_slice();

    let header_section_size: usize = 4 * 6;
    let (mut input, (_, _, _, _, dir_section_size, _)) =
        tuple((le_u32::<&[u8], ()>, le_u32, le_u32, le_u32, le_u32, le_u32))(input).unwrap();

    let (mut input, dir_data) =
        take::<usize, &[u8], ()>(dir_section_size as usize)(input).unwrap();
    let padding_size =
        calc_padding(header_section_size + dir_section_size as usize);
    let (mut input, _padding) =
        take::<usize, &[u8], ()>(padding_size as usize)(input).unwrap();
    let (dir_data, dirs) =
        many1::<&[u8], Dir, (), for<'a> fn(&'a [u8]) -> IResult<&'a [u8], Dir, ()>>(
            Dir::from_bytes
        )(dir_data).unwrap();

    // println!("{:?}", dirs);
    // println!("{:?}", dirs.len());
    println!("{:?}", dirs[0]);
    println!("{:?}", dirs[2]);
    println!("{:?}", dirs[dirs.len()-1]);

    // let out_path: PathBuf = PathBuf::from("/tmp/mx_ps2/extracted/");

    for Dir {file_path, size, unknown} in dirs {
        let mut tmp_out_path: PathBuf = out_path.clone();
        tmp_out_path.push(PathBuf::from(file_path));
    
        _ = fs::create_dir_all(tmp_out_path.with_file_name(""));
        let mut file: fs::File = fs::File::create(tmp_out_path).unwrap();
    
        let (input_2, file_data) =
            take_x_and_padding(input, size as usize).unwrap();
    
        file.write(file_data);
    
        input = input_2;
    }

    Ok(())
}
