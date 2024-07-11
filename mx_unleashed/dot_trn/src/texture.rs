use crate::{
    color::{Rgba, RgbaArray},
    ppm::{BufferType, Ppm, TuplType},
    utils::Header,
};
use nom::{
    multi::count,
    number::complete::{le_u32, le_u8},
    IResult, Parser,
};
use std::{fs::OpenOptions, io::Write, path::Path};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Texture {
    // pub _27: u32,
    // pub _01: u32,
    pub width: u32,
    pub height: u32,
    pub size: u32,
    // pub _big_number: u32,
    pub pixels: RgbaArray,
}
impl Texture {
    // pub fn from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], Texture, ()> {
    pub fn parse(offset: usize, header: &Header, clean_input: &[u8]) -> Texture {
        let (input, _27) = le_u32::<&[u8], ()>(&clean_input[offset..]).unwrap();
        let (input, _01) = le_u32::<&[u8], ()>(input).unwrap();

        let (input, width) = le_u32::<&[u8], ()>(input).unwrap();
        let (input, height) = le_u32::<&[u8], ()>(input).unwrap();
        let (input, size) = le_u32::<&[u8], ()>(input).unwrap();
        let (input, _big_number) = le_u32::<&[u8], ()>(input).unwrap();
        // let (input, pixels) = RgbaArray::from_bytes_u8(size as usize)
        //     .parse(input)
        //     .unwrap();
        let (input, indices) = count(le_u8::<&[u8], ()>, size as usize)(input).unwrap();
        let inner = indices
            .iter()
            .map(|index| header.colortable.inner[*index as usize].clone())
            .collect();

        Texture {
            // _27,
            // _01,
            width,
            height,
            size,
            // _big_number,
            pixels: RgbaArray { inner },
        }
    }
    pub fn to_ppm(&self, path: &Path) -> () {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        let mut ppm = Ppm::new(
            TuplType::RgbAlpha,
            self.width as usize,
            self.height as usize,
            255,
            BufferType::Buffer(self.pixels.to_bytes()),
        );

        f.write(ppm.to_bytes(None).as_slice());
    }
}

pub fn texture_array_to_giant(width: usize, height: usize, array: Vec<Texture>) -> Texture {
    let mut pixels: Vec<Rgba> = vec![];
    let rows = 8;
    let columns = 4;

    for row in 0..rows {
        for pixel_row in 8..height - 8 {
            for column in 0..columns {
                let id = (row * columns + column);
                let range = (pixel_row * width + 8, (pixel_row + 1) * width - 8);
                pixels.append(&mut array[id].pixels.inner[range.0..range.1].to_vec());
            }
        }
    }
    let mut texture: Texture = Texture::default();
    texture.width = (width * columns - (16 * columns)) as u32;
    texture.height = (height * rows - (16 * rows)) as u32;
    texture.size = texture.height * texture.width;

    texture.pixels = RgbaArray { inner: pixels }.to_rgb();

    texture
}
