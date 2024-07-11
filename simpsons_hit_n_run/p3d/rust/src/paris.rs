#![allow(unused)]

use std::fmt::Debug;
use nom::{
    IResult,
    Parser,
    error::ParseError,
    multi::count,
    number::complete::{le_f32, le_u32, le_u16},
    sequence::tuple,
};

use crate::{
    Chunk, Vec3f,
    Skip, P3d, Fence, Wall, OBbox, Sphere, Cylinder, CollisionVec, Intersect, 
    chunks::{paris_chunk, paris_chunk_t},
    utils::Matrix3f,
};
use crate::chunks::Def;

pub trait Paris {
    fn paris<'a, E>(&self) -> Box<dyn Parser<&'a [u8], Chunk, E>>
    where
        Self: Sized + Par,
        E: ParseError<&'a [u8]> + Debug;
    }
pub trait Par {
    fn par<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Self, E>
    where Self: Sized,
          E: ParseError<&'a [u8]> + Debug;
}

impl Paris for Skip {
    fn paris<'a, E>(&self) -> Box<dyn Parser<&'a [u8], Chunk, E>>
    where E: ParseError<&'a [u8]> + Debug {
        Box::new(move |input| {
            Ok((input, Chunk::Skip(Self {})))
})}}
impl Par for Skip {
    fn par<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], Self, E> {
        Ok((input, Self {}))
}}

impl Paris for P3d {
    fn paris<'a, E>(&self) -> Box<dyn Parser<&'a [u8], Chunk, E>>
    where E: ParseError<&'a [u8]> + Debug {
        Box::new(move |input| {
            let (input, p3d) =
                Self::par::<E>(input).unwrap();
            Ok((input, Chunk::P3d(p3d)))
})}}
impl Par for P3d {
    fn par<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], Self, E> {
        Ok((input, Self {}))
}}

impl Paris for Fence {
    fn paris<'a, E>(&self) -> Box<dyn Parser<&'a [u8], Chunk, E>>
    where E: ParseError<&'a [u8]> + Debug {
        Box::new(move |input| {
            let (input, wall) =
                // Wall::par(input)?;
                paris_chunk_t::<E, Wall>(input).unwrap();
            Ok((input, Chunk::Fence(wall)))
})}}
impl Par for Fence {
    fn par<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], Self, E> {

        Ok((input, Self {}))
    }
}

impl Par for Wall {
    fn par<'a, E: ParseError<&'a [u8]> + Debug>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], Self, E> {
        let (input, (start, end, normal)) =
            tuple((Vec3f::paris, Vec3f::paris, Vec3f::paris))(input)?;
        Ok((input, Self {start, end, normal}))
}}

impl Paris for OBbox {
    fn paris<'a, E>(&self) -> Box<dyn Parser<&'a [u8], Chunk, E>>
    where E: ParseError<&'a [u8]> + Debug {
        Box::new(move |input| {
            let (input, obbox) =
                Self::par::<E>(input).unwrap();
            let (input, position) =
                paris_chunk_t::<E, CollisionVec>(input).unwrap();

            let (input, v1) =
                paris_chunk_t::<E, CollisionVec>(input).unwrap();
            let (input, v2) =
                paris_chunk_t::<E, CollisionVec>(input).unwrap();
            let (input, v3) =
                paris_chunk_t::<E, CollisionVec>(input).unwrap();
            let matrix =
                Matrix3f::from_3_vec3f(v1.to_vec3f(), v2.to_vec3f(), v3.to_vec3f());
            Ok((input, Chunk::OBbox(obbox, position.to_vec3f(), matrix)))
})}}
impl Par for OBbox {
    fn par<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Self, E>
    where E: ParseError<&'a [u8]> + Debug {
        let (input, (l1, l2, l3)) =
            tuple((le_f32::<&'a [u8], E>, le_f32, le_f32))(input).unwrap();
        Ok((input, Self {l1, l2, l3}))
}}

impl Paris for Sphere {
    fn paris<'a, E>(&self) -> Box<dyn Parser<&'a [u8], Chunk, E>>
    where E: ParseError<&'a [u8]> + Debug {
        Box::new(move |input| {
            let (input, sphere) =
                Self::par::<E>(input).unwrap();
            let (input, position) =
                paris_chunk_t::<E, CollisionVec>(input).unwrap();
            Ok((input, Chunk::Sphere(sphere, position.to_vec3f())))
})}}
impl Par for Sphere {
    fn par<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Self, E>
    where E: ParseError<&'a [u8]> + Debug {
        let (input, radius) =
            le_f32::<&'a [u8], E>(input).unwrap();
        Ok((input, Self {radius}))
}}

