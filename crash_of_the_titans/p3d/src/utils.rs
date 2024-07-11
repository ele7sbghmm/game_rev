use std::{
    default::Default,
    fmt::Debug,
};
use nom::{
    IResult,
    error::ParseError,
    number::complete::{le_f32, le_u8},
    sequence::tuple,
};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Vec3f {pub x: f32, pub y: f32, pub z: f32}
impl Vec3f {
    // pub fn new() -> Vec3f { Default::default() }
    pub fn from_bytes<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], Vec3f, E> {
        let (input, (x, y, z)) =
            tuple((le_f32, le_f32, le_f32))(input)?;
        Ok((input, Vec3f {x, y, z}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RGBA<T> {pub r: T, pub g: T, pub b: T, pub a: T}
impl<T: Clone> RGBA<T> {
    pub fn swap(&self, s: &str) -> RGBA<T> {
        let RGBA {r, g, b, a} = &self;
        match s {
            "BGRA" => { RGBA {r: self.b.clone(), g: self.g.clone(), b: self.r.clone(), a: self.a.clone()} },
            "ABGR" => { RGBA {r: self.a.clone(), g: self.b.clone(), b: self.g.clone(), a: self.r.clone()} },
            _      => { RGBA {r: self.r.clone(), g: self.g.clone(), b: self.b.clone(), a: self.a.clone()} },
        }
    }
}
impl RGBA<u8> {
    pub fn new() -> RGBA<u8> { Default::default() }
    pub fn from_bytes<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], RGBA<u8>, E>
        where
            E: ParseError<&'a [u8]> + Debug, {
        let (input, (r, g, b, a)) =
            tuple((le_u8, le_u8, le_u8, le_u8))(input)?;
        Ok((input, RGBA {r, g, b, a}))
    }
    pub fn float(&self) -> RGBA<f32> {
        RGBA {r: self.r as f32, g: self.g as f32, b: self.b as f32, a: self.a as f32}
    }
}
impl RGBA<f32> {
    pub fn to_one(&self) -> RGBA<f32> {
        RGBA {r: self.r/255., g: self.g/255., b: self.b/255., a: self.a/255.}
    }
}
