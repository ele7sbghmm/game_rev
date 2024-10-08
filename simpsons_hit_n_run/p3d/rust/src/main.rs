#![allow(unused)]

use nom::{bytes::complete::take, multi::many0, IResult, Parser};
use std::{
    ffi::OsStr,
    fmt::Debug,
    fs::{read_dir, OpenOptions},
    io::{self, Read, Write},
    path::PathBuf,
};

mod obj2;
use crate::obj2::Obj2;
mod sphere;
mod triggers;
use crate::triggers::{Locator, Trigger};

mod paris;
use paris::Paris;
mod utils;
use utils::{Header, Matrix3f, Vec3f};
mod chunks;
use chunks::{CollisionVec, Cylinder, Fence, Intersect, OBbox, P3d, Skip, Sphere, Wall};
use chunks::{COLLISIONVEC, CYLINDER, FENCE, INTERSECT, LOCATOR, OBBOX, P3D, SPHERE, WALL};

fn chunk_paris<'a>(input: &'a [u8]) -> IResult<&'a [u8], ChunkType, ()> {
    const HEADER_SIZE: u32 = 12;

    let (mut input, header_bytes) = take::<u32, &'a [u8], ()>(HEADER_SIZE)(input)?;
    let (_, header) = Header::paris(header_bytes)?;
    // dbg!(&header);

    let chunkslice_size = header.chunk_size - HEADER_SIZE;
    let dataslice_size = header.data_size - HEADER_SIZE;

    let mut data_slice: &[u8] = &[];
    let mut chunk_slice: &[u8] = &[];

    if let Chunk::Skip(_) = Chunk::id(header.chunk_id) {
        (input, chunk_slice) = take::<u32, &'a [u8], ()>(chunkslice_size)(input).unwrap();
        (chunk_slice, data_slice) = take::<u32, &'a [u8], ()>(dataslice_size)(chunk_slice).unwrap();
    } else if let Chunk::P3d(_) = Chunk::id(header.chunk_id) {
        (input, chunk_slice) = take::<u32, &'a [u8], ()>(chunkslice_size)(input).unwrap();
        (chunk_slice, data_slice) = take::<u32, &'a [u8], ()>(dataslice_size)(chunk_slice).unwrap();
    } else {
        println!(
            "PIKS > {:x} | {:x} | {:x}",
            header.chunk_id.swap_bytes(),
            header.data_size,
            header.chunk_size
        );
        (input, data_slice) = take::<u32, &'a [u8], ()>(chunkslice_size)(input).unwrap();
    }

    let (_remaining_dataslice, chunk) = Chunk::id(header.chunk_id)
        .pariser()
        .parse(data_slice)
        .unwrap();

    let (remaining_chunkslice, sub_chunks) = many0(chunk_paris)(chunk_slice).unwrap();
    assert_eq!(remaining_chunkslice, &[]);

    Ok((
        input,
        ChunkType {
            parent: (chunk, sub_chunks),
        },
    ))
}

#[derive(Debug, PartialEq)]
struct ChunkType {
    // DataSubs: std::mem::ManuallyDrop<(Chunk, Vec<ChunkType>)>,
    parent: (Chunk, Vec<ChunkType>),
}

#[derive(Debug, PartialEq, Clone)]
enum Chunk {
    P3d(P3d),
    Fence(Wall),
    OBbox(OBbox, Vec3f, Matrix3f),
    Sphere(Sphere, Vec3f),
    Cylinder(Cylinder),
    CollisionVec(CollisionVec),
    Intersect(Intersect),
    Locator(Locator),
    Skip(Skip),
}
impl Chunk {
    fn id(id: u32) -> Chunk {
        match id {
            P3D => Chunk::P3d(P3d::new()),
            FENCE => Chunk::Fence(Wall::new()),
            OBBOX => Chunk::OBbox(OBbox::new(), Vec3f::new(), Matrix3f::identity()),
            SPHERE => Chunk::Sphere(Sphere::new(), Vec3f::new()),
            CYLINDER => Chunk::Cylinder(Cylinder::new()),
            COLLISIONVEC => Chunk::CollisionVec(CollisionVec::new()),
            INTERSECT => Chunk::Intersect(Intersect::new()),
            LOCATOR => Chunk::Locator(Locator::new()),
            _ => Chunk::Skip(Skip::new()),
        }
    }
    fn pariser<'a>(&self) -> Box<dyn Parser<&'a [u8], Chunk, ()>> {
        match self {
            Chunk::P3d(_) => P3d::new().paris(),
            Chunk::Fence(_) => Fence::new().paris(),
            Chunk::OBbox(_, _, _) => OBbox::new().paris(),
            Chunk::Sphere(_, _) => Sphere::new().paris(),
            Chunk::Cylinder(_) => Cylinder::new().paris(),
            Chunk::CollisionVec(_) => CollisionVec::new().paris(),
            Chunk::Intersect(_) => Intersect::new().paris(),
            Chunk::Locator(_) => Locator::new().paris(),
            Chunk::Skip(_) => Skip::new().paris(),
        }
    }
}

