#![allow(unused)]
use std::fmt::Debug;
use std::fs::{OpenOptions, read_dir};
use std::io::{self, prelude::*, Read};
use std::path::PathBuf;
use nom::{
    IResult,
    Parser,
    bytes::complete::take,
    combinator::opt,
    error::ParseError,
    multi::many0,
    number::complete::{le_u32, le_f32},
    sequence::tuple,
};

mod obj2; mod sphere; 
use crate::obj2::*; use sphere::*;
mod triggers;
use crate::triggers::Locator;

mod paris;
use paris::Paris;
mod utils;
use utils::{Header, Vec3f, Matrix3f};
mod chunks;
use chunks::{
    Def,
    P3d, Fence, Wall, OBbox, Sphere, Cylinder, CollisionVec, Intersect, Skip,
    P3D, FENCE, WALL, OBBOX, SPHERE, CYLINDER, COLLISIONVEC, INTERSECT, LOCATOR,
};


fn chunk_paris<'a, E: ParseError<&'a [u8]> + Debug> (
    input: &'a [u8]
) -> IResult<&'a [u8], ChunkType, E> {
    const HEADER_SIZE: u32 = 12;

    let (mut input, header_bytes) =
        take::<u32, &'a [u8], E>(HEADER_SIZE)(input)?;
    let (_, header) = Header::paris(header_bytes)?;

    let chunkslice_size = header.chunk_size-HEADER_SIZE;
    let dataslice_size = header.data_size-HEADER_SIZE;

    let mut data_slice: &[u8] = &[];
    let mut chunk_slice: &[u8] = &[];

    if let Chunk::Skip(_) = Chunk::id(header.chunk_id) { (input, chunk_slice) =
            take::<u32, &'a [u8], E>(chunkslice_size)(input)?;
        (chunk_slice, data_slice) = take::<u32, &'a [u8], E>(dataslice_size)(chunk_slice).unwrap();
    } else {
        // println!("id {:x}\nds {:x}\ncs {:x}\n", header.chunk_id, header.data_size, header.chunk_size);
        
        (input, data_slice) = take::<u32, &'a [u8], E>(chunkslice_size)(input).unwrap();
    }

    let (_remaining_dataslice, chunk) =
        Chunk::id(header.chunk_id).pariser::<E>().parse(data_slice).unwrap();

    let (_remaining_chunkslice, sub_chunks) =
        many0(chunk_paris::<E>)(chunk_slice).unwrap();
    assert_eq!(_remaining_chunkslice, &[]);

    Ok((input, ChunkType {parent: (chunk, sub_chunks)}))
}

#[derive(Debug, PartialEq)]
struct ChunkType {
    // DataSubs: std::mem::ManuallyDrop<(Chunk, Vec<ChunkType>)>,
    parent: (Chunk, Vec<ChunkType>),
    // mother: (Chunk, Chunk),
}

#[derive(Debug, PartialEq, Clone)]
enum Chunk {
    P3d(P3d),
    Fence(Wall),
    OBbox(OBbox, Vec3f, Matrix3f),
    Sphere(Sphere, Vec3f),
    Cylinder(Cylinder),
    // CollisionVec(CollisionVec),
    Intersect(Intersect),

    Locator(Locator),
    // Trigger(Trigger),

    Skip(Skip)
}
impl Chunk {
    fn id(id: u32) -> Chunk {
        match id {
            // P3D =>          Chunk::P3d(P3d::new()),
            // FENCE =>        Chunk::Fence(Wall::new()),
            // WALL =>         Chunk::Wall(Wall::new()),
            // OBBOX =>        Chunk::OBbox(OBbox::new(), Vec3f::new(), Matrix3f::identity()),
            // SPHERE =>       Chunk::Sphere(Sphere::new(), Vec3f::new()),
            CYLINDER =>     Chunk::Cylinder(Cylinder::new()),
            // COLLISIONVEC => Chunk::CollisionVec(CollisionVec::new()),
            // INTERSECT =>    Chunk::Intersect(Intersect::new()),

            // LOCATOR => Chunk::Locator(Locator::new()),
            // TRIGGER => Chunk::Trigger(Trigger::new()),

            _ =>            Chunk::Skip(Skip::new()),
        }
    }
    fn pariser<'a, E: ParseError<&'a [u8]> + Debug>(
        &self
    ) -> Box<dyn Parser<&'a [u8], Chunk, E>> {
        match &self {
            Chunk::P3d(_) =>          P3d::new().paris(),
            Chunk::Fence(_) =>     Fence::new().paris(),
            Chunk::OBbox(_, _, _) =>  OBbox::new().paris(),
            Chunk::Sphere(_, _) =>    Sphere::new().paris(),
            Chunk::Cylinder(_) =>  Cylinder::new().paris(),
            // Chunk::CollisionVec(_) => CollisionVec::new().paris(),
            Chunk::Intersect(_) =>    Intersect::new().paris(),
            Chunk::Skip(_) =>         Skip::new().paris(),

            Chunk::Locator(_) =>      Locator::new().paris(),
        }
    }
}

