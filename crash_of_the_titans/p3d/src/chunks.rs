use std::{
    default::Default,
    fmt::Debug,
};
use nom::{
    IResult,
    Parser,
    error::ParseError,
    bytes::complete::take,
    multi::{count, many0},
    number::complete::{be_u32, le_u8, le_u32, le_f32},
    sequence::tuple,
};

use crate::col::{Root, X20, col};//, ColorTable, Normals};

pub const ROOT: u32         = 0x00_00_01_00;
pub const X03_00_01_00: u32 = 0x03_00_01_00;
pub const X04_00_01_00: u32 = 0x04_00_01_00;
pub const X05_00_01_00: u32 = 0x05_00_01_00;
pub const X06_00_01_00: u32 = 0x06_00_01_00;
pub const X07_00_01_00: u32 = 0x07_00_01_00;
pub const X08_00_01_00: u32 = 0x08_00_01_00;
pub const X0A_00_01_00: u32 = 0x0a_00_01_00;
// pub const IB: u32           = 0x0a_00_01_00;

pub const X10_00_01_00: u32 = 0x10_00_01_00;
// pub const X11_00_01_00: u32 = 0x11_00_01_00;
// pub const X17_00_01_00: u32 = 0x17_00_01_00;
// pub const X1D_00_01_00: u32 = 0x1d_00_01_00;
// pub const X20_00_01_00: u32 = 0x20_00_01_00;
// pub const X21_00_01_00: u32 = 0x21_00_01_00;
// pub const X26_00_01_00: u32 = 0x26_00_01_00;
// pub const X27_00_01_00: u32 = 0x27_00_01_00;

// const X00_20_12_00: u32 = 0x00_20_12_00;
// const X01_20_12_00: u32 = 0x01_20_12_00;

pub trait FromBytes {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Self, E>
        where
            Self: Sized,
            E: ParseError<&'a [u8]> + Debug;
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Header {pub id: u32, pub ds: u32, pub cs: u32}
impl Header {
    pub fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Header, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        // dbg!(&input[..12]);
        let (input, (id, ds, cs)) =
            tuple((be_u32::<&'a [u8], E>, le_u32, le_u32))(input)?;//.unwrap();
        Ok((input, Header {id, ds, cs}))
    }
}

pub fn chunk_wrapper<'a, E, T>(input: &'a [u8]) -> IResult<&'a [u8], T, E>
    where
        E: ParseError<&'a [u8]> + Debug,
        T: FromBytes, {
    let (input, header) = Header::from_bytes(input)?;
    println!("{:x}", &header.id);
    T::from_bytes(input)
}

pub fn skip<'a, E>() -> Box<dyn Parser<&'a [u8], Chunks, E>>
    where
        E: ParseError<&'a [u8]> + Debug, {
    Box::new(move |input: &'a [u8]| {
        Ok((input, Chunks::Skip))
    })
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Chunks {
    #[default]
    Skip,
    Col(Root, Vec<X20>),    
}
impl Chunks {
    pub fn from_id<'a, E>(id: u32) -> Box<dyn Parser<&'a [u8], Chunks, E>>
        where
            E: ParseError<&'a [u8]> + Debug, {
        match id {
            0x00_00_01_00 => col(),
            _             => skip(),
        }
    }
}

pub fn chunk_paris<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], ChunkType, E>
    where
        E: ParseError<&'a [u8]> + Debug, {
    const HEADER_SIZE: u32 = 12;
    
    // let mut input = input;
    let (mut input, header_bytes) =
        take::<u32, &'a [u8], E>(HEADER_SIZE)(input)?;
    let (_, header) = Header::from_bytes::<E>(header_bytes).unwrap();
    println!("id={:x} ds={:x} cs={:x}", header.id.swap_bytes(), header.ds, header.cs);

    let chunkslice_size = header.cs-HEADER_SIZE;
    let dataslice_size = header.ds-HEADER_SIZE;

    let mut data_slice: &[u8] = &[];
    let mut chunk_slice: &[u8] = &[];

    if let Chunks::Skip = Chunks::from_id::<E>(header.id).parse(input).unwrap().1 {
        (input, chunk_slice) =
            take::<u32, &'a [u8], E>(chunkslice_size)(input).unwrap();
        (chunk_slice, data_slice) =
            take::<u32, &'a [u8], E>(dataslice_size)(chunk_slice).unwrap();
    } else {
        (input, data_slice) =
            take::<u32, &'a [u8], E>(chunkslice_size)(input).unwrap();
    }

    let (_remaining_dataslice, chunk) =
        Chunks::from_id::<E>(header.id).parse(data_slice)?;//.unwrap();
    let (_remaining_chunkslice, sub_chunks) =
        many0(chunk_paris::<E>)(&chunk_slice).unwrap();
    // assert_eq!(_remaining_chunkslice, &[]);

    Ok((input, ChunkType {parent: (chunk, sub_chunks)}))
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChunkType {
    parent: (Chunks, Vec<ChunkType>),
}

pub fn get_chunks(ct: &ChunkType, mut v: &mut Vec<Chunks>) -> Vec<Chunks> {
    match &ct.parent { 
        (Chunks::Col(_, _), _) => { v.push(ct.parent.0.clone()); }

        (_, sub) => {
            for c in sub.iter() {
                get_chunks(c, v);
            };
        }
    };
    v.to_vec()
} 
