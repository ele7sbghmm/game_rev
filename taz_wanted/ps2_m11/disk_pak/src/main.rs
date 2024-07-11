// #![allow(unused)]

use std::{
    io::{self, Read, Write},
    default::Default,
    fs::{OpenOptions, create_dir_all},
    path::PathBuf,
};
use nom::{
    bytes::complete::{take, take_till},
    multi::{count, fold_many0},
    number::complete::le_u32,
    IResult
};

#[derive(Debug)]
struct Header {
    alignment: u32,
    file_count: u32,
    crcs_offset: u32,
    crcs_size: u32,
    names_offset: u32,
    names_size: u32,
}
fn parse_header<'a>(input: &'a [u8]) -> IResult<&'a [u8], Header, ()> {
    let (input, _fourcc)      = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, alignment)    = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0)           = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, file_count)   = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, crcs_offset)  = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0)           = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0)           = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0)           = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _offset)      = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _1)           = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, names_offset) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, names_size)   = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, crcs_size)    = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0)           = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _24)          = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0)           = le_u32::<&'a [u8], ()>(input).unwrap();

    Ok((input, Header {
        alignment,
        file_count,
        crcs_offset,
        crcs_size,
        names_offset,
        names_size
    }))
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
    let (input, offset)      = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, crc)         = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, size)        = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, name_offset) = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, is_file)     = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0)          = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0)          = le_u32::<&'a [u8], ()>(input).unwrap();
    let (input, _0)          = le_u32::<&'a [u8], ()>(input).unwrap();

    Ok((input, Crc {offset, crc, size, name_offset, is_file}))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];

    let mut buf: Vec<u8> = vec![];
    let _ = OpenOptions::new().read(true).open(path).unwrap().read_to_end(&mut buf);
    let input = buf.as_slice();

    let (_, header) = parse_header(input).unwrap();
    // dbg!(&header);

    let (_, names_block) =
        take::<u32, &[u8], ()>
            (header.names_size)
            (&input[(header.names_offset * header.alignment) as usize..])
            .unwrap();
    let (_, crcs_block) =
        take::<u32, &[u8], ()>
            (header.crcs_size)
            (&input[(header.crcs_offset * header.alignment) as usize..])
            .unwrap();

    let mut tmp = crcs_block;
    let mut crc: Crc;
    let _ = create_dir_all("./out/");
    let out = PathBuf::from("./out/");
    for _ in 0..header.file_count {
        (tmp, crc) = parse_crc(tmp).unwrap();
        let (garbage, _) = take::<u32, &[u8], ()>(crc.name_offset)(names_block).unwrap();
        let (_, name) = take_till::<fn(u8) -> bool, &[u8], ()>(|b| b == 0)(garbage).unwrap();
        if name == b"" { continue; }

        let tmp_out = out.clone();
        let t = tmp_out.join(String::from_utf8_lossy(name).to_string());

        let mut tmp_file = OpenOptions::new().create(true).write(true).open(&t).unwrap();
        let (s, o) = (crc.size as usize, (0x800 * crc.offset) as usize);
        println!("{:x} {:x} ", s, o);
        let _ = tmp_file.write(&input[o..o+s]);
        println!("done {:?}", &t);
    }

    Ok(())
}
