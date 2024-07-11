#![allow(unused)]

use crate::chunks::OBbox;

fn obbox(c: OBbox, i: u32) -> (u32, String) {
    if let Chunk::OBbox(c, p) = cc {
        c.to_cube()
            .inside_out()
            .mat_dot2(m)
            .add_offset(&p)
            .to_obj_i(i)
    }

    if let Chunk::Sphere(s, p) = cc {
        let sph = hmm(p, s.radius, 10., 10.);
        obj.sphere(&sph);
    } else { () }
}
