use std::fmt::Debug;
use nom::{
    IResult,
    Parser,
    error::ParseError,
    number::complete::{le_f32, le_u32, le_u16},
    sequence::tuple,
};
use crate::Chunk;
use crate::paris::{Paris, Par};
use crate::utils::{Vec3f, Tri, Plane, Header};
use crate::obj2::Obj2;
use crate::sphere::cylinder_points;

pub const P3D: u32 =          0xFF_44_33_50;
pub const FENCE: u32 =        0x03_F0_00_07;
pub const WALL: u32 =         0x03_00_00_00;
pub const OBBOX: u32 =        0x07_01_00_04;
pub const SPHERE: u32 =       0x07_01_00_02;
pub const CYLINDER: u32 =     0x07_01_00_03;
pub const COLLISIONVEC: u32 = 0x07_01_00_07;
pub const INTERSECT: u32 =    0x03_f0_00_03;

pub const LOCATOR: u32 =      0x03_00_00_05;
pub const TRIGGER: u32 =      0x03_00_00_06;


pub fn paris_chunk<'a, E: ParseError<&'a [u8]> + Debug>(
    input: &'a [u8]
) -> IResult<&'a [u8], Chunk, E> {
    let (input, header) = Header::paris::<E>(input).unwrap();
    let (input, ch) =
        Chunk::id(header.chunk_id)
            .pariser::<E>()
            .parse(input)
            .unwrap();
    Ok((input, ch))
}
pub fn paris_chunk_t<'a, E, C: Par>(
    input: &'a [u8]
) -> IResult<&'a [u8], C, E>
where E: ParseError<&'a [u8]> + Debug {
    let (input, header) =
        Header::paris::<E>(input)?;
    C::par::<E>(input)
}

pub trait Def where Self: Default {
    fn new() -> Self { Default::default() }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Skip {}
impl Def for Skip {}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct P3d {}
impl Def for P3d {}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Fence {}
impl Def for Fence {}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Wall {
    pub start: Vec3f,
    pub end: Vec3f,
    pub normal: Vec3f
}
impl Def for Wall {}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct OBbox {pub l1: f32, pub l2: f32, pub l3: f32}
impl Def for OBbox {}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub radius: f32
}
impl Def for Sphere {}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cylinder {
    pub position: Vec3f,
    pub axis: Vec3f,
    pub radius: f32,
    pub length: f32,
    pub flat_end: bool
}
impl Def for Cylinder {}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct CollisionVec {pub x: f32, pub y: f32, pub z: f32}
impl Def for CollisionVec {}
impl CollisionVec {
    pub fn to_vec3f(&self) -> Vec3f {
        let CollisionVec {x, y, z} = &self;
        Vec3f {x: *x, y: *y, z: *z}
}}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Intersect {
    pub indices: Vec<u32>,
    pub positions: Vec<Vec3f>,
    pub normals: Vec<Vec3f>
}
impl Def for Intersect {}
