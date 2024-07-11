use std::{
    default::Default,
    fmt,
    fs::OpenOptions,
    io::{self, Read, Write},
    path::PathBuf,
};

use nom::{
    multi::count,
    number::complete::{le_f32, le_u32, le_u8},
    sequence::tuple,
    IResult, Parser,
};

use crate::color::{Rgba, RgbaArray};
use crate::texture::Texture;
use crate::trn::{Release, Track, Trn};

pub trait FromBytes
where
    Self: Default,
{
    fn from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        Ok((input, Self::default()))
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl FromBytes for Vec3f {
    fn from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, (x, y, z)) = tuple((le_f32::<&'a [u8], ()>, le_f32, le_f32))(input).unwrap();
        Ok((input, Self { x, y, z }))
    }
}
impl fmt::Display for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6} {:.6} {:.6}", self.x, self.y, self.z)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec4f {
    pub fn to_vec3f(&self) -> Vec3f {
        let Vec4f { x, y, z, w } = self.clone();
        Vec3f { x, y, z }
    }
}
impl FromBytes for Vec4f {
    fn from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, (w, x, y, z)) =
            tuple((le_f32::<&'a [u8], ()>, le_f32, le_f32, le_f32))(input).unwrap();
        Ok((input, Self { x, y, z, w }))
    }
}
impl std::ops::Add for Vec4f {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let (x, y, z, w) = (
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        );
        Self { x, y, z, w }
    }
}
impl std::ops::Sub for Vec4f {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let (x, y, z, w) = (
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        );
        Self { x, y, z, w }
    }
}
impl fmt::Display for Vec4f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} {:.2} {:.2} {:.2}", self.x, self.y, self.z, self.w,)
    }
}

pub fn read_file(file_name: String, mut buf: &mut Vec<u8>) {
    // let path_in: String = format!("/Users/timmeh/Desktop/{}", file_name);

    let _ = OpenOptions::new()
        .read(true)
        .open(file_name)
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
}

pub fn write_file(path: PathBuf, buf: &[u8]) -> io::Result<usize> {
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();
    f.write(buf)
}

pub type OffsetTable = Vec<usize>;

#[derive(Default, Debug)]
pub struct Header {
    pub pln: Vec<u8>,       // [u8; 4] "PLN\0"
    pub seven_ps2: Vec<u8>, // [u8; 5] "7ps2\0"
    pub psxpal8: Vec<u8>,   // [u8; 32], "UnCompressedPSXPal8"
    pub one: u32,
    pub colortable: RgbaArray, // 0x801b4449
    pub float: f32,
    pub two: u32,
    pub vertices_offset: usize,
    pub num_7c31_offsets: usize,
    pub num_7c31_offsets_2: usize,
    pub offset_table: OffsetTable,
    pub num_of_textures: u32,
    pub num_of_textures_2: u32,
    pub texture_offsets: Vec<usize>,
}
impl Header {
    pub fn parse<'a>(
        release: Release,
        track: Track,
        input: &'a [u8],
    ) -> IResult<&'a [u8], Header, ()> {
        let (input, pln) = count(le_u8::<&'a [u8], ()>, 4)(input).unwrap();
        let (input, seven_ps2) = count(le_u8::<&'a [u8], ()>, 5)(input).unwrap();
        let (input, psxpal8) = count(le_u8::<&'a [u8], ()>, 32)(input).unwrap();
        let (input, one) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, colortable) = match release {
            Release::Proto => RgbaArray::from_bytes_rgba(0x100).parse(input).unwrap(),
            Release::Xbox => RgbaArray::from_bytes_rgba(0x100).parse(input).unwrap(),
        };
        let (input, float) = le_f32::<&'a [u8], ()>(input).unwrap();
        let (input, two) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, vertices_offset) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, num_7c31_offsets) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, num_7c31_offsets_2) = le_u32::<&'a [u8], ()>(input).unwrap();

        let (input, offset_table) =
            count(le_u32::<&'a [u8], ()>, num_7c31_offsets as usize)(input).unwrap();
        let (input, num_of_textures) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, num_of_textures_2) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, texture_offsets) =
            count(le_u32::<&'a [u8], ()>, num_of_textures as usize)(input).unwrap();

        Ok((
            input,
            Header {
                pln,
                seven_ps2,
                psxpal8,
                one,
                colortable,
                float,
                two,
                vertices_offset: vertices_offset as usize,
                num_7c31_offsets: num_7c31_offsets as usize,
                num_7c31_offsets_2: num_7c31_offsets_2 as usize,
                offset_table: offset_table.iter().map(|num| *num as usize).collect(),
                num_of_textures,
                num_of_textures_2,
                texture_offsets: texture_offsets.iter().map(|num| *num as usize).collect(),
            },
        ))
    }
}
pub enum TrackType {
    Supercross,
    National,
    Freestyle,
}
