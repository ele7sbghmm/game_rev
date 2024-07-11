#![allow(unused)]

use std::fmt::Debug;
use std::default::Default;
use nom::{
    IResult, Parser, error::ParseError,
    bytes::complete::take,
    multi::{count, length_data},
    number::complete::{le_f32, le_u8, le_u32},
    sequence::tuple,
};
use crate::Chunk;
use crate::utils::{Header, Vec3f, Matrix4f};
use crate::paris::{Par, Paris};
use crate::chunks::Def;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Locator {
    pub name: String,
    pub ttype: u32,
    pub position: Vec3f,
    pub elements: Ttype,
    pub triggers: Vec<Trigger>,
}
impl Def for Locator {}
impl Paris for Locator {
    fn paris<'a, E>(&self) -> Box<dyn Parser<&'a [u8], crate::Chunk, E>>
    where Self: Sized + Par, E: ParseError<&'a [u8]> + Debug {
        Box::new(move |input| {
            let (input, locator) =
                Self::par::<E>(input).unwrap();
            Ok((input, Chunk::Locator(locator)))
        })
    }
}
impl Par for Locator {
    fn par<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], Self, E> {
        let (input, name_bytes) =
            length_data::<&'a [u8], u8, E, fn(&'a [u8]) -> IResult<&'a [u8], u8, E>>(
                le_u8::<&'a [u8], E>
            )(input).unwrap();
        let (input, (ttype, size)) =
            tuple((le_u32::<&'a [u8], E>, le_u32))(input).unwrap();
        dbg!(&input, &ttype, &size, input.len());
        let (input, elements) =
            Ttype::paris::<E>(ttype, size).parse(input).unwrap();    
        dbg!(&input);
        let (input, position) = Vec3f::paris::<E>(input).unwrap();
        let (input, num_of_triggers) = le_u32::<&'a [u8], E>(input).unwrap();

        let (input, triggers) =
            count::<&'a [u8], Trigger, E, fn(&'a [u8]) -> IResult<&'a [u8], Trigger, E>>(
                Trigger::par, num_of_triggers as usize
        )(input).unwrap();

        let name = String::from_utf8_lossy(name_bytes).to_string();

        Ok((input, Locator {name, ttype, position, elements, triggers}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Trigger {
    pub name: String,
    pub type_of: u32,
    pub scale: Vec3f,
    pub matrix: Matrix4f,
}
impl Par for Trigger {
    fn par<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], Self, E> {
        let (input, header) = Header::paris::<E>(input).unwrap();
        if header.chunk_id != 0x03_00_00_06 {
            return Ok((input, Trigger {
                name: String::new(),
                type_of: 0u32,
                scale: Vec3f::new(),
                matrix: Matrix4f::new()
            }))
        }
        
        let (input, name_bytes) = length_data::<
            &'a [u8], u8, E, fn(&'a [u8]) -> IResult<&'a [u8], u8, E>
        >(le_u8::<&'a [u8], E>)(input).unwrap();
        let (input, type_of) = le_u32::<&'a [u8], E>(input).unwrap();
        let (input, scale) = Vec3f::paris::<E>(input).unwrap();
        let (input, matrix) = Matrix4f::paris::<E>(input).unwrap();

        let name = String::from_utf8_lossy(name_bytes).to_string();
        
        Ok((input, Trigger {name, type_of, scale, matrix}))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Ttype {
    #[default]
    Skip,       // 
    // Event(Vec<Vec3f>),    // 0
    // Script(),             // 1
    // Generic(),            // 2
    CarStart(Vec<Vec3f>), // 3
    // Spline(),             // 4
    DynamicZone(String),  // 5
    // Occlusion(),          // 6
    // InteriorEntrance() ,  // 7
    // Directional(),        // 8
    // Action(),             // 9
    // FOV(),                // A
    // BreakableCamera(),    // B
    // StaticCamera(),       // C
    // PedGroup(),           // D
    // Coin(),               // E
    // SpawnPoin(),          // F
}
impl Ttype {
    pub fn from_id(id: u32) -> Self {
        match id {
            3 => Ttype::CarStart(vec![]),
            5 => Ttype::DynamicZone("".to_string()),
            _ => Ttype::Skip,
        }
    }
    fn paris<'a, E: ParseError<&'a [u8]> + Debug>(
        ttype: u32, num: u32
    ) -> impl Parser<&'a [u8], Ttype, E> {
        move |input: &'a [u8]| {
            match ttype {
                5 => {
                    let (input, bytes) =
                        take::<u32, &'a [u8], E>(num * 4)(input).unwrap();
                    let s = String::from_utf8_lossy(bytes).to_string();
                    Ok((input, Ttype::DynamicZone(s)))
                },
                _ => {
                    let (input, _trigger) =
                        take::<u32, &'a [u8], E>(num * 4)(input).unwrap();
                    Ok((input, Ttype::Skip))
                },
            }
        }
    }
}
