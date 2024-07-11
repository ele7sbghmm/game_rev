use std::{
    default::Default,
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

use crate::col::{X20, Vb, Vc, Ib};
use crate::utils::{Vec3f, RGBA};

#[derive(Debug, Default)]
pub struct Obj {
    v: u32,
    s: String,
}
impl Obj {
    pub fn new() -> Obj { Default::default() }
    pub fn obj_write(&self, out_path: PathBuf) -> Result<usize, io::Error> {
        let mut file = OpenOptions::new()
            .create(true).write(true).truncate(true).open(out_path).unwrap();
        file.write(self.s.as_bytes())
    }
    pub fn comment<T: std::fmt::Display>(&mut self, comment: T) -> &mut Self {
        self.s += &format!("# {comment}\n"); 
        self
    }
    pub fn add_v(&mut self, vert: &Vec3f) -> u32 {
        self.s += &format!("v {} {} {}\n", vert.x, vert.y, vert.z);
        self.v += 1;
        self.v
    }
    pub fn add_vc(&mut self, vert: &Vec3f, color: &RGBA<u8>) -> u32 {
        let color_one = color.float().to_one();
        self.s += &format!(
            "v {} {} {}  {} {} {} {}\n",
            vert.x, vert.y, vert.z, color_one.r,
            color_one.g, color_one.b, color_one.a,
        );
        self.v += 1;
        self.v
    }
    pub fn add_face_doublesided(
        &mut self, vert1: &Vec3f, vert2: &Vec3f, vert3: &Vec3f, vert4: &Vec3f,
    ) -> &mut Self {
        let (v1, v2, v3, v4): (u32, u32, u32, u32) = (
            self.add_v(vert1), self.add_v(vert2),
            self.add_v(vert3), self.add_v(vert4),
        ); 
        self.s += &format!("f {} {} {}\n", v1, v2, v3);
        self.s += &format!("f {} {} {}\n", v1, v3, v4);
        self
    }
    pub fn add_face(
        &mut self, vert1: &Vec3f, vert2: &Vec3f, vert3: &Vec3f
    ) -> &mut Self {
        let (v1, v2, v3): (u32, u32, u32) = (
            self.add_v(vert1), self.add_v(vert2), self.add_v(vert3)
        ); 
        self.s += &format!("f {} {} {}\n", v1, v2, v3);
        self
    }

    pub fn obj_vb(&mut self, vb: &Vb) -> &mut Self {
        for vert in vb.inner.iter() {
            self.add_v(vert);
        }
        self
    }
    pub fn obj_vb_vc(&mut self, vb: &Vb, vc: &Vc) -> &mut Self {
        for (i, _) in vb.inner.iter().enumerate() {
            self.add_vc(&vb.inner[i], &vc.inner[i]);
        }
        self
    }
    pub fn obj_ib(&mut self, ib: &Ib, offset: u32) -> &mut Self {
        for (i, _) in ib.inner[..ib.inner.len()-2].iter().enumerate() {
            // if i+2 >= ib.inner.len() { break; }
            if i % 3 != 0 { continue; }
            self.s += &format!("f {} {} {}\n",
                ib.inner[i]+1 + offset,
                ib.inner[i+1]+1 + offset,
                ib.inner[i+2]+1 + offset, 
            );
        }
        self
    }

    pub fn x20(&mut self, x20: &X20) -> &mut Self {
        let len = self.v;

        if let Some(vb) = &x20.vb {
            if let Some(ib) = &x20.ib {

                if let Some(vc) = &x20.vc{
                    self.comment(&x20.name);

                    // self.obj_vb(&vb);
                    self.obj_vb_vc(&vb, &vc.rbg2bgr());

                    self.obj_ib(&ib, len);
                } else {}

            } else {}
        } else {}

        self
    }
}