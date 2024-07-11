#![allow(unused)]
use std::{
    fs::File,
    io::{self, Read, Write},
    path::{PathBuf, Path},
};
use nom::{ self, IResult,
    bytes::complete::{take},
    error::{ParseError, VerboseErrorKind},
    Err::Error,
    multi::{count, length_data},
    number::complete::{le_u16, le_f32, le_u32},
    sequence::{tuple},
};

#[derive(Debug)]
struct Vec3f {x: f32, y: f32, z: f32}
impl Vec3f {
    fn parse<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec3f, ()> {
        let (input, (x, y, z)) = tuple((le_f32::<&'a [u8], ()>, le_f32, le_f32))(input).unwrap();
        Ok((input, Vec3f {x, y, z}))
    }
    fn to_obj(self) -> String {
        format!("v {:#.6} {:#.6} {:#.6}\n", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct IB {inner: Vec<u16>}
impl IB {
    fn parse_len<'a>(input: &'a [u8], len: u16) -> IResult<&'a [u8], IB, ()> {
        dbg!(len);
        let (input, inner) = count(le_u16::<&'a [u8], ()>, len as usize)(input).unwrap();
        Ok((input, IB {inner}))
    }
    fn strips_to_tris(self) -> IB {
        let mut out: Vec<u16> = vec![];
        for (index, _) in self.inner[..&self.inner.len()-2].iter().enumerate() {
            if index % 2 == 0 { out.push(self.inner[index]); out.push(self.inner[index+1]); out.push(self.inner[index+2]);
            } else {            out.push(self.inner[index]); out.push(self.inner[index+2]); out.push(self.inner[index+1]); }
        }
        IB {inner: out}
    }
    fn vec_to_obj(self) -> String {
        let mut s: String = String::new();
        for (index, _) in self.inner.iter().enumerate().step_by(3) {
            s += &format!("f {} {} {}\n", self.inner[index]+1, self.inner[index+1]+1, self.inner[index+2]+1);
        }
        s
    }
}

#[derive(Debug)]
struct VB {inner: Vec<Vec3f>}
impl VB {
    fn parse_len<'a>(input: &'a [u8], len: u16) -> IResult<&'a [u8], VB, ()> {
        let (input, inner) = count(Vec3f::parse, len as usize)(input).unwrap();
        Ok((input, VB {inner}))
    }
    fn to_obj(self) -> String {
        let mut s: String = String::new();
        for v in self.inner { s += &v.to_obj(); }
        s
    }
}

type HeaderGarbage<'a> = (&'a [u8], u32, u32, &'a [u8], Vec<f32>, (f32, f32, f32, f32, f32, f32), u32, u32, &'a [u8], (u32, u16, u16, u32, u32, u32, u32, u32, u32, u16));
fn parse_header_garbage<'a>(
    input: &'a [u8]
) -> IResult<&'a [u8], HeaderGarbage, ()> {
    let (input, second
    ) = tuple((
        take::<u32, &'a [u8], ()>(4_u32),
        le_u32, le_u32,
        take(0x80_u32),
        count(le_f32, 16_usize),
        tuple((le_f32, le_f32, le_f32, le_f32, le_f32, le_f32)),
        le_u32, le_u32, take(0x40_u8),
        tuple((le_u32, le_u16, le_u16, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32, le_u16)),
    ))(input).unwrap();

    Ok((input, second))
}
fn parse_dxg<'a>(input: &'a [u8]) -> IResult<&'a [u8], (IB, VB), ()> {
    let (input, header_garbage) =
        parse_header_garbage(input).unwrap();
    let (input, (vb_len, ib_len)) =
        tuple((le_u16::<&'a [u8], ()>, le_u16))(input).unwrap();
    let (input, ib) = IB::parse_len(input, ib_len).unwrap();
    let (input, vb) = VB::parse_len(input, vb_len).unwrap();

    Ok((input, (ib, vb)))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    let file_name: &Path = Path::new(&args[1]);
    let mut path: PathBuf = PathBuf::from("/Users/mair/_kode/_reversing/game_files/mx/xbox_ex2/");
    path.push(file_name);
    dbg!(path.clone());
    let mut buf: Vec<u8> = vec![];
    _ = File::open(path).unwrap().read_to_end(&mut buf);
    let input: &[u8] = buf.as_slice();

    let (input, (ib, vb)) = parse_dxg(input).unwrap();

    let mut out_path: PathBuf = PathBuf::from("/Users/mair/_kode/_smrt/game_rev/mx_unleashed/dot_dxg/rust/all_dxgs/");
    out_path.push(file_name);

    let mut out: File = File::create(out_path)?;
    out.write(vb.to_obj().as_bytes());
    out.write(b"\n");
    out.write(ib.strips_to_tris().vec_to_obj().as_bytes());

    Ok(())
}
