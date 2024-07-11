use crate::utils::{Vec3f, Vec4f};

type Vert = String;
type Face = String;

#[derive(Default, Debug, PartialEq)]
pub enum Mode {
    Points,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
    #[default]
    None,
}

pub trait ToObj {
    fn to_obj(&self) -> String {
        String::new()
    }
}

#[derive(Default, Debug)]
pub struct Obj {
    vert_count: usize,
    verts: Vert,
    polys: Face,
}
impl Obj {
    pub fn new() -> Self {
        Obj::default()
    }
    pub fn draw(&self) -> String {
        format!("\n{}\n{}\n", self.verts, self.polys)
    }
    pub fn add_vert<V: ToObj>(&mut self, vert: &V) {
        self.vert_count += 1;
        self.verts += &vert.to_obj();
    }
    pub fn add_face<V: ToObj>(&mut self, a: V, b: V, c: V) {
        self.add_vert(&a);
        self.add_vert(&b);
        self.add_vert(&c);

        self.polys += &format!(
            "f {} {} {}\n",
            self.vert_count,
            self.vert_count - 1,
            self.vert_count - 2,
        );
    }
    pub fn mode(self, mode: Mode) -> Self {
        let mut s = String::new();

        match mode {
            Mode::Points => Obj {
                vert_count: self.vert_count,
                verts: self.verts,
                polys: s,
            },
            Mode::Lines => {
                for i in 1..=self.vert_count {
                    if i % 2 != 0 {
                        s += &format!("l {} {}\n", i, i + 1,)
                    } else {
                        continue;
                    }
                }
                Obj {
                    vert_count: self.vert_count,
                    verts: self.verts,
                    polys: s,
                }
            }
            Mode::LineLoop => {
                for i in 1..=self.vert_count >>>>>>>>>>>>>>>>>. as usize {
                    s += &format!("l {} {}\n", i, i + 1,)
                }
                Obj {
                    vert_count: self.vert_count,
                    verts: self.verts,
                    polys: s,
                }
            }
            Mode::LineStrip => {
                for i in 1..=self.vert_count as usize {
                    s += &format!("l {} {}\n", 1, i + 1,)
                }
                Obj {
                    vert_count: self.vert_count,
                    verts: self.verts,
                    polys: s,
                }
            }
            Mode::Triangles => {
                for i in 3..=self.vert_count as usize {
                    if i % 3 == 0 {
                        s += &format!("f {} {} {}\n", i - 2, i - 1, i,)
                    }
                }
                Obj {
                    vert_count: self.vert_count,
                    verts: self.verts,
                    polys: s,
                }
            }
            Mode::TriangleStrip => {
                for i in 1..=self.vert_count as usize {
                    if i % 2 == 0 {
                        s += &format!("f {} {} {}\n", i + 2, i + 1, i,)
                    } else if i % 2 != 0 {
                        s += &format!("f {} {} {}\n", i, i + 1, i + 2,)
                    }
                }
                Obj {
                    vert_count: self.vert_count,
                    verts: self.verts,
                    polys: s,
                }
            }
            Mode::TriangleFan => {
                for i in 1..=self.vert_count as usize {
                    s += &format!("f {} {} {}\n", 1, i + 1, i + 2,)
                }
                Obj {
                    vert_count: self.vert_count,
                    verts: self.verts,
                    polys: s,
                }
            }
            Mode::None => self,
        }

}

impl ToObj for Vec3f {
    fn to_obj(&self) -> String {
        format!("v {:.2} {:.2} {:.2}\n", self.x, self.y, self.z)
    }
}
impl ToObj for Vec4f {
    fn to_obj(&self) -> String {
        format!("v {:.2} {:.2} {:.2}\n", self.x, self.y, self.z)
    }
}
