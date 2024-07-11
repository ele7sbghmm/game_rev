use nom::{
    multi::count,
    number::complete::{le_u24, le_u32, le_u8},
    IResult, Parser,
};
use std::{
    cmp::{max, min},
    ops::Range,
    path::PathBuf,
};

use crate::color::{Color, Rgba, RgbaArray};
use crate::ppm::{BufferType, Ppm, TuplType};
use crate::utils::{Header, OffsetTable};

pub const FREE: [[Range<usize>; 3]; 43] = [
    [0..1, 129..131, 1..22],
    [387..389, 645..646, 1..22],
    [646..649, 0..0, 1..22],
    [389..405, 0..0, 1..9],
    [405..421, 0..0, 1..9],
    [421..437, 0..0, 1..9],
    [437..453, 0..0, 1..9],
    [453..469, 0..0, 1..9],
    [469..485, 0..0, 1..9],
    [485..501, 0..0, 1..9],
    [501..517, 0..0, 1..9],
    [517..533, 0..0, 1..9],
    [533..549, 0..0, 1..9],
    [549..565, 0..0, 1..9],
    [565..581, 0..0, 1..9],
    [581..597, 0..0, 1..9],
    [597..613, 0..0, 1..9],
    [613..629, 0..0, 1..9],
    [629..645, 0..0, 1..9],
    [1..9, 65..73, 131..139],
    [9..17, 73..81, 139..147],
    [17..25, 81..89, 147..155],
    [25..33, 89..97, 155..163],
    [33..41, 97..105, 163..171],
    [41..49, 105..113, 171..179],
    [49..57, 113..121, 179..187],
    [57..65, 121..129, 187..195],
    [195..203, 259..267, 323..331],
    [203..211, 267..275, 331..339],
    [211..219, 275..283, 339..347],
    [219..227, 283..291, 347..355],
    [227..235, 291..299, 355..363],
    [235..243, 299..307, 363..371],
    [243..251, 307..315, 371..379],
    [251..259, 315..323, 379..387],
    [649..657, 713..721, 777..785],
    [657..665, 721..729, 785..793],
    [665..673, 729..737, 793..801],
    [673..681, 737..745, 801..809],
    [681..689, 745..753, 809..817],
    [689..697, 753..761, 817..825],
    [697..705, 761..769, 825..833],
    [705..713, 769..777, 833..841],
];
const SX: [[Range<usize>; 1]; 16] = [
    [0..16],
    [16..32],
    [32..48],
    [48..64],
    [64..80],
    [80..96],
    [96..112],
    [112..128],
    [128..144],
    [144..160],
    [160..176],
    [176..192],
    [192..208],
    [208..224],
    [224..240],
    [240..256],
];
const NAT: [[Range<usize>; 1]; 16] = [
    [0..16],
    [16..32],
    [32..48],
    [48..64],
    [64..80],
    [80..96],
    [96..112],
    [112..128],
    [128..144],
    [144..160],
    [160..176],
    [176..192],
    [192..208],
    [208..224],
    [224..240],
    [240..256],
];

#[derive(Debug, Clone)]
pub struct _7c31 {
    pub _7c31: u32,
    pub _aeb3: Vec<Vec<u8>>, // [[u8;      40]; 17],
    // pub _aeb3: Vec<RgbaArray>, // [[u8;      40]; 17],
    pub _7f: Vec<RgbaArray>, // [[[u8; 3]; 17]; 17],
    pub _00: Vec<Vec<u8>>,   // [[u8;      17]; 17],
    pub _0f: Vec<Vec<u8>>,   // [[u8;      17]; 17],
}
impl _7c31 {
    pub fn parse(offset: usize, clean_input: &[u8]) -> _7c31 {
        let input = &clean_input[offset..];

        let (input, _7c31) = le_u32::<&[u8], ()>(input).unwrap();
        let (input, _aeb3) = count(count(le_u8::<&[u8], ()>, 40), 17)(input).unwrap();
        let (input, _7f) = count(RgbaArray::from_bytes_rgb(17), 17)(input).unwrap();
        let (input, _00) = count(count(le_u8::<&[u8], ()>, 17), 17)(input).unwrap();
        let (input, _0f) = count(count(le_u8::<&[u8], ()>, 17), 17)(input).unwrap();

        _7c31 {
            _7c31,
            _aeb3,
            _7f,
            _00,
            _0f,
        }
    }
}