fn red(path: impl AsRef<std::path::Path>) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    _ = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap()
        .read_to_end(&mut buf);
    buf
}

fn get_chunks(ct: &ChunkType, v: &mut Vec<Chunk>) -> Vec<Chunk> {
    match &ct.parent {
        // (Chunk::Intersect(_), _) => {
        //     v.push(ct.parent.0.clone());
        // }
        // (Chunk::OBbox(_, _, _), _) => {
        //     v.push(ct.parent.0.clone());
        // }
        // (Chunk::Cylinder(_), _) => {
        //     v.push(ct.parent.0.clone());
        // }
        // (Chunk::Sphere(_, _), _) => {
        //     v.push(ct.parent.0.clone());
        // }
        // (Chunk::Fence(_), _) => {
        //     v.push(ct.parent.0.clone());
        // }
        (Chunk::Locator(_), _) => {
            v.push(ct.parent.0.clone());
        }

        (_, sub) => {
            for c in sub.iter() {
                get_chunks(c, v);
            }
        }
    };
    v.to_vec()
}

fn main() -> io::Result<()> {
    // std::env::set_var("RUST_BACKTRACE", "1");

    // let args: Vec<String> = std::env::args().collect();
    // let dir: &String = &args[1];

    let mut path: PathBuf = PathBuf::new();

    let mut obj_int = Obj2::new();
    let mut obj_cyl = Obj2::new();
    let mut obj_sph = Obj2::new();
    let mut obj_obbox = Obj2::new();
    let mut obj_fence = Obj2::new();
    let mut obj_trigger5 = Obj2::new();
    // let mut obj_= Obj2::new();

    let dir: String = "../files/loadzones/".to_string();
    for p in read_dir(&dir).unwrap() {
        path = PathBuf::from(p.unwrap().path());
        dbg!(&path);
        if path.extension().is_none() {
            continue;
        }
        // if path.extension() != Some(&OsStr::from(String::from("p3d"))) {
        //     continue;
        // }

        let buf = red(&path);
        let input = buf.as_slice();
        let (input, c) = chunk_paris(input).unwrap();

        let v = get_chunks(&c, &mut vec![]);

        for cc in v.iter() {
            // if let Chunk::Cylinder(cyl) = cc {
            //     obj_cyl.cylind(cyl);
            // }
            // if let Chunk::Sphere(sph, pos) = cc {
            //     obj_sph.sphere(&sphere_points(pos, sph.radius, 10., 10.));
            // }
            // if let Chunk::Intersect(int) = cc {
            //     obj_int.intersec(int);
            // }
            // if let Chunk::OBbox(obbox, pos, mat) = cc {
            //     obj_obbox.obj_fn_obbox(obbox, pos, mat);
            // }
            // if let Chunk::Fence(wall) = cc {
            //     obj_fence.obj_fn_fence(wall, 5.);
            // }
            if let Chunk::Locator(locator) = cc {
                // dbg!(&locator);
                obj_trigger5.obj_fn_trigger5(locator);
            }
        }

        // println!(" GOOD {:?}", &path);
    }

    // let mut file_int = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l2/l2_int.obj").unwrap(); file_int.write(obj_int.s.as_bytes());
    // let mut file_cyl = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l2/l2_cyl.obj").unwrap();
    // file_cyl.write(obj_cyl.s.as_bytes());
    // let mut file_sph = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l1/l1_sph.obj").unwrap(); file_sph.write(obj_sph.s.as_bytes());
    // let mut file_obbox = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l2/l2_obbox.obj").unwrap(); file_obbox.write(obj_obbox.s.as_bytes());
    // let mut file_fence = OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/l2/l2_fence.obj").unwrap(); file_fence.write(obj_fence.s.as_bytes());
    let mut file_trigger5 = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("/Users/timmeh/Desktop/l7_trigger5.obj")
        .unwrap();
    file_trigger5.write(obj_trigger5.s.as_bytes());

    Ok(())
}
