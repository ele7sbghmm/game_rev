#![allow(unused)]
use std::default::Default;

use crate::utils::{Vec3f, Tri, Matrix3f};
use crate::chunks::{Intersect, Cylinder, OBbox, Wall};
use crate::triggers::{Ttype, Locator};
use crate::sphere::cylinder_points;

pub struct Obj2 {pub i: u32, pub s: String}
impl Obj2 {
    pub fn new() -> Self { Obj2 {i: 1u32, s: String::new()}}
    pub fn add_v(&mut self, v: &Vec3f) -> u32 {
        self.s += &format!("v {} {} {}\n", -v.x, v.y, v.z);
        self.i += 1u32; self.i-1 }
    pub fn add_f(&mut self) -> &mut Self {
        self.s += &format!("f {} {} {}\n", self.i-1, self.i-2, self.i-3); self }
    pub fn add_tri(&mut self, t: &Tri) -> &mut Self {
        self.add_v(&t.p1); self.add_v(&t.p2);
        self.add_v(&t.p3); self.add_f(); self }

    pub fn obj_fn_fence(&mut self, wall: &Wall, height: f32) -> &mut Self {
        let h = Vec3f {x: 0., y: height, z: 0.};
        self.add_v(&wall.start.sub(&h)); self.add_v(&wall.start.add(&h));
        self.add_v(&wall.end.add(&h)); self.add_v(&wall.end.sub(&h));
        self.s += &format!("f {} {} {}\n", self.i-4, self.i-3, self.i-2);
        self.s += &format!("f {} {} {}\n", self.i-4, self.i-2, self.i-1);
        self
    }

    pub fn sphere(&mut self, c: &(Vec3f, Vec<Vec<Vec3f>>, Vec3f)) -> &mut Self {
        self.s += &format!("# SPHERE \n");
        for (i, p) in c.1.iter().enumerate() {
            if i == 0 {
                todo!("origin index");
                self.trifan(&p, true); continue; }
            if i == c.1.len()-1 {
                self.trifan(&p, false); }
            for (j, pp) in p.iter().enumerate() {
                if j == p.len()-1 { continue; }
                self.add_tri(&Tri {p1: c.1[i-1][j], p2: p[j], p3: p[j+1]});
                self.add_tri(&Tri {p1: c.1[i-1][j], p2: p[j+1], p3: c.1[i-1][j+1]});
            }} self }
    pub fn cylind(&mut self, cylinder: &Cylinder) -> &mut Self {
        let tup =
            cylinder_points(cylinder, 10u32);
        // self.trifan(&top, false);
        // self.trifan(&bottom, true);
        
        let mut bottom: Vec<Vec3f> = vec![];
        for (ring_index, ring) in tup.iter().enumerate() {
            if ring_index % 2 != 0 { continue; }
            let top = ring;

            if ring_index == tup.len() - 1 { bottom = tup[0].clone();
            } else { bottom = tup[ring_index + 1].clone(); }

            for (i, v) in ring.iter().enumerate() {
                // if i == 0 { continue; }
                // println!("{} {:.1} {:.1} {:.1}", i, v.x, v.y, v.z);
                if i >= top.len()-1 {
                    self.add_tri(&Tri {p1: top[i], p2: bottom[i], p3: bottom[0]});
                    self.add_tri(&Tri {p1: top[i], p2: bottom[0], p3: top[0]});
                    break;
                }
                self.add_tri(&Tri {p1: top[i], p2: bottom[i], p3: bottom[i+1]});
                self.add_tri(&Tri {p1: top[i], p2: bottom[i+1], p3: top[i+1]});
            }
        } self }

    pub fn trifan(&mut self, array: &Vec<Vec3f>, reverse: bool) -> &mut Self {
        let origin = array[0];
        let oi = self.add_v(&origin);

        let mut array = array.clone();
        if reverse == true {
            array.push(array[0]);
            array.reverse();
            array.pop();
        }
        for (i, v) in array.iter().enumerate() {
            // println!("{} {:?}", i, v);
            if i == 0 { continue; }
            if i == array.len()-1 {
                self.add_v(v); self.add_v(&array[1]);
            } else {
                self.add_v(v); self.add_v(&array[i+1]);
            }
            self.s += &format!("f {} {} {}\n", oi, self.i-1, self.i-2);
            // print!("{}", format!("f {} {} {}\n", oi, self.i-1, self.i-2));
        }; self }
    
