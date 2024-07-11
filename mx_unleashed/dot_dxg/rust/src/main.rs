#![allow(unused)]
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use nom::{
    IResult,
    bytes::complete::take,
    multi::count,
    number::complete::{le_f32, le_u16, le_u32},
    sequence::{tuple},
};

mod ib;
use ib::{Color, IndexBuffer, VertexBuffer, Vec3f};

type Mat44f = (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32);
// type Color = [u8; 4];

struct Header {
    // fourcc: &'a [u8],
    // _some_count: u32,
    // _some_one: u32,
    // file_name: &'a [u8],
    // _mat44f: Mat44f,
    // _null: u32,
    // _vert_0: Vec3f,
    // _vert_1: Vec3f,
    // _neg_one: u32,
    // block_names_count: u32,
    block_names: Vec<String>,
    // _one: u32,
    // _neg_one: u16,
    // _sevenf: u16,
    // seven: u32,
}
impl Header {
    fn from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], Header, ()> {
        let (input, fourcc) = take::<usize, &'a [u8], ()>(4)(input).unwrap();
        let (input, some_count) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, some_one) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, file_name) = take::<usize, &'a [u8], ()>(0x80)(input).unwrap();
        let (input, mat44f_identiy) = tuple((le_f32::<&'a [u8], ()>, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32))(input).unwrap();
        let (input, null0) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, (vert1, vert2)) = tuple((Vec3f::from_bytes, Vec3f::from_bytes))(input).unwrap();
        let (input, _ffffffff) = le_u32::<&'a [u8], ()>(input).unwrap();

        let (input, block_names_count) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, block_names_slices) = count(
            take::<usize, &'a [u8], ()>(0x40), block_names_count as usize
        )(input)?;
        let (input, one_u32) = le_u32::<&'a [u8], ()>(input).unwrap();
        let (input, ffff_u16) = le_u16::<&'a [u8], ()>(input).unwrap();
        let (input, sevenf_u16) = le_u16::<&'a [u8], ()>(input).unwrap();
        let (input, seven) = le_u32::<&'a [u8], ()>(input).unwrap();

        // let (input, _null) = le_u32::<&'a [u8], ()>(input).unwrap();
        // let (input, fifteen) = le_u32::<&'a [u8], ()>(input).unwrap();
        // let (input, one_u32_2) = le_u32::<&'a [u8], ()>(input).unwrap();
        // let (input, null_u16) = le_u16::<&'a [u8], ()>(input).unwrap();
        // let (input, float) = le_f32::<&'a [u8], ()>(input).unwrap();

        let block_names: Vec<String> = block_names_slices.iter().map(
            |&v| String::from_utf8_lossy(v)
                .trim_end_matches("\x00")
                .to_string()
        ).collect();

        Ok((input, Header {block_names}))
    }
}

#[derive(Debug, Clone)]
struct Block {
    file_name: String,
    pos: u32,
    index_buffer: IndexBuffer,
    vertex_buffer: VertexBuffer,
    vertex_normals: VertexBuffer,
    vertex_colors: Vec<Color<u8>>,
    float_pairs: Vec<(f32, f32)>,
}
impl Block {
    fn from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], Block, ()> {
        let (input, (pos, _fifteen, one, null_16, float)) =
            tuple((le_u32::<&'a [u8], ()>, le_u32, le_u32, le_u16, le_f32))(input).unwrap();
        let (input, (vb_count, ib_count)) = tuple((le_u16::<&'a [u8], ()>, le_u16))(input).unwrap();

        let (input, index_buffer) = IndexBuffer::from_bytes(input, ib_count as usize).unwrap();
        let (input, vertex_buffer) = VertexBuffer::from_bytes(input, vb_count as usize).unwrap();
        let (input, vertex_normals) = VertexBuffer::from_bytes(input, vb_count as usize).unwrap();
        let (input, vertex_colors) = count(Color::from_bytes, vb_count as usize)(input)?;
        let (input, float_pairs) = count(tuple((le_f32, le_f32)), vb_count as usize)(input)?;

        Ok((input, Block { file_name: String::new(), pos, index_buffer, vertex_buffer, vertex_normals, vertex_colors, float_pairs}))
    }
    fn to_obj_colors(&self) -> String {
        assert_eq!(self.vertex_buffer.inner.len(), self.vertex_colors.len());

        let mut s: String = String::new();
        
        for i in 0..self.vertex_buffer.inner.len() {
            s += &format!(
                "v {} {}\n",
                self.vertex_buffer.inner[i].to_obj(),
                self.vertex_colors[i].to_float().to_obj(),
            );
        };
        s
    }
}

fn from_bytes<'a>(input: &'a [u8]) -> Vec<Block> {
    let mut v: Vec<Block> = vec![];
    let (input, header) = Header::from_bytes(input).unwrap();
    let (input, blocks) = count(Block::from_bytes, header.block_names.len())(input).unwrap();
    for (i, s) in header.block_names.iter().enumerate() {
        let mut b = blocks[i].clone();
        b.file_name = s.to_string();

        v.push(b);
    }
    v
}

fn lets_read<'a, P>(path: P) -> Result<Vec<u8>, ()>
where P: AsRef<std::path::Path> {
    let mut buf: Vec<u8> = vec![];
    _ = File::open(path).unwrap().read_to_end(&mut buf);
    Ok(buf)
}

fn main() -> io::Result<()> {
    // let args: Vec<String> = std::env::args().collect();
    // let in_path = PathBuf::from(&args[1]);
    let mut out_path = PathBuf::from("/private/tmp/dxg/all/free10_eco/");

    // let input = lets_read(&in_path).unwrap();

    // let blocks = from_bytes(input.as_slice());
    // for bl in blocks.iter() {
    //     let mut tmp_out_path = out_path.clone();
    //     tmp_out_path.push(in_path.file_name().unwrap().to_str().unwrap().to_string() + &".obj");
    //     // tmp_out_path.push(PathBuf::from(bl.file_name.to_string() + &".obj"));
    //     println!("{:?}", tmp_out_path);
    //     _ = std::fs::create_dir_all(tmp_out_path.with_file_name(""));
    //     let mut file: File =
    //         OpenOptions::new().create(true).write(true).open(tmp_out_path).unwrap();
    //     file.write(bl.to_obj_colors().as_bytes());
    //     file.write(b"");
    //     file.write(bl.index_buffer.tris2strips().to_obj().as_bytes());
    //     println!("{}", bl.file_name);
    // }

    Ok(())
}
