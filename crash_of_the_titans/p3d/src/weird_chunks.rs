use std::fmt::Debug;
use nom::{
    IResult,
    error::ParseError,
    multi::count,
    number::complete::{le_f32, le_u8, le_u32},
    sequence::tuple,
};

use crate::utils::Vec3f;
use crate::chunks::FromBytes;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct X03 {
    f1: f32,
    f2: f32,
    f3: f32,
    f4: f32,
    f5: f32,
    f6: f32,
}
impl FromBytes for X03 {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X03, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, (f1, f2, f3, f4, f5, f6)) = tuple((
            le_f32::<&'a [u8], E>, le_f32, le_f32, le_f32, le_f32, le_f32
        ))(input).unwrap();
        Ok((input, X03 {f1, f2, f3, f4, f5, f6}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct X04 {
    f1: f32,
    f2: f32,
    f3: f32,
    f4: f32,
}
impl FromBytes for X04 {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X04, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        // let (input, (f1, f2, f3, f4)) = tuple((
        //     le_f32::<&'a [u8], E>, le_f32, le_f32, le_f32
        // ))(input).unwrap();
        dbg!("before", input.len());
        let (input, f1) = le_f32::<&'a [u8], ()>(input).unwrap();
        let (input, f2) = le_f32::<&'a [u8], ()>(input).unwrap();
        let (input, f3) = le_f32::<&'a [u8], ()>(input).unwrap();
        let (input, f4) = le_f32::<&'a [u8], ()>(input).unwrap();
        // dbg!(&f2);
        Ok((input, X04 {f1, f2, f3, f4}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct X06 {
    pub num: u32,
    pub normals: Vec<Vec3f>,
}
impl X06{ pub fn new() -> X06 { Default::default() }}
impl FromBytes for X06 {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X06, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, num) = le_u32(input)?;  
        let (input, normals) = count::<
            &'a [u8], Vec3f, E, fn(&'a [u8]) -> IResult<&'a [u8], Vec3f, E>
        >(Vec3f::from_bytes, num as usize)(input)?;
        Ok((input, X06 {num, normals}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct X10 {
    num: u32,
    bytes: Vec<u8>,
}
impl X10 { pub fn new() -> X10 { Default::default() }}
impl FromBytes for X10 {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X10, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, num) = le_u32::<&'a [u8], E>(input).unwrap();
        let (input, bytes) = count::<
            &'a [u8], u8, E, fn (&'a [u8]) -> IResult<&'a [u8], u8, E>
        >(le_u8, num as usize)(input).unwrap();

        Ok((input, X10 {num, bytes}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct X11 {
    nl: u8,
}
impl FromBytes for X11 {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X11, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, nl) = le_u8::<&'a [u8], E>(input).unwrap();
        Ok((input, X11 {nl}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct X17 {
    one: u32,
}
impl FromBytes for X17 {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X17, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, one) = le_u32::<&'a [u8], E>(input).unwrap();
        Ok((input, X17 {one}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct X1d {
    null: u32,
}
impl FromBytes for X1d {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X1d, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, null) = le_u32::<&'a [u8], E>(input).unwrap();
        Ok((input, X1d {null}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct X21 {
    null: u32,
    thirty_two_1: u32,
    thirty_two_2: u32,
    thirty_two_3: u32,
    thirty_two_4: u32,
    thirty_two_5: u32,
    thirty_two_6: u32,
    thirty_two_7: u32,
}
impl FromBytes for X21 {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X21, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (
            input, (
                null,
                thirty_two_1,
                thirty_two_2,
                thirty_two_3,
                thirty_two_4,
                thirty_two_5,
                thirty_two_6,
                thirty_two_7,
        )) = tuple((
            le_u32::<&'a [u8], E>,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
        ))(input).unwrap();
        let x21 = X21 {
            null,
            thirty_two_1,
            thirty_two_2,
            thirty_two_3,
            thirty_two_4,
            thirty_two_5,
            thirty_two_6,
            thirty_two_7,
        };

        Ok((
            input,
            x21
        ))
    }
}
#[derive(Debug, Default, PartialEq, Clone)]
pub struct X26 {
    null: u32,
}
impl FromBytes for X26 {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X26, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, null) = le_u32::<&'a [u8], E>(input).unwrap();
        Ok((input, X26 {null}))
    }
}
#[derive(Debug, Default, PartialEq, Clone)]
pub struct X27 {
    null: u32,
}
impl FromBytes for X27 {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X27, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, null) = le_u32::<&'a [u8], E>(input).unwrap();
        Ok((input, X27 {null}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct X00_20_12 {
    f1: f32,
    f2: f32,
}
impl FromBytes for X00_20_12  {
    fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], X00_20_12  , E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, (f1, f2)) = tuple((
            le_f32::<&'a [u8], E>, le_f32
        ))(input).unwrap();
        Ok((input, X00_20_12 {f1, f2}))
    }
}
#[derive(Debug, Default, PartialEq, Clone)]
pub struct X01_20_12 {}
