#![allow(unused)]

use std::{
    default::Default,
    fmt::Debug
};
use nom::{
    IResult, Parser, error::ParseError,
    bytes::complete::take,
    multi::{count, many0},
    number::complete::{be_u32, le_f32, le_u8, le_u32},
    sequence::tuple,
};

use crate::utils::{Vec3f, RGBA};
use crate::chunks::{FromBytes, Header, Chunks, chunk_wrapper,
    X03_00_01_00,
    X04_00_01_00,
    X05_00_01_00,
    X06_00_01_00,
    X07_00_01_00,
    X08_00_01_00,
    X0A_00_01_00,
    X10_00_01_00,
};
use crate::weird_chunks::{X03, X04, X06, X10};

pub fn col<'a, E>() -> Box<dyn Parser<&'a [u8], Chunks, E>>
where E: ParseError<&'a [u8]> + Debug {
    Box::new(move |input: &'a [u8],| {
        let (_, root_header) = Header::from_bytes::<E>(input).unwrap();
        let (input, root) = Root::from_bytes::<E>(input).unwrap();
        dbg!(&root);

        let (_, x21_header) = Header::from_bytes::<E>(input).unwrap();
        dbg!(&x21_header);
        let (input, x21) = take::<u32, &'a [u8], E>(x21_header.ds)(input).unwrap();

        let (input, x20s) = count::<
            &'a [u8], X20, E, fn(&'a [u8]) -> IResult<&'a [u8], X20, E>
        >(chunk_wrapper::<E, X20>, root.num as usize)(input).unwrap();

        let (_, x00_20_12_header) = Header::from_bytes::<E>(input)?;//.unwrap();
        let (input, x00_20_12) = take::<u32, &'a [u8], E>(x00_20_12_header.ds)(input)?;//.unwrap();
        let (_, x01_20_12_header) = Header::from_bytes::<E>(input)?;//.unwrap();
        let (input, x01_20_12) = take::<u32, &'a [u8], E>(x01_20_12_header.ds)(input)?;//.unwrap();
        let (_, x17_header) = Header::from_bytes::<E>(input)?;//.unwrap();
        let (input, x17) = take::<u32, &'a [u8], E>(x17_header.ds)(input)?;//.unwrap();
        let (_, x1d_header) = Header::from_bytes::<E>(input)?;//.unwrap();
        let (input, x1d) = take::<u32, &'a [u8], E>(x1d_header.ds)(input)?;//.unwrap();
        let (_, x1d_header) = Header::from_bytes::<E>(input)?;//.unwrap();
        let (input, x1d) = take::<u32, &'a [u8], E>(x1d_header.ds)(input)?;//.unwrap();

        let (input, x03) = chunk_wrapper::<E, X03>(input)?;//.unwrap();
        let (input, x04) = chunk_wrapper::<E, X04>(input)?;//.unwrap();

        Ok((input, Chunks::Col(root, x20s)))
    })
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Root {
    pub name_len: u8,
    pub name: String,
    pub _null: u32,
    pub num: u32,
}
impl Root {
    pub fn new() -> Root { Default::default() }
}
impl FromBytes for Root {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Root, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, name_len) = le_u8::<&'a [u8], E>(input).unwrap();
        let (input, name_bytes) = take::<u8, &'a [u8], E>(name_len)(input).unwrap();
        let (input, _null) = le_u32::<&'a [u8], E>(input).unwrap();
        let (input, num) = le_u32::<&'a [u8], E>(input).unwrap();

        let name = String::from_utf8_lossy(name_bytes).to_string();

        Ok((input, Root {name_len, name, _null, num}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Vb {pub inner: Vec<Vec3f>}
impl Vb {
    pub fn new() -> Vb { Default::default() }
}
impl FromBytes for Vb {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Vb, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, num) = le_u32(input)?;  
        let (input, vb) = count::<
            &'a [u8], Vec3f, E,
            fn(&'a [u8]) -> IResult<&'a [u8], Vec3f, E>
        >(Vec3f::from_bytes, num as usize)(input)?;
        Ok((input, Vb {inner: vb}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Norm {a: f32, b: f32}
impl FromBytes for Norm {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Norm, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, a) = le_f32(input)?;  
        let (input, b) = le_f32(input)?;  
        Ok((input, Norm {a, b}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Vn {
    pub null: u32,
    pub inner: Vec<Norm>,
}
impl Vn {
    pub fn new() -> Vn { Default::default() }
}
impl FromBytes for Vn {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Vn, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, num) = le_u32(input)?;  
        let (input, null) = le_u32(input)?;  
        let (input, vn) = count::<
            &'a [u8], Norm, E,
            fn(&'a [u8]) -> IResult<&'a [u8], Norm, E>
        >(
            Norm::from_bytes,
            num as usize
        )(input)?;
        Ok((input, Vn {null, inner: vn}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Vc {
    pub num: u32,
    pub inner: Vec<RGBA<u8>>
}
impl Vc {
    pub fn new() -> Vc { Default::default() }
    pub fn rbg2bgr(&self) -> Vc {
        // let mut v: Vec<RGBA<u8> = vec![];
        let inner: Vec<RGBA<u8>> = self.inner.iter().map(|v: &RGBA<u8>| v.swap("BGRA")).collect();
        Vc {num: inner.len() as u32, inner}
    }
}
impl FromBytes for Vc {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Vc, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, num) = le_u32(input)?;  
        let (input, vc) = count::<
            &'a [u8], RGBA<u8>, E,
            fn(&'a [u8]) -> IResult<&'a [u8], RGBA<u8>, E>
        >(RGBA::from_bytes, num as usize)(input)?;
        Ok((input, Vc {num, inner: vc}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ib {pub inner: Vec<u32>}
impl Ib {
    pub fn new() -> Ib { Default::default() }
}
impl FromBytes for Ib {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Ib, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, num) = le_u32(input)?;  
        let (input, ib) = count::<
            &'a [u8], u32, E,
            fn(&'a [u8]) -> IResult<&'a [u8], u32, E>
        >(le_u32, num as usize)(input)?;
        Ok((input, Ib {inner: ib}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct X20 {
    pub null_1: u32,
    pub name_len: u8,
    pub name: String,
    pub null_2: u32, fake_header_id: u32, fake_header_dw: u32, fake_header_cs: u32, null_3: u32, null_4: u32, one: u32, null_5: u32, null_6: u32,
    pub vb: Option<Vb>,
    pub x10: Option<X10>,
    pub x06: Option<X06>,
    pub vn: Option<Vn>,
    pub vc: Option<Vc>,
    pub ib: Option<Ib>,
    pub x03: Option<X03>,
    pub x04: Option<X04>,
}
impl FromBytes for X20 {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X20, E>
        where
            E: ParseError<&'a [u8]> + Debug {
        let (input, (null_1, name_len)) = tuple((le_u32::<&'a [u8], E>, le_u8))(input).unwrap();
        let (input, name_bytes) = take::<u8, &'a [u8], E>(name_len)(input).unwrap();
        let name = String::from_utf8_lossy(name_bytes).to_string();

        println!("{}", &name);

        let (mut input, (
            null_2, fake_header_id, fake_header_dw,
            fake_header_cs, null_3, null_4, one, null_5, null_6
        )) = tuple((
            le_u32::<&'a [u8], E>, le_u32, le_u32,
            le_u32, le_u32, le_u32, le_u32, le_u32, le_u32,
        ))(input).unwrap();

        let (
            mut vb,
            mut vn,
            mut vc,
            mut ib,
            mut x10,
            mut x06,
            mut x03,
            mut x04,
            mut x27,
            mut x26,
            mut x11,
        ) = Default::default();
        let (
            mut xx03,
            mut xx04,
            mut xx05,
            mut xx06,
            mut xx07,
            mut xx08,
            mut xx0a,
            mut xx10,
        ) = Default::default();
        loop {
            let (_, header) = Header::from_bytes::<E>(input)?;
            match header.id {
                X03_00_01_00 => {
                    (input, xx03) = chunk_wrapper::<E, X03>(input).unwrap();
                    x03 = Some(xx03);
                }
                X04_00_01_00 => {
                    (input, xx04) = chunk_wrapper::<E, X04>(input).unwrap();
                    x04 = Some(xx04);
                }
                X05_00_01_00 => {
                    (input, xx05) = chunk_wrapper::<E, Vb>(input).unwrap();
                    vb = Some(xx05);
                }
                X06_00_01_00 => {
                    (input, xx06) = chunk_wrapper::<E, X06>(input).unwrap();
                    x06 = Some(xx06);
                }
                X07_00_01_00 => {
                    (input, xx07) = chunk_wrapper::<E, Vn>(input).unwrap();
                    vn = Some(xx07);
                }
                X08_00_01_00 => {
                    (input, xx08) = chunk_wrapper::<E, Vc>(input).unwrap();
                    vc = Some(xx08);
                }
                X0A_00_01_00 => {
                    (input, xx0a) = chunk_wrapper::<E, Ib>(input).unwrap();
                    ib = Some(xx0a);
                }
                X10_00_01_00 => {
                    (input, xx10) = chunk_wrapper::<E, X10>(input).unwrap();
                    x10 = Some(xx10);
                }
                0x26_00_01_00u32 => { (input, x26) = take::<u32, &'a [u8], E>(header.ds)(input).unwrap(); }
                0x27_00_01_00u32 => { (input, x27) = take::<u32, &'a [u8], E>(header.ds)(input).unwrap(); }
                0x11_00_01_00u32 => { (input, x11) = take::<u32, &'a [u8], E>(header.ds)(input).unwrap(); }
                // _ => panic!(" unknown col id {}", header.id),
                _ => break,
            }
        }

        Ok((input, X20 {
            null_1, name_len, name, null_2, fake_header_id, fake_header_dw,
            fake_header_cs, null_3, null_4, one, null_5, null_6,
            vb, x10, x06, vn, vc, ib, x03, x04,
        }))
    }
}
