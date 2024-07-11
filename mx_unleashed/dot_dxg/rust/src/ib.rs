#![allow(unused)]
use nom::{
    IResult,
    bytes::complete::take,
    multi::count,
    number::complete::{le_u8, le_u16, le_f32},
    sequence::tuple,
};

#[derive(Debug, Clone)]
pub struct Color<N>(pub N, pub N, pub N, pub N);
impl Color<u8> {
    pub fn from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, (r, g, b, a)) =
            tuple((le_u8::<&'a [u8], ()>, le_u8, le_u8, le_u8))(input).unwrap();
        Ok((input, Color (r, g, b, a)))
    }
    pub fn to_float(&self) -> Color<f32> {
        let r = self.0 as f32 / 256.0;
        let g = self.1 as f32 / 256.0;
        let b = self.2 as f32 / 256.0;
        let a = self.3 as f32 / 256.0;
        Color (r, g, b, a)
    }
}
impl Color<f32> {
    pub fn to_obj(&self) -> String {
        format!("{} {} {} {}", self.0, self.1, self.2, self.3)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3f {pub x: f32, pub y: f32, pub z: f32}
impl Vec3f {
    pub fn from_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, (x, y, z)) =
            tuple((le_f32::<&'a [u8], ()>, le_f32, le_f32))(input).unwrap();
        Ok((input, Vec3f {x, y, z}))
    }
    pub fn to_obj(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
pub struct IndexBuffer {inner: Vec<u16>}
impl IndexBuffer {
    pub fn from_bytes<'a>(input: &'a [u8], num: usize) -> IResult<&'a [u8], Self, ()> {
        let (input, inner) =
            count(le_u16, num)(input)?;
        Ok((input, IndexBuffer {inner}))
    }
    pub fn tris2strips(&self) -> Self {
        let mut new: Vec<u16> = vec![];
        for (i, _vertex) in self.inner.iter().enumerate() {
            if i < self.inner.len() - 3 {
                new.push(self.inner[i]);
                new.push(self.inner[i+1]);
                new.push(self.inner[i+2]);

                new.push(self.inner[i+3]);
                new.push(self.inner[i+2]);
                new.push(self.inner[i+1]);
            }
        }
        IndexBuffer {inner: new}
    }
    pub fn to_obj(&self) -> String {
        let mut s: String = String::new();
        for (i, _index) in self.inner.iter().enumerate() {
            if i > self.inner.len() - 2 { return s }
            if i % 3 == 0 {
                s += &format!("f {} {} {}\n", self.inner[i]+1, self.inner[i+1]+1, self.inner[i+2]+1);
            }
        }
        s
    }
    pub fn to_obj_skip(&self) -> String {
        let mut s: String = String::new();
        for (i, _index) in self.inner.iter().enumerate() {
            if i > self.inner.len() - 2 { return s }
            if i % 3 == 0
             && self.inner[i]     != self.inner[i+1]
             && self.inner[i+1]   != self.inner[i+2]
             && self.inner[i+2]   != self.inner[i] {
                s += &format!("f {} {} {}\n", self.inner[i]+1, self.inner[i+1]+1, self.inner[i+2]+1);
            }
        }
        s
    }
    pub fn to_obj_remdups(&self) -> String {
        let mut s: String = String::new();
        let rec: Vec<u16> = f(0, self.inner.clone());
        for (i, _index) in rec.iter().enumerate() {
            if i % 3 == 0 {
                s += &format!("f {} {} {}\n", self.inner[i]+1, self.inner[i+1]+1, self.inner[i+2]+1);
            }
        }
        s
    }
}
fn f<T>(n: usize, mut a: Vec<T>) -> Vec<T>
where T: Clone + Copy + std::cmp::PartialEq {
    if n + 2 > a.len() { return a }

    let (x, y, z) = (a[n], a[n+1], a[n+2]);
    
    if x == y || y == z {
        a.remove(n+1);
        f(n, a.clone())
    } else if x == z {
        a.remove(n+2);
        f(n, a.clone())
    } else {
        f(n+3, a.clone())
    }
}

#[derive(Debug, Clone)]
pub struct VertexBuffer {pub inner: Vec<Vec3f>}
impl VertexBuffer {
    pub fn from_bytes<'a>(input: &'a [u8], num: usize) -> IResult<&'a [u8], Self> {
        let (input, inner) =
            count(Vec3f::from_bytes, num)(input)?;
        Ok((input, VertexBuffer {inner}))
    }
    pub fn to_obj(&self) -> String {
        let mut s: String = String::new();
        for Vec3f {x, y, z} in self.inner.iter() {
            s += &format!("v {} {} {}\n", x, y, z);
        }
        s
    }
    pub fn to_obj_colors(&self, colors: Self) -> String {
        let mut s: String = String::new();
        for Vec3f {x, y, z} in self.inner.iter() {
            s += &format!("v {} {} {}\n", x, y, z);
        };
        assert_eq!(self.inner.len(), colors.inner.len());
        
        s
    }
}
