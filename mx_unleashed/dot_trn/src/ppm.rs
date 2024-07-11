use crate::color::{Rgba, RgbaArray};
use crate::utils::write_file;
use std::{
    default::Default,
    fmt::{self, format},
    fs,
    io::Write,
    path::PathBuf,
};

// BLACKANDWHITE	1	1	special case of GRAYSCALE
// GRAYSCALE	2...65535	1	2 bytes per pixel for MAXVAL > 255
// RGB	1...65535	3	6 bytes per pixel for MAXVAL > 255
// BLACKANDWHITE_ALPHA	1	2	2 bytes per pixel
// GRAYSCALE_ALPHA	2...65535	2	4 bytes per pixel for MAXVAL > 255
// RGB_ALPHA	1...65535	4	8 bytes per pixel for MAXVAL > 255

#[derive(Default, Debug, PartialEq, Clone)]
pub enum TuplType {
    BlackAndWhite,
    GrayScale,
    Rgb,
    BlackAndWhiteAlpha,
    GrayScaleAlpha,
    #[default]
    RgbAlpha,
}
impl fmt::Display for TuplType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TuplType::BlackAndWhite => write!(f, "BLACKANDWHITE"),
            TuplType::GrayScale => write!(f, "GRAYSCALE"),
            TuplType::Rgb => write!(f, "RGB"),
            TuplType::BlackAndWhiteAlpha => write!(f, "BLACKANDWHITE_ALPHA"),
            TuplType::GrayScaleAlpha => write!(f, "GRAYSCALE_ALPHA"),
            TuplType::RgbAlpha => write!(f, "RGB_ALPHA"),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub enum BufferType {
    #[default]
    Default,
    Buffer(Vec<u8>),
    Buffers(Vec<Vec<u8>>),
}
impl BufferType {
    pub fn to_bytes(&self) -> Vec<u8> {
        match &self {
            BufferType::Default => panic!(" !!! no buffer to turn to bytes "),
            BufferType::Buffer(buffer) => buffer.clone(),
            BufferType::Buffers(buffers) => {
                let mut out: Vec<u8> = vec![];
                buffers.iter().map(|buf| out.append(&mut buf.clone()));
                out
            }
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Ppm {
    pub ppm_type: TuplType,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub maxval: usize,
    pub tupltype: String,
    pub buffer: BufferType,
}
impl Ppm {
    pub fn new(
        ppm_type: TuplType,
        width: usize,
        height: usize,
        maxval: usize,
        buffer: BufferType,
    ) -> Self {
        Ppm {
            ppm_type,
            width,
            height,
            depth: 4,
            maxval,
            tupltype: String::from("RGB_ALPHA"),
            buffer,
        }
    }
    pub fn get_header(&self) -> String {
        format!(
            "P7\nWIDTH {}\nHEIGHT {}\nDEPTH {}\nMAXVAL {}\nTUPLTYPE {}\nENDHDR\n",
            self.width, self.height, self.depth, self.maxval, self.tupltype,
        )
    }
    pub fn to_bytes(&self, buffer: Option<BufferType>) -> Vec<u8> {
        if let Some(buf) = buffer {
            buf.to_bytes()
        } else {
            let mut header = self.get_header();

            let mut out: Vec<u8> = vec![];
            out.append(&mut header.as_bytes().to_vec());
            out.append(&mut self.buffer.to_bytes());

            out
        }
    }
    pub fn ppm_write(&self, path: PathBuf, buffer: Option<BufferType>) -> () {
        let _ = write_file(path, self.to_bytes(buffer).as_slice());
    }
}
