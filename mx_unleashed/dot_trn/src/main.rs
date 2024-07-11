#![allow(unused)]

use nom::{
    bytes::complete::take,
    multi::count,
    number::complete::{le_f32, le_u16, le_u32, le_u8},
    sequence::tuple,
    IResult, Parser,
};
use std::{
    cmp,
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

mod utils;
use utils::{read_file, write_file, Header, Vec4f};
mod vertex;
use vertex::{Vertex, VertexArray, VertexSectionFree, VertexSectionSx};
mod color;
use color::{Color, Rgba, RgbaArray};
mod thing;
use thing::{_7c31, _7c31Array, FREE};
mod ppm;
use ppm::{BufferType, Ppm, TuplType};
mod texture;
use texture::{texture_array_to_giant, Texture};
mod trn;
use trn::{Release, Track, Trn};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file_name: String = match args.len() {
        2 => args[1].clone(),
        _ => "/Users/timmeh/Desktop/mx_unleashed/trn_proto/free06.ps2.trn".to_string(),
    };
    dbg!(&file_name);

    let mut buf: Vec<u8> = vec![];
    let _ = read_file(file_name, &mut buf);

    let input = buf.as_slice();
    let clean_input = buf.as_slice();

    let (input, header) = Header::parse(Release::Proto, Track::Free, input).unwrap();

    let mut _7c31: Vec<Vec<u8>> = vec![vec![]; 841];
    let mut _aeb3: Vec<Vec<u8>> = vec![vec![]; 841];
    let mut _7f: Vec<Vec<u8>> = vec![vec![]; 841];
    let mut _00: Vec<Vec<u8>> = vec![vec![]; 841];
    let mut _0f: Vec<Vec<u8>> = vec![vec![]; 841];

    let mut out_7c31: Vec<u8> = vec![];
    let mut out_aeb3: Vec<u8> = vec![];
    let mut out_7f: Vec<u8> = vec![];
    let mut out_00: Vec<u8> = vec![];
    let mut out_0f: Vec<u8> = vec![];

    for y in FREE[0..1].iter() {
        for range in y.iter() {
            for index in range.clone() {
                let offset = &header.offset_table[index];
                let input = &clean_input[*offset..];

                let (input, tmp_7c31) = count(le_u8::<&[u8], ()>, 4)(input).unwrap();
                let (input, tmp_aeb3) = count(le_u8::<&[u8], ()>, 40 * 17)(input).unwrap();
                let (input, tmp_7f) = count(le_u8::<&[u8], ()>, 3 * 17 * 17)(input).unwrap();
                let (input, tmp_00) = count(le_u8::<&[u8], ()>, 17 * 17)(input).unwrap();
                let (input, tmp_0f) = count(le_u8::<&[u8], ()>, 17 * 17)(input).unwrap();

                _7c31[index] = tmp_7c31;
                _aeb3[index] = tmp_aeb3;
                _7f[index] = tmp_7f;
                _00[index] = tmp_00;
                _0f[index] = tmp_0f;

                // _7c31 [u8;             4],
                // _aeb3 [[u8;      40]; 17],
                // _7f   [[[u8; 3]; 17]; 17],
                // _00   [[u8;      17]; 17],
                // _0f   [[u8;      17]; 17],
            }
        }
    }

    for y in 0..43 {
        for line in 0..17 {
            for x in 0..24 {
                for pixel in 0..17 {
                    let block_offset = 17 * 17 * (y * 24 + x);
                    let pixel_offset = line * 17 + pixel;

                    out_7c31.append(&mut _7c31[block_offset + pixel_offset]);
                    out_7f.append(&mut _7f[block_offset + pixel_offset]);
                    out_00.append(&mut _00[block_offset + pixel_offset]);
                    out_0f.append(&mut _0f[block_offset + pixel_offset]);
                }
                for pixel in 0..40 {
                    let block_offset = 17 * 40 * (y * 24 + x);
                    let pixel_offset = line * 40 + pixel;
                    out_aeb3.append(&mut _aeb3[block_offset + pixel_offset]);
                }
            }
        }
    }

    write_file(
        PathBuf::from("/Users/timmeh/Desktop/7c31/7c31.bin"),
        out_7c31.as_slice(),
    );
    write_file(
        PathBuf::from("/Users/timmeh/Desktop/7c31/aeb3.bin"),
        out_aeb3.as_slice(),
    );
    write_file(
        PathBuf::from("/Users/timmeh/Desktop/7c31/7f.bin"),
        out_7f.as_slice(),
    );
    write_file(
        PathBuf::from("/Users/timmeh/Desktop/7c31/00.bin"),
        out_00.as_slice(),
    );
    write_file(
        PathBuf::from("/Users/timmeh/Desktop/7c31/0f.bin"),
        out_0f.as_slice(),
    );

    // let mut textures: Vec<Texture> = vec![];

    // for offset in header.texture_offsets.iter() {
    //     let mut tex = Texture::parse(offset.clone(), &header, clean_input);
    //     textures.push(tex.clone());
    // }

    // let outpath = &format!("/tmp/textures/{}.ppm", &file_name[file_name.len() - 13..]);
    // println!("done {}", outpath);
    // texture_array_to_giant(256, 256, textures).to_ppm(Path::new(outpath));

    Ok(())
}

// fn hsv2rgb() -> () {
//     let mut array = RgbaArray { inner: vec![] };

//     for i in 0..100 {
//         let rgb = Rgba::from_hsv((i as f32), 0.0, 100.0);
//         println!("{} {:?}", i, &rgb);
//         array.inner.push(rgb);
//     }

//     Ppm::new(
//         TuplType::RgbAlpha,
//         10,
//         10,
//         255,
//         BufferType::Buffer(array.to_bytes()),
//     )
//     .ppm_write(Path::new("/Users/timmeh/Desktop/rainbow.ppm"), None);
// }
