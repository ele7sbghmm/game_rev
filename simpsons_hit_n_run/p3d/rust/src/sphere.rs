#![allow(unused)]
use std::default::Default;
use std::f32::consts::{PI, TAU};
use crate::utils::{Vec3f, Matrix3f};
use crate::chunks::{Def, Cylinder};

pub type RingStack = Vec<Ring>;
pub type Ring = Vec<Vec3f>;
pub type Centre = Vec3f;

pub type SpherePoints = (Centre, RingStack, Centre);
pub type HalfSpherePoints = (Centre, RingStack);
pub type CylinderPoints = (Ring, Ring);
pub type CapsulePoints = (HalfSpherePoints, HalfSpherePoints);

fn s2c(r: f32, t: f32, p: f32) -> Vec3f {
    let (ts, tc) = t.sin_cos();
    let (ps, pc) = p.sin_cos();
    // Vec3f {x: r*ps*tc, y: r*ps*ts, z: r*pc}
    Vec3f {x: r*ps*tc, y: r*pc, z: r*ps*ts}
}

pub fn sphere_points(offset: &Vec3f, r: f32, width: f32, height: f32) -> SpherePoints {
    let start = Vec3f {x: 0., y: r, z: 0.};
    let end = Vec3f {x: 0., y: -r, z: 0.};
    let mut h: f32 = 0.; let mut w: f32 = 0.;
    let mut v: Vec<Vec<Vec3f>> = vec![];

    loop {
        w = 0.;
        if h == 0. { h += PI/height; continue; }
        if h >= PI-0.00001 { break; }
        let mut vv: Vec<Vec3f> = vec![];
        loop {
            // if w == 0. { w += PI/width; continue; }
            if w >= TAU { break; }
            let xyz = s2c(r, w, h);
            vv.push(xyz.add(offset));
            w += PI/width;
        }
        if vv != vec![] {
            vv.push(vv[0]);
            v.push(vv);
        }
        h += PI/height;
    }
    (start.add(offset), v, end.add(offset))
}

pub fn cylinder_points(
    cylinder: &Cylinder, wires: u32
) -> Vec<Vec<Vec3f>> {
    let Cylinder {position: p, axis: a, radius: r, length: h, flat_end: fl} = cylinder;

    let mag = a.scale(*h).magnitude();
    let half_mag = mag/2.;
    let axis = a.scale(half_mag);
    let naxis = a.scale(-half_mag);
    
    let mut wire = 0f32;
    let mut vt = Vec3f::new(); let mut vb = Vec3f::new();
    let mut top: Vec<Vec3f> = vec![];
    let mut bot: Vec<Vec3f> = vec![];
    let (mut c, mut s) = (0f32, 0f32);

    // let mut theta = 0f32;
    let asin = axis.x.asin();
    let mut theta = asin;
    println!("t {:?}", theta);
    println!("x {:?}", axis);
    let ax = if axis.x > 1. { 1f32; } else if axis.x < 1f32 { 1f32; } else { 1f32; };
    let ay = if axis.y > 1. { 1f32; } else if axis.y < 1f32 { 1f32; } else { 1f32; };
    let az = if axis.z > 1. { 1f32; } else if axis.z < 1f32 { 1f32; } else { 1f32; };
    loop {
        if theta >= asin + TAU-0.001 { break; }
        c = theta.cos();
        s = theta.sin();

        vt = Vec3f {x: r*c, z: r*s, y: 0.};
        vb = Vec3f {x: r*c, z: r*s, y: 0.};

        theta += TAU/wires as f32;

        vt = vt.add(&a.scale(theta));
        vb = vb.add(&a.scale(theta));
        vt = vt.add(&axis);
        vb = vb.add(&naxis);
        vt = vt.add(&p);
        vb = vb.add(&p);
        
        top.push(vt);
        bot.push(vb);
    }
    
    vec![top, bot]
}

pub fn printvec(h: &str, v: &Vec3f) -> () {
    println!("{} {:.4} {:.4} {:.4}", h, v.x, v.y, v.z);
}
