use crate::exts::utils::{path_windows_to_posix, write_file};
use nom::{
    bytes::complete::take,
    number::complete::{le_u16, le_u32},
};
use std::{fs, path::PathBuf};

// struct Dir {
//     path: String, // [u8; 0x26]
//     _7f: u16,
//     _08: u32,
//     _1a209101: u32,
//     _780114__: u32,
//     _20202120: u32,
//     _c6023020: u32,
//     _52018d__: u32,
//     _7d018f__: u32,
//     _90__1820: u32,
//     _19201c20: u32,
//     _1d202220: u32,
//     _780114__2: u32,
//     _dc022221: u32,
//     _c04314__: u32,
//     _52019d__: u32,
//     start: u32,
//     end: u32,
// }

struct Dir(PathBuf, usize, usize);
pub fn parse_pak(out_path: PathBuf, input: &[u8]) {
    let (mut input, num_of_dirs) = le_u32::<&[u8], ()>(input).unwrap();
    let num_of_dirs = num_of_dirs as usize;
    let mut dirs: Vec<Dir> = vec![];

    let mut path_bytes: &[u8] = &[0u8; 0x26];
    let mut _unknown: &[u8] = &[0u8; 0x3e];
    let mut start: u32 = 0;
    let mut size: u32 = 0;

    for _ in 0..num_of_dirs {
        (input, path_bytes) = take::<usize, &[u8], ()>(0x26)(input).unwrap();
        (input, _unknown) = take::<usize, &[u8], ()>(0x3e)(input).unwrap();
        (input, start) = le_u32::<&[u8], ()>(input).unwrap();
        (input, size) = le_u32::<&[u8], ()>(input).unwrap();

        let mut path = path_bytes.split(|i| *i == 0);

        let path = String::from_utf8_lossy(path.next().unwrap()).to_string();
        let path = PathBuf::from(path);
        let path = path_windows_to_posix(path);

        let start = start as usize;
        let size = size as usize;

        dirs.push(Dir(path, start, size))
    }
    for dir in dirs.iter() {
        let (start, end) = (dir.1, dir.1 + dir.2);

        let data = &input[start..end];

        let path = out_path.join(dir.0.clone());
        let _ = fs::create_dir_all(&path.with_file_name("")).unwrap();

        write_file(vec![data.to_vec()], path.clone());
    }
}
