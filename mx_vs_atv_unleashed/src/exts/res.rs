use nom::{
    bytes::complete::take,
    multi::{count, many0},
    number::complete::{le_u16, le_u32},
    Parser,
};
use zune_inflate::DeflateDecoder;

use super::utils::{path_windows_to_posix, write_file};
use std::{fs, path::PathBuf};

#[derive(Debug)]
struct ResHeader {
    _03: u32,
    offset_to_dirs: usize,
    _0340: u32,
    _0: u32,
}
impl ResHeader {
    fn from_bytes<'i>() -> impl Parser<&'i [u8], Self, ()> {
        move |input: &'i [u8]| {
            let (input, _03) = le_u32::<&'i [u8], ()>(input).unwrap();
            let (input, offset_to_dirs) = le_u32::<&'i [u8], ()>(input).unwrap();
            let (input, _0340) = le_u32::<&'i [u8], ()>(input).unwrap();
            let (input, _0) = le_u32::<&'i [u8], ()>(input).unwrap();

            let offset_to_dirs = offset_to_dirs as usize;

            Ok((
                input,
                Self {
                    _03,
                    offset_to_dirs,
                    _0340,
                    _0,
                },
            ))
        }
    }
}
#[derive(Debug)]
struct Res<'z> {
    header: ResHeader,
    // _1f6b: Vec<u8>,
    zlib_offsets: Vec<usize>,
    dirs: Vec<Dir>,
    compressed_data: &'z [u8],
}
impl<'z> Res<'z> {
    fn from_bytes<'i: 'z>() -> impl Parser<&'i [u8], Self, ()> {
        move |input: &'i [u8]| {
            let (input, header) = ResHeader::from_bytes().parse(input).unwrap();
            let (input, zlib_offsets_section_size) = le_u32::<&'i [u8], ()>(input).unwrap();
            let (input, zlib_offset_data) =
                take::<u32, &'i [u8], ()>(zlib_offsets_section_size)(input).unwrap();
            let (_, zlib_offsets) = many0(le_u16::<&'i [u8], ()>)(zlib_offset_data).unwrap();

            let (input, num_of_dirs) = le_u32::<&'i [u8], ()>(input).unwrap();
            let (input, _dir_section_size) = le_u32::<&'i [u8], ()>(input).unwrap();
            let (compressed_data, dirs) =
                count(Dir::from_bytes(), num_of_dirs as usize)(input).unwrap();

            let zlib_offsets = zlib_offsets
                .iter()
                .map(|offset| offset.clone() as usize)
                .collect();

            Ok((
                input,
                Self {
                    header,
                    zlib_offsets,
                    dirs,
                    compressed_data,
                },
            ))
        }
    }
    fn decompress<'d>(&mut self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        let mut previous_offset = 0usize;

        for offset in self.zlib_offsets.iter() {
            if offset.clone() == 0usize {
                break;
            }
            let (start, end) = (previous_offset, previous_offset + offset + 1);
            // dbg!(start, end);
            let slc = &self.compressed_data[start..end];
            // dbg!(&slc[..10]);
            let mut decoder = DeflateDecoder::new(slc);
            let _ = match decoder.decode_zlib() {
                Ok(mut decompressed_data) => buf.append(&mut decompressed_data),
                Err(_e) => panic!(
                    "decompression failed\n{:02x} {:02x}\n{:?}",
                    start, end, self.zlib_offsets,
                ),
            };
            previous_offset = end;
        }

        self.compressed_data = &[0];
        buf
    }
}
#[derive(Debug)]
struct Dir {
    path: PathBuf,
    decompressed_offset: usize,
    decompressed_size: usize,
}
impl Dir {
    fn from_bytes<'i>() -> impl Parser<&'i [u8], Self, ()> {
        move |input: &'i [u8]| {
            let (input, path_len) = le_u32::<&'i [u8], ()>(input).unwrap();
            let (input, path) = take::<u32, &'i [u8], ()>(path_len)(input).unwrap();
            let (input, decompressed_offset) = le_u32::<&'i [u8], ()>(input).unwrap();
            let (input, decompressed_size) = le_u32::<&'i [u8], ()>(input).unwrap();

            let path = PathBuf::from(String::from_utf8_lossy(path).to_string());
            let decompressed_offset = decompressed_offset as usize;
            let decompressed_size = decompressed_size as usize;

            Ok((
                input,
                Self {
                    path,
                    decompressed_offset,
                    decompressed_size,
                },
            ))
        }
    }
}

pub fn parse_res(dir_to: PathBuf, input: &[u8]) {
    let (_input, mut res) = Res::from_bytes().parse(input).unwrap();
    let decompressed_data: Vec<u8> = res.decompress();
    for dir in res.dirs.iter() {
        let path = path_windows_to_posix(dir.path.clone());
        let path = dir_to.join(path);
        let (start, end) = (
            dir.decompressed_offset,
            dir.decompressed_offset + dir.decompressed_size,
        );
        let file_data = decompressed_data[start..end].to_vec();
        let _ = fs::create_dir_all(&path.with_file_name("")).unwrap();

        println!("{:02x} {:02x} {:?}", start, end, path);
        write_file(vec![file_data], path);
    }
}
