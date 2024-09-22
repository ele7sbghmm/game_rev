use nom::{
    error::ParseError,
    multi::count,
    number::complete::{le_f32, le_u16, le_u32},
    sequence::tuple,
    IResult, Parser,
};
use std::fmt::Debug;

use crate::{
    chunks::{paris_chunk, paris_chunk_t},
    utils::Matrix3f,
    Chunk, CollisionVec, Cylinder, Fence, Intersect, OBbox, P3d, Skip, Sphere, Vec3f, Wall,
};

pub trait Paris {
    fn paris<'a>(&self) -> Box<dyn Parser<&'a [u8], Chunk, ()>>
    where
        Self: Sized + Par;
}
pub trait Par {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()>
    where
        Self: Sized;
}

impl Paris for Skip {
    fn paris<'a>(&self) -> Box<dyn Parser<&'a [u8], Chunk, ()>> {
        Box::new(move |input| Ok((input, Chunk::Skip(Self {}))))
    }
}
impl Par for Skip {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        Ok((input, Self {}))
    }
}

impl Paris for P3d {
    fn paris<'a>(&self) -> Box<dyn Parser<&'a [u8], Chunk, ()>> {
        Box::new(move |input| {
            let (input, p3d) = Self::par(input).unwrap();
            Ok((input, Chunk::P3d(p3d)))
        })
    }
}
impl Par for P3d {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        Ok((input, Self {}))
    }
}

impl Paris for Fence {
    fn paris<'a>(&self) -> Box<dyn Parser<&'a [u8], Chunk, ()>> {
        Box::new(move |input| {
            let (input, wall) = paris_chunk_t::<Wall>(input).unwrap();
            Ok((input, Chunk::Fence(wall)))
        })
    }
}
impl Par for Fence {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        Ok((input, Self {}))
    }
}

impl Par for Wall {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, start) = Vec3f::paris(input).unwrap();
        let (input, end) = Vec3f::paris(input).unwrap();
        let (input, normal) = Vec3f::paris(input).unwrap();
        Ok((input, Self { start, end, normal }))
    }
}

impl Paris for OBbox {
    fn paris<'a>(&self) -> Box<dyn Parser<&'a [u8], Chunk, ()>> {
        Box::new(move |input| {
            let (input, obbox) = Self::par(input).unwrap();
            let (input, position) = paris_chunk_t::<CollisionVec>(input).unwrap();

            let (input, v1) = paris_chunk_t::<CollisionVec>(input).unwrap();
            let (input, v2) = paris_chunk_t::<CollisionVec>(input).unwrap();
            let (input, v3) = paris_chunk_t::<CollisionVec>(input).unwrap();
            let matrix = Matrix3f::from_3_vec3f(v1.to_vec3f(), v2.to_vec3f(), v3.to_vec3f());

            Ok((input, Chunk::OBbox(obbox, position.to_vec3f(), matrix)))
        })
    }
}
impl Par for OBbox {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, l1) = le_f32::<&'a [u8], ()>(input).unwrap();
        let (input, l2) = le_f32::<&'a [u8], ()>(input).unwrap();
        let (input, l3) = le_f32::<&'a [u8], ()>(input).unwrap();
        Ok((input, Self { l1, l2, l3 }))
    }
}

impl Paris for Sphere {
    fn paris<'a>(&self) -> Box<dyn Parser<&'a [u8], Chunk, ()>> {
        Box::new(move |input| {
            let (input, sphere) = Self::par(input).unwrap();
            let (input, position) = paris_chunk_t::<CollisionVec>(input).unwrap();
            Ok((input, Chunk::Sphere(sphere, position.to_vec3f())))
        })
    }
}
impl Par for Sphere {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, radius) = le_f32::<&'a [u8], ()>(input).unwrap();
        Ok((input, Self { radius }))
    }
}

impl Paris for Cylinder {
    fn paris<'a>(&self) -> Box<dyn Parser<&'a [u8], Chunk, ()>> {
        Box::new(move |input| {
            let (input, radius) = le_f32::<&'a [u8], ()>(input).unwrap();
            let (input, length) = le_f32::<&'a [u8], ()>(input).unwrap();
            let (input, flat_end_u16) = le_u16::<&'a [u8], ()>(input).unwrap();
            let (input, position) = paris_chunk_t::<CollisionVec>(input).unwrap();
            let (input, axis) = paris_chunk_t::<CollisionVec>(input).unwrap();

            let flat_end: bool = if flat_end_u16 == 1 { true } else { false };
            let cylinder = Cylinder {
                position: position.to_vec3f(),
                axis: axis.to_vec3f(),
                radius,
                length,
                flat_end,
            };
            Ok((input, Chunk::Cylinder(cylinder)))
        })
    }
}
impl Par for Cylinder {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        Ok((input, Self::new()))
    }
}

impl Paris for CollisionVec {
    fn paris<'a>(&self) -> Box<dyn Parser<&'a [u8], Chunk, ()>> {
        Box::new(move |input| {
            let (input, collision_vec) = Self::par(input).unwrap();
            Ok((input, Chunk::CollisionVec(collision_vec)))
        })
    }
}
impl Par for CollisionVec {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, x) = le_f32::<&'a [u8], ()>(input).unwrap();
        let (input, y) = le_f32::<&'a [u8], ()>(input).unwrap();
        let (input, z) = le_f32::<&'a [u8], ()>(input).unwrap();
        Ok((input, Self { x, y, z }))
    }
}

impl Paris for Intersect {
    fn paris<'a>(&self) -> Box<dyn Parser<&'a [u8], Chunk, ()>> {
        Box::new(move |input| {
            let (input, intersect) = Self::par(input).unwrap();
            Ok((input, Chunk::Intersect(intersect)))
        })
    }
}
impl Par for Intersect {
    fn par<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, num_of_indices) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, indices) =
            count(le_u32::<&'a [u8], ()>, num_of_indices as usize)(input).unwrap();

        let (input, num_of_positions) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, positions) = count(Vec3f::paris, num_of_positions as usize)(input).unwrap();

        let (input, num_of_normals) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, normals) = count(Vec3f::paris, num_of_normals as usize)(input).unwrap();

        Ok((
            input,
            Self {
                indices,
                positions,
                normals,
            },
        ))
    }
}
