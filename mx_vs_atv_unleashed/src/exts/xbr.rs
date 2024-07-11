use crate::exts::utils::{path_windows_to_posix, write_file};
use std::{fs, path::PathBuf};

use nom::{bytes::complete::take, multi::count, number::complete::le_u32, Parser};

const CHUNK_SIZE: usize = 0x800;

struct XbrHeader {
    _02: u32,
    num_of_dirs: usize,
    _75d: u32,
    _7c2: u32,
    dir_section_size: usize,
    _7ecb7e: u32,
}
impl XbrHeader {
    fn from_bytes<'a>() -> impl Parser<&'a [u8], Self, ()> {
        move |input: &'a [u8]| {
            let (input, _02) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (input, num_of_dirs) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (input, _75d) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (input, _7c2) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (input, dir_section_size) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (input, _7ecb7e) = le_u32::<&'a [u8], ()>(input).unwrap();

            let num_of_dirs = num_of_dirs as usize;
            let dir_section_size = dir_section_size as usize;

            Ok((
                input,
                Self {
                    _02,
                    num_of_dirs,
                    _75d,
                    _7c2,
                    dir_section_size,
                    _7ecb7e,
                },
            ))
        }
    }
}
pub struct Dir {
    pub path: PathBuf,
    pub size: usize,
    pub offset: usize,
}
impl Dir {
    fn from_bytes<'a>() -> impl Parser<&'a [u8], Self, ()> {
        move |input: &'a [u8]| {
            let (input, path_len) = le_u32::<&'a [u8], ()>(input).unwrap();

            let (input, path_bytes) = take::<u32, &'a [u8], ()>(path_len)(input).unwrap();
            let path = PathBuf::from(&String::from_utf8_lossy(path_bytes).to_string());
            let path = path_windows_to_posix(path);

            let (input, size) = le_u32::<&'a [u8], ()>(input).unwrap();
            let size = size as usize;

            let (input, offset) = le_u32::<&'a [u8], ()>(input).unwrap();
            let offset = offset as usize;

            Ok((input, Self { path, size, offset }))
        }
    }
}
pub struct Xbr {
    pub header: XbrHeader,
    pub dirs: Vec<Dir>,
}
impl Xbr {
    pub fn parse(clean_input: &[u8]) -> Self {
        let (input, header) = XbrHeader::from_bytes().parse(clean_input).unwrap();
        let (_input, dirs) = count(Dir::from_bytes(), header.num_of_dirs)(input).unwrap();

        Xbr { header, dirs }
    }
    pub fn do_stuff_with_dirs(&self, path: PathBuf, clean_input: &[u8]) {
        for dir in &self.dirs {
            let data_offset = dir.offset * CHUNK_SIZE;
            let data_end_offset = data_offset + dir.size;
            let data = &clean_input[data_offset..data_end_offset];

            let posix_path = path_windows_to_posix(dir.path.clone());
            let full_path = path.join(posix_path);

            let _ = fs::create_dir_all(&full_path.with_file_name("")).unwrap();
            write_file(vec![data.to_vec()], full_path);
        }
    }
}
