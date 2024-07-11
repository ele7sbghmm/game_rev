// #![allow(unused)]

use nom::{
    bytes::complete::{take, take_till},
    number::complete::le_u32,
    IResult, // sequence::tuple,
};
use std::{
    default::Default,
    fs::{create_dir_all, OpenOptions},
    io::{self, Read, Write},
    path::PathBuf,
};

#[derive(Debug)]
struct Header {
    fourcc: u32,
    alignment: u32,
    file_count: u32,
    crcs_offset: u32,
    crcs_size: u32,
    names_offset: u32,
    names_size: u32,
}

fn parse_header<'a>(input: &'a [u8]) -> IResult<&'a [u8], Header, ()> {
    let (input, fourcc) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, alignment) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, file_count) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, crcs_offset) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _offset) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _1) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, names_offset) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, names_size) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, crcs_size) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _24) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0) = le_u32::<&'a [u8], ()>(input).unwrap();

    Ok((
        input,
        Header {
            fourcc,
            alignment,
            file_count,
            crcs_offset,
            crcs_size,
            names_offset,
            names_size,
        },
    ))
}

#[derive(Debug, Default)]
struct Crc {
    offset: u32,
    crc: u32,
    size: u32,
    name_offset: u32,
    is_file: u32,
}
fn parse_crc<'a>(input: &'a [u8]) -> IResult<&'a [u8], Crc, ()> {
    let (input, offset) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, crc) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, size) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, name_offset) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, is_file) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0) = le_u32::<&'a [u8], ()>(input).unwrap();

    Ok((
        input,
        Crc {
            offset,
            crc,
            size,
            name_offset,
            is_file,
        },
    ))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];

    let mut buf: Vec<u8> = vec![];
    let _ = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap()
        .read_to_end(&mut buf);
    let input = buf.as_slice();

    let (_, header) = parse_header(input).unwrap();
    // dbg!(&header);

    let (_, names_block) = take::<u32, &[u8], ()>(header.names_size)(
        &input[(header.names_offset * header.alignment) as usize..],
    )
    .unwrap();
    let (_, crcs_block) = take::<u32, &[u8], ()>(header.crcs_size)(
        &input[(header.crcs_offset * header.alignment) as usize..],
    )
    .unwrap();

    let mut out = PathBuf::from("./_files/out/");
    let in_filename = PathBuf::from(&out)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let subbed = in_filename.replace(".", "_");

    out.push(subbed);
    let _ = create_dir_all(&out);

    let mut tmp = crcs_block;
    let mut crc: Crc;

    for _ in 0..header.file_count {
        (tmp, crc) = parse_crc(tmp).unwrap();
        let (garbage, _) = take::<u32, &[u8], ()>(crc.name_offset)(names_block).unwrap();
        let (_, name_bytes) = take_till::<fn(u8) -> bool, &[u8], ()>(|b| b == 0)(garbage).unwrap();

        if name_bytes == b"" {
            println!(" skipping {:?}", &name_bytes);
            continue;
        }
        // dbg!(&crc);

        fn win_to_posix_path(s: String) -> String {
            s.replace("\\", "/")
        }

        let name = String::from_utf8_lossy(name_bytes).to_string();

        let mut tmp_path = out.clone();
        tmp_path.push(win_to_posix_path(name));

        let (path_to, file_name) = (tmp_path.with_file_name(""), tmp_path.file_name());
        if let Some(_) = file_name {
            let _ = create_dir_all(path_to.to_string_lossy().to_string());

            let mut tmp_file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&tmp_path)
                .unwrap();

            let start = (header.alignment * crc.offset) as usize;
            let end = start + crc.size as usize;

            let _ = tmp_file.write(&input[start..end]);
        } else {
            println!("empty filename. skipping");
            dbg!(&crc);
        }

        println!("done {:?}", &tmp_path);
    }

    Ok(())
}
