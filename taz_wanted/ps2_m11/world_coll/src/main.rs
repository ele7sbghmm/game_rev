use std::io;
use nom::{
    number::complete::le_f32,
    sequence::tuple, IResult
};

struct Vec3f {x: f32, y: f32, z: f32}
impl Vec3f {
    fn parse<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec3f, ()> {
        let (input, (x, y, z)) =
            tuple((le_f32::<&'a [u8], ()>, le_f32, le_f32))(input).unwrap();
        Ok((input, Vec3f {x, y, z}))
    }
}
struct BigArrayHeader {
    fourcc: u32,
    weird: [u8; 0x50],
}
struct BigVertex {
    pos1: Vec3f, normal1: Vec3f,
    pos2: Vec3f, normal2: Vec3f,
    pos3: Vec3f, normal3: Vec3f,
}

fn main() -> io::Result<()> {

    Ok(())
}
