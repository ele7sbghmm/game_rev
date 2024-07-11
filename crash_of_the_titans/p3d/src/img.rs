#![allow(unused)]
use std::fmt::Debug;
use std::io::{self, prelude::*, Read, Write};
use std::rc::Rc;
use nom::{
    IResult, Parser,
    bytes::complete::{take},
    error::ParseError,
    multi::length_data,
    number::complete::{le_f32, le_u8, le_u16, le_u32},
    sequence::tuple,
};

pub const P3D: u32 =  0x503344FF;
pub const TGA1: u32 = 0x05900100;
pub const TGA2: u32 = 0x01900100;
pub const TGA3: u32 = 0x02900100;

// trait Paris<'a, E: ParseError<&'a [u8]> + Debug> {
//     fn paris(&self) -> &dyn Parser<&'a [u8], Chunks<'a>, E>;
// }
trait Paris<'a, E: ParseError<&'a [u8]> + Debug> {
    fn paris(&self) -> Box<dyn Parser<&'a [u8], Chunks<'a>, E>>;
}

// #[derive(Debug)]
// struct Tga2<'a> {
//     file_name: &'a [u8],
//     seven_strange_u32s: (u32, u32, u32, u32, u32, u32, u32),
// }
// impl<'a> Tga2<'a> {
//     fn paris<E: ParseError<&'a [u8]> + Debug>(&self) -> impl Parser<&'a [u8], Tga2<'a>, E> {
//         Box::new(move |input: &'a [u8]| {
//             let (input, file_name_size) = le_u8::<&'a [u8], E>(input).unwrap();
//             let (input, file_name) = take::<u8, &'a [u8], E>(file_name_size)(input).unwrap();
//             let (input, seven_strange_u32s) =
//                 tuple((le_u32::<&'a [u8], E>, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32))
//                 (input).unwrap();
// 
//             Ok((input, Tga2 {file_name, seven_strange_u32s}))
//         })
//     }
// }

#[derive(Debug, Default)]
struct Tga3<'a> { png: PNG<'a>, }
impl<'a> Tga3<'a> { fn new() -> Self { Default::default() }}
impl<'a, E> Paris<'a, E> for Tga3<'a>
where
    E: ParseError<&'a [u8]> + Debug
{
    // fn paris(&self) -> &dyn Parser<&'a [u8], Chunks<'a>, E> {
    fn paris(&self) -> Box<dyn Parser<&'a [u8], Chunks<'a>, E>> {
        // &move |input: &'a [u8]| {
        Box::new(move |input: &'a [u8]| {
            let (input, png_size) =
                le_u32::<&'a [u8], E>(input).unwrap();
            let (input, png) =
                paris_png::<E>(png_size).parse(input).unwrap();

            Ok((input, Chunks::Tga3(Tga3 {png})))
        })
    }
}
type PNG<'a> = &'a [u8];
fn paris_png<'a, E: ParseError<&'a [u8]> + Debug>(
    size: u32
) -> impl Parser<&'a [u8], PNG<'a>, E> {
    move |input: &'a [u8]| {
        take::<u32, &'a [u8], E>(size)(input)
    }
}

enum Chunks<'a> {
    // P3d,
    // Tga1,
    // Tga2,
    Tga3(Tga3<'a>),
    // Png,
}
impl<'a> Chunks<'a> {
    fn get_parser<E>(
        id: u32
    ) -> Rc<Box<dyn Parser<&'a [u8], Chunks<'a>, E>>>
    where
        E: ParseError<&'a [u8]> + Debug
   {
        match id {
            TGA3 => Rc::new(Tga3::new().paris()),
            _ => { panic!(" unknown id ") }
        }
    }
}