impl Paris for Cylinder {
    fn paris<'a, E>(&self) -> Box<dyn Parser<&'a [u8], Chunk, E>>
    where E: ParseError<&'a [u8]> + Debug {
        Box::new(move |input| {
            let (input, (radius, length, flat_end_u16)) =
                tuple((le_f32::<&'a [u8], E>, le_f32, le_u16))(input).unwrap();
            let (input, position) =
                paris_chunk_t::<E, CollisionVec>(input).unwrap();
            let (input, axis) =
                paris_chunk_t::<E, CollisionVec>(input).unwrap();
            let flat_end: bool = if flat_end_u16 == 0 { false } else { true };
            let cylinder = Cylinder {position: position.to_vec3f(), axis: axis.to_vec3f(), radius, length, flat_end};
            Ok((input, Chunk::Cylinder(cylinder)))
})}}
impl Par for Cylinder {
    fn par<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Self, E>
    where E: ParseError<&'a [u8]> + Debug {
        Ok((input, Self::new()))
}}

// impl Paris for CollisionVec {
//     fn paris<'a, E>(&self) -> Box<dyn Parser<&'a [u8], Chunk, E>>
//     where E: ParseError<&'a [u8]> + Debug {
//         Box::new(move |input| {
//             let (input, collision_vec) =
//                 Self::par::<E>(input).unwrap();
//             Ok((input, Chunk::CollisionVec(collision_vec)))
// })}}
impl Par for CollisionVec {
    fn par<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Self, E>
    where E: ParseError<&'a [u8]> + Debug {
        let (input, (x, y, z)) =
            tuple((le_f32::<&'a [u8], E>, le_f32, le_f32))(input).unwrap();
        Ok((input, Self {x, y, z}))
}}

impl Paris for Intersect {
    fn paris<'a, E>(&self) -> Box<dyn Parser<&'a [u8], Chunk, E>>
    where E: ParseError<&'a [u8]> + Debug {
        Box::new(move |input| {
            let (input, intersect) =
                Self::par::<E>(input).unwrap();
            Ok((input, Chunk::Intersect(intersect)))
        })
    }
}
impl Par for Intersect {
    fn par<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Self, E>
    where E: ParseError<&'a [u8]> + Debug {
        let (input, num_of_indices) =
            le_u32::<&'a [u8], E>(input).unwrap();
        let (input, indices) =
            count::<&'a [u8], u32, E,
                    fn(&'a [u8]) -> IResult<&'a [u8], u32, E>>(
                le_u32, num_of_indices as usize
            )(input).unwrap();

        let (input, num_of_positions) =
            le_u32::<&'a [u8], E>(input).unwrap();
        let (input, positions) =
            count::<&'a [u8], Vec3f, E,
                    fn(&'a [u8]) -> IResult<&'a [u8], Vec3f, E>
                >(Vec3f::paris, num_of_positions as usize
            )(input).unwrap();

        let (input, num_of_normals) =
            le_u32::<&'a [u8], E>(input).unwrap();
        let (input, normals) =
            count::<&'a [u8], Vec3f, E,
                    fn(&'a [u8]) -> IResult<&'a [u8], Vec3f, E>>(
                Vec3f::paris, num_of_normals as usize
            )(input).unwrap();

        Ok((input, Self {indices, positions, normals}))
    }
}
