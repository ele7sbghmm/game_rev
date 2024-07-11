use nom::{multi::count, number::complete::le_u32, Parser};

use crate::aa::Texture;

#[derive(Debug, Clone)]
struct Header {
    _02: u32,
    _36: u32,
    _00: u32,
    num_of_textures: usize,
    _max_width: u32,
    _max_height: u32,
}
impl Header {
    fn from_bytes<'a>() -> impl Parser<&'a [u8], Header, ()> {
        move |input: &'a [u8]| {
            let (input, _02) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (input, _36) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (input, _00) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (input, num_of_textures) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (input, _max_width) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (input, _max_height) = le_u32::<&'a [u8], ()>(input).unwrap();

            let num_of_textures = num_of_textures as usize;

            Ok((
                input,
                Header {
                    _02,
                    _36,
                    _00,
                    num_of_textures,
                    _max_width,
                    _max_height,
                },
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dxt {
    header: Header,
    pub textures: Vec<Texture>,
}
impl Dxt {
    pub fn from_bytes<'a>() -> impl Parser<&'a [u8], Dxt, ()> {
        move |input: &'a [u8]| {
            let (input, header) = Header::from_bytes().parse(input).unwrap();
            let (input, textures) =
                count(Texture::from_bytes(), header.num_of_textures)(input).unwrap();

            Ok((input, Dxt { header, textures }))
        }
    }
}
