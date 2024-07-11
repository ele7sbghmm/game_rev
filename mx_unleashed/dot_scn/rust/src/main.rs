#![allow(unused)]
use std::fs::{OpenOptions, create_dir_all};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use nom::{
    IResult,
    character::{is_digit, is_alphabetic},
    bytes::complete::{tag, take_until},
    character::complete::{char, newline},
    sequence::tuple,
};

fn bracket(input: &str) -> IResult<&str, &str> {
    let (input, (_, heading, _, _)) =
        tuple((char('['), take_until("]"), char(']'), newline))(input)?; 
    Ok((input, heading))
}
fn whole(input: &str) -> IResult<&str, &str> {
    take_until("\n[")(input)
}

fn main() {
    let input = 
r#"[SceneInfo]
SceneName=Supercross01
DefaultStartPosition=101.49,90.12,458.48
DefaultStartDirection=0.00,0.00,1.00

[Environment]
ShrubberyDistLimit=0.000000
"#;
    let (input, whole) = whole(input).unwrap(); 

    let (_, scene) = bracket(whole).unwrap();
    // let (input, env) = bracket(input).unwrap();

    println!("{:?}", input);
    println!();
    println!("{:?}", whole);
    println!();
    println!("{:?}", scene);
    // println!("{:?}", env);
}