fn red<'a>(path: impl AsRef<std::path::Path>) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![]; 
    _ = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap()
        .read_to_end(&mut buf);
    buf
}

fn get_chunks(ct: &ChunkType, mut v: &mut Vec<Chunk>) -> Vec<Chunk> {
    match &ct.parent { 
        (Chunk::Intersect(_), _)   => { v.push(ct.parent.0.clone()); }
        (Chunk::OBbox(_, _, _), _) => { v.push(ct.parent.0.clone()); }
        (Chunk::Cylinder(_), _)    => { v.push(ct.parent.0.clone()); }
        (Chunk::Sphere(_, _), _)   => { v.push(ct.parent.0.clone()); }
 
        (Chunk::Fence(_), _)       => { v.push(ct.parent.0.clone()); }
        (Chunk::Locator(_), _)     => { v.push(ct.parent.0.clone()); }

        (_, sub) => {
            for c in sub.iter() {
                get_chunks(c, v);
            };
        }
    };
    v.to_vec()
}

fn main() -> io::Result<()> {
    // std::env::set_var("RUST_BACKTRACE", "1");

    // let args: Vec<String> = std::env::args().collect();
    // let dir: &String = &args[1];
    let dir: String = "/tmp/l2/".to_string();
    let mut path: PathBuf = PathBuf::from("".to_string());

    let mut obj_int = Obj2::new();
    let mut obj_cyl= Obj2::new();
    let mut obj_sph= Obj2::new();
    let mut obj_obbox= Obj2::new();
    let mut obj_fence= Obj2::new();
    let mut obj_trigger5= Obj2::new();
    // let mut obj_= Obj2::new();

    for p in read_dir(&dir)? {
        path = PathBuf::from(p?.path());
        if path.extension().is_none() { continue; }
        if path.extension().unwrap().to_str().unwrap() != "p3d" { continue; }

        let buf = red(&path);
        let input = buf.as_slice();
        let (input, c) = chunk_paris::<()>(input).unwrap();

        let v = get_chunks(&c, &mut vec![]);
        for cc in v.iter() {
            if let Chunk::Cylinder(cyl) = cc { obj_cyl.cylind(cyl); }
            if let Chunk::Sphere(sph, pos) = cc { obj_sph.sphere(&sphere_points(pos, sph.radius, 10., 10.)); }
            if let Chunk::Intersect(int) = cc { obj_int.intersec(int); }
            if let Chunk::OBbox(obbox, pos, mat) = cc { obj_obbox.obj_fn_obbox(obbox, pos, mat); }
            if let Chunk::Fence(wall) = cc { obj_fence.obj_fn_fence(wall, 5.); }
            if let Chunk::Locator(locator) = cc { obj_trigger5.obj_fn_trigger5(locator); }
        }

        // println!(" GOOD {:?}", &path);
    }

    // let mut file_int = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l2/l2_int.obj").unwrap(); file_int.write(obj_int.s.as_bytes());
    let mut file_cyl = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l2/l2_cyl.obj").unwrap(); file_cyl.write(obj_cyl.s.as_bytes());
    // let mut file_sph = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l1/l1_sph.obj").unwrap(); file_sph.write(obj_sph.s.as_bytes());
    // let mut file_obbox = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l2/l2_obbox.obj").unwrap(); file_obbox.write(obj_obbox.s.as_bytes());
    // let mut file_fence = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l2/l2_fence.obj").unwrap(); file_fence.write(obj_fence.s.as_bytes());
    // let mut file_trigger5 = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l2/l2_trigger5.obj").unwrap(); file_trigger5.write(obj_trigger5.s.as_bytes());

    Ok(())
}
