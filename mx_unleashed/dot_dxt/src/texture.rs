use nom::{multi::count, number::complete::le_u32, sequence::tuple, Parser};

use crate::color::Rgba;
use crate::utils::write_file;

#[derive(Debug, Clone)]
struct Dxt {
    c0: Rgba,
    c1: Rgba,
    c2: Rgba,
    c3: Rgba,
    _20004108: u32,
    _aaaaaaaa: u32,
}
impl Dxt {
    fn from_bytes<'a>() -> impl Parser<&'a [u8], Dxt, ()> {
        move |input: &'a [u8]| {
            let (input, (c0, c1, c2, c3, _20004108, _aaaaaaaa)) = tuple((
                Rgba::from_bytes_bgr565(),
                Rgba::from_bytes_bgr565(),
                Rgba::from_bytes_bgr565(),
                Rgba::from_bytes_bgr565(),
                le_u32,
                le_u32,
            ))(input)
            .unwrap();
            Ok((
                input,
                Dxt {
                    c0,
                    c1,
                    c2,
                    c3,
                    _20004108,
                    _aaaaaaaa,
                },
            ))
        }
    }
    fn colors_to_bytes(&self) -> [Rgba; 4] {
        let Dxt {
            c0,
            c1,
            c2,
            c3,
            _20004108,
            _aaaaaaaa,
        } = self;
        [c0.clone(), c1.clone(), c2.clone(), c3.clone()]
    }
}

#[derive(Debug, Clone)]
struct DxtArray {
    inner: Vec<Dxt>,
}
impl DxtArray {
    fn from_bytes<'a>(size: usize) -> impl Parser<&'a [u8], DxtArray, ()> {
        move |input: &'a [u8]| {
            let (input, inner) = count(Dxt::from_bytes(), size)(input).unwrap();

            Ok((input, DxtArray { inner }))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    width: usize,
    height: usize,
    size: usize,
    dxts: DxtArray,
}
impl Texture {
    pub fn from_bytes<'a>() -> impl Parser<&'a [u8], Texture, ()> {
        move |input: &'a [u8]| {
            let (input, (width, height, size)) =
                tuple((le_u32::<&'a [u8], ()>, le_u32, le_u32))(input).unwrap();
            let (width, height, size) = (width as usize, height as usize, size as usize);

            let colors_per_dxt = 16;
            let dxts_in_texture_size = (size as f32 / colors_per_dxt as f32).floor() as usize;
            let min_dxts = std::cmp::max(1, dxts_in_texture_size);

            println!("width {width:x}");
            let (input, dxts) = DxtArray::from_bytes(min_dxts).parse(input).unwrap();

            Ok((
                input,
                Texture {
                    width,
                    height,
                    size,
                    dxts,
                },
            ))
        }
    }
    pub fn to_ppm(&self, path: String) {
        let dim = 2;
        let mut buffer: Vec<u8> = vec![];
        let width = self.width / 4;
        let height = self.height / 4;

        for row in 0..height {
            for y in 0..dim {
                for column in 0..width {
                    let dxt_index = (row * width + column) as usize;
                    dbg!(dxt_index);
                    let dxt = self.dxts.inner[dxt_index].clone();
                    for x in 0..dim {
                        let pixel_index = (y * dim + x) as usize;
                        let color = dxt.colors_to_bytes()[pixel_index].clone();
                        dbg!(pixel_index);
                        dbg!(&color);
                        buffer.append(&mut color.to_bytes().to_vec());
                    }
                }
            }
        }

        let h = format!(
            "P7\nWIDTH {}\nHEIGHT {}\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n",
            self.width / 2,
            self.height / 2
        );
        write_file(vec![h.as_bytes().to_vec(), buffer], path);
    }
}

pub fn u32_to_2bit_array(value: u32) -> Vec<u8> {
    let b0: u32 = (value & 0xff000000) >> 24;
    let b1: u32 = (value & 0xff0000) >> 16;
    let b2: u32 = (value & 0xff00) >> 8;
    let b3: u32 = value & 0xff;
    let (b0, b1, b2, b3) = (b0 as u8, b1 as u8, b2 as u8, b3 as u8);

    let mut b0s = u8_to_2bit_array(b0);
    let mut b1s = u8_to_2bit_array(b1);
    let mut b2s = u8_to_2bit_array(b2);
    let mut b3s = u8_to_2bit_array(b3);

    let mut out: Vec<u8> = vec![];
    out.append(&mut b0s);
    out.append(&mut b1s);
    out.append(&mut b2s);
    out.append(&mut b3s);

    out
}
pub fn u8_to_2bit_array(value: u8) -> Vec<u8> {
    let n0: u8 = (value & 0b1100_0000) >> 6;
    let n1: u8 = (value & 0b11_0000) >> 4;
    let n2: u8 = (value & 0b1100) >> 2;
    let n3: u8 = value & 0b11;
    let (n0, n1, n2, n3) = (n0 as u8, n1 as u8, n2 as u8, n3 as u8);

    vec![n0, n1, n2, n3]
}