pub struct _7c31Array {
    inner: Vec<_7c31>,
}
impl _7c31Array {
    pub fn parse<'a>(clean_input: &'a [u8], header: &Header) -> Self {
        let mut inner: Vec<_7c31> = vec![];
        for offset in &header.offset_table {
            let blob = _7c31::parse(offset.clone(), clean_input);
            inner.push(blob);
        }
        Self { inner }
    }
    pub fn _7f_ppm_free(&self, header: &Header) -> Ppm {
        let mut sevenfs: Vec<u8> = vec![];

        for row in FREE {
            for line in 0..17 {
                for range in row.clone() {
                    for index in range {
                        let cln = &self.inner[index].clone();
                        sevenfs.append(&mut cln._7f[line].to_bytes())
                    }
                }
            }
        }

        let w = 408;
        let h = 731;

        // println!("{} {}", w, h);

        Ppm::new(TuplType::RgbAlpha, w, h, 255, BufferType::Buffer(sevenfs))
    }
    pub fn _aeb3_ppm_sx(&self, header: &Header) -> () {
        let mut sevenfs: Vec<u8> = vec![];

        for row in SX {
            for line in 0..17 {
                for range in row.clone() {
                    for index in range {
                        let cln = &self.inner[index].clone();
                        sevenfs.append(&mut cln._aeb3[line][0..34].to_vec());
                    }
                }
            }
        }

        let w = 408;
        let h = 731;

        // println!("{} {}", w, h);

        let mut ppm = Ppm::new(TuplType::RgbAlpha, w, h, 255, BufferType::Buffer(sevenfs));
        ppm.depth = 8;
        ppm.ppm_write(PathBuf::from("/Users/timmeh/Desktop/sx_aeb3.ppm"), None);
    }
    pub fn _aeb3_ppm_free(&self, header: &Header) -> Ppm {
        let mut sevenfs: Vec<u8> = vec![];

        for row in FREE {
            for line in 0..17 {
                for range in row.clone() {
                    for index in range {
                        let cln = &self.inner[index].clone();
                        sevenfs.append(&mut cln._aeb3[line][0..34].to_vec())
                        // sevenfs.append(&mut cln._aeb3[line].to_bytes()[0..34].to_vec())
                    }
                }
            }
        }

        let w = 408;
        let h = 731;

        // println!("{} {}", w, h);

        Ppm::new(TuplType::GrayScale, w, h, 511, BufferType::Buffer(sevenfs))
    }
    pub fn _0f_ppm_free(&self, header: &Header) -> Ppm {
        let mut sevenfs: Vec<u8> = vec![];

        for row in FREE {
            for line in 0..17 {
                for range in row.clone() {
                    for index in range {
                        let cln = &self.inner[index].clone();
                        sevenfs.append(&mut cln._0f[line].to_owned())
                    }
                }
            }
        }

        let w = 408;
        let h = 731;

        // println!("{} {}", w, h);

        Ppm::new(TuplType::GrayScale, w, h, 255, BufferType::Buffer(sevenfs))
    }
    // pub fn _7f_ppm_sx(&self, header: &Header) -> Ppm {
    //     let mut sevenfs: Vec<u8> = vec![];

    //     for row in SUPER {
    //         for line in 0..17 {
    //             for range in row.clone() {
    //                 for index in range {
    //                     let cln = &self.inner[index].clone();
    //                     sevenfs
    //                         .append(&mut cln._7f[line].strip(false, false, true, true).to_bytes())
    //                 }
    //             }
    //         }
    //     }

    //     let w = 272;
    //     let h = 272;

    //     println!("{} {}", w, h);

    //     Ppm::new(TuplType::RgbAlpha, w, h, 255, BufferType::Buffer(sevenfs))
    // }
}
