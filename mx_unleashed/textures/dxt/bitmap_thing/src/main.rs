use std::fs;
use std::io::{Read, Write};
use std::fmt::Debug;

use nom::{
    IResult,
    error::ParseError,
    bytes::complete::take,
    multi::count,
    number::complete::{le_u8, le_u32},
    sequence::tuple,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut buf: Vec<u8> = vec![];
    _ = fs::OpenOptions::new()
        .read(true)
        .open(&args[1])
        .unwrap()
        .read_to_end(&mut buf);
    let input = buf.as_slice();

    let (_, dxt) = DXT::from_bytes::<()>(input).unwrap();
    let ppm_header = dxt.ppm_header();
    
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("/tmp/mx/d.ppm")
        .unwrap();

    dbg!(&dxt.header);
    dbg!(dxt.color_table.len());
    dbg!(&dxt.header2);
    dbg!(dxt.pixel_table.len());

    file.write(ppm_header.as_bytes());

    let mut slice: Vec<u8> = vec![];
    for index in dxt.pixel_table {
        let Color {red, green, blue, alpha} =
            dxt.color_table[index as usize];
        slice.push(blue);
        slice.push(green);
        slice.push(red);
        slice.push(alpha);
    }
    file.write(slice.as_slice());

}

#[derive(Debug)]
struct DXT {
    header: Header,
    color_table: ColorTable,
    header2: Header2,
    pixel_table: PixelTable,
}
impl DXT {
    fn from_bytes<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], DXT, E> {
        let (input, header) = Header::from_bytes(input)?;
        let (input, color_table) =
            count(
                Color::from_bytes,
                header.color_table_size as usize
            )(input)?;
        let (input, header2) = Header2::from_bytes(input)?;
        let (input, pixel_table) =
            count(
                le_u8,
                header2.pixel_table_size as usize
            )(input)?;
        Ok((input, DXT {header, color_table, header2, pixel_table}))
    }
    fn ppm_header(&self) -> String {
        let mut s = String::new();
        s += &format!(
"P7
WIDTH {}
HEIGHT {}
DEPTH 4
MAXVAL 255
TUPLETYPE RGB_ALPHA
ENDHDR
",
            self.header2.width, self.header2.height
        );
        s
    }
}

type PixelTable = Vec<ColorTableIndex>;
type ColorTableIndex = u8;
type ColorTable = Vec<Color>;
// type ColorTable = Vec<u32>;
#[derive(Debug)]
struct Color {red: u8, green: u8, blue: u8, alpha: u8}
impl Color {
    fn from_bytes<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Color, E> {
        let (input, (red, green, blue, alpha)) =
            tuple((le_u8::<&'a [u8], E>, le_u8, le_u8, le_u8))(input)?;
        Ok((input, Color {red, green, blue, alpha}))
    }
}
#[derive(Debug)]
struct Header {_unk: u32, _unk2: u32, color_table_size: u32}
impl Header {
    fn from_bytes<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Header, E> {
        let (input, (_unk, _unk2, color_table_size)) =
            tuple((le_u32::<&'a [u8], E>, le_u32, le_u32))(input)?;
        Ok((input, Header {_unk, _unk2, color_table_size}))
    }
}

#[derive(Debug)]
struct Header2 {
    thing_type: u32,
    width: u32,
    height: u32,
    pixel_table_size: u32,
}
impl Header2 {
    fn from_bytes<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], Header2, E> {
        // let mut input = input;
        let (mut input, (thing_type, width, height)) =
            tuple((le_u32, le_u32, le_u32))(input)?;
        match thing_type {
            9 => { (input, _) = take(0x8c_usize)(input)?; },
            1 => { (input, _) = take(8_usize)(input)?; },
            _ => panic!("unknown thing_type"),
        }
        let (input, pixel_table_size) = le_u32(input)?;

        Ok((input, Header2 {thing_type, width, height, pixel_table_size}))
    }
}