    pub fn intersec(&mut self, int: &Intersect) -> &mut Self {
        let k = self.i-1 + 1;
        let ii = int.indices.clone();

        for v in int.positions.iter() {
            self.add_v(v);
        }
        for (j, _) in ii[..ii.len()-2].iter().enumerate() {
            if j % 3 != 0 { continue; }
            let s = &format!("f {} {} {}\n", ii[j+1]+k, ii[j]+k, ii[j+2]+k);
            self.s += s;
        }

        self
    }

    pub fn obj_fn_obbox(&mut self, obbox: &OBbox, position: &Vec3f, matrix: &Matrix3f) -> &mut Self {
        let OBbox {l1: x, l2: y, l3: z} = obbox;
        let i = self.i-1;
        let (i1, i2, i3, i4, i5, i6, i7, i8) =
            (i+1, i+2, i+3, i+4, i+5, i+6, i+7, i+8);

        let p1 = Vec3f {x: *x, y: *y, z: -z};
        let p2 = Vec3f {x: *x, y: *y, z: *z};
        let p3 = Vec3f {x: -x, y: *y, z: *z};
        let p4 = Vec3f {x: -x, y: *y, z: -z};
        let p5 = Vec3f {x: -x, y: -y, z: -z};
        let p6 = Vec3f {x: -x, y: -y, z: *z};
        let p7 = Vec3f {x: *x, y: -y, z: *z};
        let p8 = Vec3f {x: *x, y: -y, z: -z};

        for mut p in [p1, p2, p3, p4, p5, p6, p7, p8].iter() {
            let mut q = matrix.dot(p);
            q = q.add(position);
            self.add_v(&q);
        }
        self.s += &format!("f {} {} {}\n", i1, i2, i3);
        self.s += &format!("f {} {} {}\n", i1, i3, i4);

        self.s += &format!("f {} {} {}\n", i2, i7, i6);
        self.s += &format!("f {} {} {}\n", i2, i6, i3);
        
        self.s += &format!("f {} {} {}\n", i3, i6, i5);
        self.s += &format!("f {} {} {}\n", i3, i5, i4);

        self.s += &format!("f {} {} {}\n", i4, i5, i8);
        self.s += &format!("f {} {} {}\n", i4, i8, i1);

        self.s += &format!("f {} {} {}\n", i5, i6, i7);
        self.s += &format!("f {} {} {}\n", i5, i7, i8);

        self.s += &format!("f {} {} {}\n", i1, i8, i7);
        self.s += &format!("f {} {} {}\n", i1, i7, i2);

        self
    }
    
    pub fn obj_fn_trigger5(&mut self, locator: &Locator) -> &mut Self {
        for trigger in locator.triggers.iter() {
            if let Ttype::DynamicZone(_) = Ttype::from_id(locator.ttype) {
                let m4 = trigger.matrix;
                let m = Vec3f {x: m4.m30, y: m4.m31, z: m4.m32};
                let Vec3f {x, y, z} = trigger.scale;
                let i = self.i-1;
                self.add_v(&m.add(&Vec3f {x: -x, y,     z: -z}));
                self.add_v(&m.add(&Vec3f {x,     y,     z: -z}));
                self.add_v(&m.add(&Vec3f {x,     y,     z    }));
                self.add_v(&m.add(&Vec3f {x: -x, y,     z    }));

                self.add_v(&m.add(&Vec3f {x: -x, y: -y, z    }));
                self.add_v(&m.add(&Vec3f {x,     y: -y, z    }));
                self.add_v(&m.add(&Vec3f {x,     y: -y, z: -z}));
                self.add_v(&m.add(&Vec3f {x: -x, y: -y, z: -z}));

                self.s += &format!("l {} {}\n", i+1, i+2);
                self.s += &format!("l {} {}\n", i+2, i+3);
                self.s += &format!("l {} {}\n", i+3, i+4);
                self.s += &format!("l {} {}\n", i+4, i+1);
                self.s += &format!("l {} {}\n", i+5, i+6);
                self.s += &format!("l {} {}\n", i+6, i+7);
                self.s += &format!("l {} {}\n", i+7, i+8);
                self.s += &format!("l {} {}\n", i+8, i+5);
                self.s += &format!("l {} {}\n", i+1, i+8);
                self.s += &format!("l {} {}\n", i+2, i+7);
                self.s += &format!("l {} {}\n", i+3, i+6);
                self.s += &format!("l {} {}\n", i+4, i+5);
            } else { continue; }
        }
        self
    }
}
