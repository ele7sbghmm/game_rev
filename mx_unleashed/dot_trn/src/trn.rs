use crate::{
    color::RgbaArray, texture::Texture, thing::_7c31Array, utils::Header, vertex::VertexArray,
};

#[derive(Default, Debug)]
pub enum Release {
    #[default]
    Proto,
    // Ps2,
    Xbox,
}
#[derive(Default, Debug)]
pub enum Track {
    #[default]
    Free,
    Nat,
    Sx,
}

#[derive(Default, Debug)]
pub struct Trn {
    pub release: Release,
    pub track: Track,
    pub header: Header,
    // pub _7c31s: _7c31Array,
    pub textures: Vec<Texture>,
    // pub vertex_section: VertexSectionSx,
}
// impl Trn {
//     pub fn parse(release: Release, track: Track, clean_input: &[u8]) -> Self {
//         let (input, header) = Header::parse(release, track, clean_input);

//         todo!();
//     }
// }
