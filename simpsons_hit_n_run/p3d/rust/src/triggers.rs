use crate::paris::{Par, Paris};
use crate::utils::{Header, Matrix4f, Vec3f};
use crate::Chunk;
use nom::{
    bytes::complete::take,
    multi::{count, length_data},
    number::complete::{le_f32, le_u32, le_u8},
    sequence::tuple,
    IResult, Parser,
};
use std::default::Default;
use std::fmt::Debug;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Locator {
    pub name: String,
    pub ttype: u32,
    pub elements: Ttype,
    pub position: Vec3f,
    pub triggers: Vec<Trigger>,
}
impl Locator {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Paris for Locator {
    fn paris<'a>(&self) -> Box<dyn Parser<&'a [u8], crate::Chunk, ()>>
    where
        Self: Sized + Par,
    {
        Box::new(move |input| {
            let (input, locator) = Self::par(input).unwrap();
            Ok((input, Chunk::Locator(locator)))
        })
    }
}
impl Par for Locator {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, name_bytes) = length_data(le_u8::<&'a [u8], ()>)(input).unwrap();
        let (input, ttype) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, size) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, elements) = Ttype::paris(ttype, size).parse(input).unwrap();
        let (input, position) = Vec3f::paris(input).unwrap();
        let (input, num_of_triggers) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, triggers) = count(Trigger::par, num_of_triggers as usize)(input).unwrap();

        let name_string = String::from_utf8_lossy(name_bytes);
        let name_stripped = name_string.trim_matches(char::from(0));
        let name = name_stripped.to_string();
        dbg!(&name);

        Ok((
            input,
            Locator {
                name,
                ttype,
                elements,
                position,
                triggers,
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Trigger {
    pub name: String,
    pub type_of: u32,
    pub scale: Vec3f,
    pub matrix: Matrix4f,
}
impl Trigger {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Par for Trigger {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, header) = Header::paris(input).unwrap();
        if header.chunk_id != 0x03_00_00_06 {
            panic!("should be trigger 0x03000006 but is {:x}", header.chunk_id);
        }

        let (input, name_bytes) = length_data(le_u8::<&'a [u8], ()>)(input).unwrap();
        let (input, type_of) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, scale) = Vec3f::paris(input).unwrap();
        let (input, matrix) = Matrix4f::paris(input).unwrap();

        let name_string = String::from_utf8_lossy(name_bytes);
        let name_stripped = name_string.trim_matches(char::from(0));
        let name = name_stripped.to_string();
        dbg!(&name);

        Ok((
            input,
            Trigger {
                name,
                type_of,
                scale,
                matrix,
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Ttype {
    #[default]
    Skip,
    Event(Vec<Vec3f>),    // 0
    Script(),             // 1
    Generic(),            // 2
    CarStart(Vec<Vec3f>), // 3
    Spline(),             // 4
    DynamicZone(String),  // 5
    Occlusion(),          // 6
    InteriorEntrance(),   // 7
    Directional(),        // 8
    Action(),             // 9
    Fov(),                // A
    BreakableCamera(),    // B
    StaticCamera(),       // C
    PedGroup(),           // D
    Coin(),               // E
    SpawnPoint(),         // F
}
impl Ttype {
    pub fn from_id(id: u32) -> Self {
        match id {
            0 => Ttype::Event(vec![]),
            1 => Ttype::Script(),
            2 => Ttype::Generic(),
            3 => Ttype::CarStart(vec![]),
            4 => Ttype::Spline(),
            5 => Ttype::DynamicZone(String::new()),
            6 => Ttype::Occlusion(),
            7 => Ttype::InteriorEntrance(),
            8 => Ttype::Directional(),
            9 => Ttype::Action(),
            10 => Ttype::Fov(),
            11 => Ttype::BreakableCamera(),
            12 => Ttype::StaticCamera(),
            13 => Ttype::PedGroup(),
            14 => Ttype::Coin(),
            15 => Ttype::SpawnPoint(),
            _ => Ttype::Skip,
        }
    }
    fn paris<'a>(ttype: u32, num: u32) -> impl Parser<&'a [u8], Ttype, ()> {
        move |input: &'a [u8]| match ttype {
            5 => {
                let (input, bytes) = take::<u32, &'a [u8], ()>(num * 4)(input).unwrap();
                let string = String::from_utf8_lossy(bytes);
                let stripped = string.trim_matches(char::from(0));

                dbg!(stripped.to_string());
                Ok((input, Ttype::DynamicZone(stripped.to_string())))
            }
            _ => {
                let (input, _trigger) = take::<u32, &'a [u8], ()>(num * 4)(input).unwrap();
                Ok((input, Ttype::Skip))
            }
        }
    }
}
