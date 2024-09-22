use crate::obj2::Obj2;
use crate::paris::{Par, Paris};
use crate::sphere::cylinder_points;
use crate::utils::{Header, Plane, Tri, Vec3f};
use crate::Chunk;
use nom::{
    error::ParseError,
    number::complete::{le_f32, le_u16, le_u32},
    IResult, Parser,
};
use std::fmt::Debug;

pub const P3D: u32 = 0xFF_44_33_50;
pub const FENCE: u32 = 0x03_F0_00_07;
pub const WALL: u32 = 0x03_00_00_00;
pub const OBBOX: u32 = 0x07_01_00_04;
pub const SPHERE: u32 = 0x07_01_00_02;
pub const CYLINDER: u32 = 0x07_01_00_03;
pub const COLLISIONVEC: u32 = 0x07_01_00_07;
pub const INTERSECT: u32 = 0x03_f0_00_03;

pub const LOCATOR: u32 = 0x03_00_00_05;
pub const TRIGGER: u32 = 0x03_00_00_06;

pub fn paris_chunk<'a>(input: &'a [u8]) -> IResult<&'a [u8], Chunk, ()> {
    let (input, header) = Header::paris(input).unwrap();
    let (input, ch) = Chunk::id(header.chunk_id).pariser().parse(input).unwrap();
    Ok((input, ch))
}
pub fn paris_chunk_t<'a, C: Par>(input: &'a [u8]) -> IResult<&'a [u8], C, ()> {
    let (input, header) = Header::paris(input)?;
    C::par(input)
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Skip {}
impl Skip {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct P3d {}
impl P3d {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Fence {}
impl Fence {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Wall {
    pub start: Vec3f,
    pub end: Vec3f,
    pub normal: Vec3f,
}
impl Wall {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OBbox {
    pub l1: f32,
    pub l2: f32,
    pub l3: f32,
}
impl OBbox {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sphere {
    pub radius: f32,
}
impl Sphere {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Cylinder {
    pub position: Vec3f,
    pub axis: Vec3f,
    pub radius: f32,
    pub length: f32,
    pub flat_end: bool,
}
impl Cylinder {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CollisionVec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl CollisionVec {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn to_vec3f(&self) -> Vec3f {
        let CollisionVec { x, y, z } = &self;
        Vec3f {
            x: *x,
            y: *y,
            z: *z,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Intersect {
    pub indices: Vec<u32>,
    pub positions: Vec<Vec3f>,
    pub normals: Vec<Vec3f>,
}
impl Intersect {
    pub fn new() -> Self {
        Self::default()
    }
}
