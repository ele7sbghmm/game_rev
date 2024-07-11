#![allow(unused)]
use nom::{
    self, IResult, Parser,
    number::complete::{le_f32, le_u16, le_u32},
    error::ParseError,
    multi::{many0, count},
    sequence::{tuple},
};
use std::io::{self, Read, Write};

#[derive(Debug)]
struct Vec3f {x: f32, y: f32, z: f32}
impl Vec3f {
    fn paris<'a, E: ParseError<&'a [u8]>>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], Vec3f, E> {
        let (input, (x, y, z)) =
            tuple((le_f32, le_f32, le_f32))(input)?;
        Ok((input, Vec3f{x, y, z}))
    }
    fn to_obj(self) -> String {
        format!("v {:?} {:?} {:?}\n", self.x, self.y, self.z)
    }
}

// #[derive(Debug)]
// struct _36 {vec3f: Vec3f, unk32: u32, h1: u16, h2: u16, h3: u16}
// impl _36 {
//     fn paris<'a, E: ParseError<&'a [u8]>>( input: &'a [u8]) -> IResult<&'a [u8], _36, E> {
//         let (input, (vec3f_1, vec3f_2, vec3f_3, h1, h2)) =
//             tuple((Vec3f::paris, Vec3f::paris, Vec3f::paris, le_u16, le_u16))(input)?;
//         Ok((input, _36 {vec3f_1, unk32, h1, h2, h3}))
//     }
// }

#[derive(Debug)]
struct _49 {vec_of_floats: Vec<f32>, h1: u16, h2: u16}
impl _49 {
    fn paris<'a, E: ParseError<&'a [u8]>>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], _49, E> {
        let (input, vec_of_floats) =
            count(le_f32, 7)(input)?;
        let (input, (h1, h2)) =
            tuple((le_u16, le_u16))(input)?;
        Ok((input, _49 {vec_of_floats, h1, h2}))
    }
    fn to_obj_135(self) -> String {
        format!(
            "v {:?} {:?} {:?}\n",
            self.vec_of_floats[0],
            self.vec_of_floats[3],
            self.vec_of_floats[6],
        )
    }
}

#[derive(Debug)]
struct _50 {vec3f: Vec3f, unk32: u32, h1: u16, h2: u16, h3: u16}
impl _50 {
    fn paris<'a, E: ParseError<&'a [u8]>>(
        input: &'a [u8]
    ) -> IResult<&'a [u8], _50, E> {
        let (input, (vec3f, unk32, h1, h2, h3)) =
            tuple((Vec3f::paris, le_u32, le_u16, le_u16, le_u16))(input)?;
        Ok((input, _50 {vec3f, unk32, h1, h2, h3}))
}}

fn main() -> io::Result<()> {

    for index in 1..62 {
        let path =
            format!(
                "/Users/mair/_kode/_smrt/game_rev/mx_unleashed/file_parsing/dot_col/rr_col_XX/all_free06_rr_col/col__free06_rr_col_{:#02}.col",
                index
            );
        
        let mut v: Vec<u8> = vec![];
        _ = std::fs::File::open(path).unwrap().read_to_end(&mut v);
        let input = v.as_slice();

        let (input, _) = count(le_u32::<&[u8], ()>, 18)(input).unwrap();
        let (input, (size_1, size_2, size_3, unk_1)) =
            tuple((le_u32::<&[u8], ()>, le_u32, le_u32, le_u32))(input).unwrap();
        
        let (input, vec_1) =
            count(Vec3f::paris::<()>, size_1 as usize)(input).unwrap(); 
        let (input, unk_u32) =
            le_u32::<&[u8], ()>.parse(input).unwrap();
        let (input, vec_2) =
            count(Vec3f::paris::<()>, size_1 as usize)(input).unwrap(); 
        let (input, vec_3) =
            count(_49::paris::<()>, size_2 as usize)(input).unwrap(); 
        // let (input, vec_4) =
        //     count(_50::paris::<()>, size_3 as usize)(input).unwrap(); 

        // println!("{:?}", vec_11);
        // println!("{:?}", input);

        let mut obj = std::fs::OpenOptions::new()
            .append(true)
            .open::<&str>(
            "/Users/mair/_kode/_smrt/game_rev/mx_unleashed/file_parsing/dot_col/rr_col_XX/"
        ).unwrap();

        for vec3f in vec_3 {
            obj.write(vec3f.to_obj_135().as_ref());
        }
    }

    Ok(())
}
