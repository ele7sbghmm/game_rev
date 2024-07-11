use nom::{
    bytes::complete::take,
    multi::count,
    number::complete::{le_f32, le_u16, le_u32, le_u8},
    sequence::tuple,
    IResult, Parser,
};

use crate::{
    color::RgbaArray,
    trn::{Release, Track, Trn},
    utils::{FromBytes, TrackType, Vec3f, Vec4f},
};

#[derive(Default, Debug)]
pub struct Vertex {
    pub vec4: Vec4f,
    pub sometimes_null: u16,
    pub id: u16,
    // pub feffs: Vec<u32>,
    pub feffs: RgbaArray,
    pub _00_1: u32,
    pub _27: u8,
    pub _trailing_null: Option<u32>,
}
impl Vertex {
    pub fn from_bytes<'a>(offset: usize, trail: bool) -> impl Parser<&'a [u8], Vertex, ()> {
        move |clean_input: &'a [u8]| {
            let input = &clean_input[offset..];

            let (input, vec4) = Vec4f::from_bytes(input).unwrap();
            let (input, sometimes_null) = le_u16::<&'a [u8], ()>(input).unwrap();
            let (input, id) = le_u16::<&'a [u8], ()>(input).unwrap();
            // let (input, feffs) = count(le_u32::<&'a [u8], ()>, 17)(input).unwrap();
            let (input, feffs) = RgbaArray::from_bytes_rgba(17).parse(input).unwrap();
            let (input, _00_1) = le_u32::<&'a [u8], ()>(input).unwrap();
            let (mut input, _27) = le_u8::<&'a [u8], ()>(input).unwrap();

            let mut trailing_null: u32 = 0;
            let _trailing_null = match trail {
                true => {
                    (input, trailing_null) = le_u32::<&'a [u8], ()>(input).unwrap();
                    Some(trailing_null)
                }
                false => None,
            };

            Ok((
                input,
                Vertex {
                    vec4,
                    sometimes_null,
                    id,
                    feffs,
                    _00_1,
                    _27,
                    _trailing_null,
                },
            ))
        }
    }
}

#[derive(Default, Debug)]
pub struct VertexArray {
    pub leading_vertex: Vertex,
    pub indices_offset: usize,
    pub indices: Vec<usize>,
    pub vertices: Vec<Vertex>,
}
impl VertexArray {
    pub fn parse(offset: usize, clean_input: &[u8]) -> VertexArray {
        // let input = &clean_input[offset..];

        let (input, leading_vertex) = Vertex::from_bytes(offset, false)
            .parse(clean_input)
            .unwrap();
        let (input, indices_offset) = le_u32::<&[u8], ()>(input).unwrap();

        let input = &clean_input[indices_offset as usize..];
        let (_, indices) = count(le_u32::<&[u8], ()>, 0x100)(input).unwrap();

        let mut vertices: Vec<Vertex> = vec![];
        for index_offset in &indices {
            let (_, vertex) = Vertex::from_bytes(*index_offset as usize, true)
                .parse(input)
                .unwrap();
            vertices.push(vertex);
        }
        VertexArray {
            leading_vertex,
            indices_offset: indices_offset as usize,
            indices: indices.iter().map(|index| *index as usize).collect(),
            vertices,
        }
    }
}

#[derive(Default, Debug)]
pub struct VertexSectionFree {
    pub vertex_section_header: VertexSectionHeader,
    pub vertex_arrays: Vec<VertexArray>,
}
impl VertexSectionFree {
    pub fn parse(release: Release, track: Track, offset: usize, clean_input: &[u8]) -> Self {
        let vertex_section_header = VertexSectionHeader::parse(offset, &clean_input);

        let mut vertex_arrays: Vec<VertexArray> = vec![];
        for offset in vertex_section_header.vertex_array_offsets.iter() {
            vertex_arrays.push(VertexArray::parse(*offset, clean_input));
        }

        VertexSectionFree {
            vertex_section_header,
            vertex_arrays,
        }
    }
}

#[derive(Default, Debug)]
pub struct VertexSectionSx {
    vertex_section: VertexArray,
}
impl VertexSectionSx {
    pub fn parse<'a>(offset: usize, clean_input: &'a [u8]) -> Self {
        let vertex_section = VertexArray::parse(offset, clean_input);

        VertexSectionSx { vertex_section }
    }
}

#[derive(Default, Debug)]
pub struct VertexSectionHeader {
    pub leading_vertex: Vertex,
    pub vertex_array_offsets_offset: usize,
    pub vertex_array_offsets: Vec<usize>,
}
impl VertexSectionHeader {
    pub fn parse(offset: usize, clean_input: &[u8]) -> Self {
        let input = &clean_input[offset..];

        let (input, leading_vertex) = Vertex::from_bytes(offset, false).parse(input).unwrap();
        let (input, vertex_array_offsets_offset) = le_u32::<&[u8], ()>(input).unwrap();
        let vertex_array_offsets_offset = vertex_array_offsets_offset as usize;

        let input = &clean_input[vertex_array_offsets_offset..];
        let (input, offsets) = count(le_u32::<&[u8], ()>, 0x100)(input).unwrap();

        println!("{:08x}", vertex_array_offsets_offset);
        println!("{:?}", offsets);

        let mut vertex_array_offsets: Vec<usize> = vec![];
        for vertex_offset in offsets {
            let vertex_offset = vertex_offset as usize;
            if vertex_offset != 0usize {
                vertex_array_offsets.push(vertex_offset);
            }
        }

        VertexSectionHeader {
            leading_vertex,
            vertex_array_offsets_offset,
            vertex_array_offsets,
        }
    }
}
