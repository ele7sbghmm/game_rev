use nom::{
    error::ParseError,
    multi::count,
    number::complete::{le_f32, le_u32},
    sequence::tuple,
    IResult,
};
use std::f32::consts::{PI, TAU};
use std::{default::Default, fmt::Debug};
// use ndarray::{arr2, arr3};

#[derive(Debug)]
pub struct Header {
    pub chunk_id: u32,
    pub data_size: u32,
    pub chunk_size: u32,
}
impl Header {
    pub fn paris<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, (chunk_id, data_size, chunk_size)) =
            tuple((le_u32::<&'a [u8], ()>, le_u32, le_u32))(input).unwrap();
        Ok((
            input,
            Header {
                chunk_id,
                data_size,
                chunk_size,
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3f {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn paris<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, (x, y, z)) = tuple((le_f32, le_f32, le_f32))(input)?;
        Ok((input, Vec3f { x, y, z }))
    }
    pub fn dot(&self, mat: &Matrix3f) -> Self {
        mat.dot(self)
    }
    pub fn cross(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
        let x = v1.y * v2.z - v1.z * v2.y;
        let y = v1.z * v2.x - v1.x * v2.z;
        let z = v1.x * v2.y - v1.y * v2.x;
        Vec3f { x, y, z }
    }
    pub fn swap(&self) -> Self {
        let Vec3f { x, y, z } = &self;
        Vec3f {
            x: *y,
            y: *x,
            z: *z,
        }
    }
    pub fn mirror(&self, field: &str) -> Self {
        match field.to_lowercase().as_str() {
            "x" => Vec3f {
                x: -self.x,
                y: self.y,
                z: self.z,
            },
            "xy" => Vec3f {
                x: -self.x,
                y: -self.y,
                z: self.z,
            },
            "xz" => Vec3f {
                x: -self.x,
                y: self.y,
                z: -self.z,
            },
            "y" => Vec3f {
                x: self.x,
                y: -self.y,
                z: self.z,
            },
            "yz" => Vec3f {
                x: self.x,
                y: -self.y,
                z: -self.z,
            },
            "z" => Vec3f {
                x: self.x,
                y: self.y,
                z: -self.z,
            },
            "xyz" | _ => Vec3f {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            },
        }
    }
    pub fn scale_add(vect: &Vec3f, factor: f32, me: &Vec3f) -> Vec3f {
        let x = vect.x + me.x * factor;
        let y = vect.y + me.y * factor;
        let z = vect.z + me.z * factor;
        Vec3f { x, y, z }
    }
    pub fn product(&self, other: &Vec3f) -> Vec3f {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;
        Vec3f { x, y, z }
    }
    pub fn recip(&self) -> Vec3f {
        Vec3f {
            x: 1. / self.x,
            y: 1. / self.y,
            z: 1. / self.z,
        }
    }
    pub fn square(&self) -> Vec3f {
        Vec3f {
            x: self.x.powf(2.),
            y: self.y.powf(2.),
            z: self.z.powf(2.),
        }
    }
    pub fn scale(&self, float: f32) -> Vec3f {
        Vec3f {
            x: self.x * float,
            y: self.y * float,
            z: self.z * float,
        }
    }
    pub fn add_f32(&self, float: f32) -> Vec3f {
        Vec3f {
            x: self.x + float,
            y: self.y + float,
            z: self.z + float,
        }
    }
    pub fn add(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    pub fn sub(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    pub fn magnitude(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()
    }
    pub fn normalize(&self) -> Vec3f {
        let mag = self.magnitude();
        Vec3f {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Tri {
    pub p1: Vec3f,
    pub p2: Vec3f,
    pub p3: Vec3f,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Plane {
    pub p1: Vec3f,
    pub p2: Vec3f,
    pub p3: Vec3f,
    pub p4: Vec3f,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Matrix3f {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
}
impl Matrix3f {
    pub fn identity() -> Matrix3f {
        Matrix3f {
            m00: 1.,
            m01: 0.,
            m02: 0.,
            m10: 0.,
            m11: 1.,
            m12: 0.,
            m20: 0.,
            m21: 0.,
            m22: 1.,
        }
    }
    pub fn from_3_vec3f(v1: Vec3f, v2: Vec3f, v3: Vec3f) -> Matrix3f {
        let Vec3f {
            x: m00,
            y: m01,
            z: m02,
        } = v1;
        let Vec3f {
            x: m10,
            y: m11,
            z: m12,
        } = v2;
        let Vec3f {
            x: m20,
            y: m21,
            z: m22,
        } = v3;

        Matrix3f {
            m00,
            m01,
            m02,
            m10,
            m11,
            m12,
            m20,
            m21,
            m22,
        }
    }
    pub fn dot(&self, v: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.m00 * v.x + self.m10 * v.y + self.m20 * v.z,
            y: self.m01 * v.x + self.m11 * v.y + self.m21 * v.z,
            z: self.m02 * v.x + self.m12 * v.y + self.m22 * v.z,
        }
    }
    pub fn rot_x(a: &f32) -> Self {
        Matrix3f {
            m00: 1.,
            m10: 0.,
            m20: 0.,
            m01: 0.,
            m11: a.cos(),
            m21: -a.sin(),
            m02: 0.,
            m12: a.sin(),
            m22: a.cos(),
        }
    }
    pub fn rot_y(a: &f32) -> Self {
        Matrix3f {
            m00: a.cos(),
            m10: 0.,
            m20: a.sin(),
            m01: 0.,
            m11: 1.,
            m21: 0.,
            m02: -a.sin(),
            m12: 0.,
            m22: a.cos(),
        }
    }
    pub fn rot_z(a: &f32) -> Self {
        Matrix3f {
            m00: a.cos(),
            m10: -a.sin(),
            m20: 0.,
            m01: a.sin(),
            m11: a.cos(),
            m21: 0.,
            m02: 0.,
            m12: 0.,
            m22: 1.,
        }
    }
    // pub fn rot_z_v(v: &Vec3f) -> Self {
    // Matrix3f {
    //     m00: v.cos(), m10: -a.sin(), m20: 0.,
    //     m01: a.sin(), m11:  a.cos(), m21: 0.,
    //     m02:      0., m12:       0., m22: 1.
    // }
    // }
    pub fn euler2mat(euler: &Vec3f) -> Matrix3f {
        let Vec3f {
            x: heading,
            y: attitude,
            z: bank,
        } = euler;
        let ht = -heading;
        let bt = -attitude;
        let at = bank;
        // println!("{:.4} {:.4} {:.4}", ht, at, bt);

        let ch = ht.cos();
        let sh = ht.sin();
        let ca = at.cos();
        let sa = at.sin();
        let cb = bt.cos();
        let sb = bt.sin();

        Matrix3f {
            m00: ch * ca,
            m01: sh * sb - ch * sa * cb,
            m02: ch * sa * sb + sh * cb,
            m10: sa,
            m11: ca * cb,
            m12: -ca * sb,
            m20: -sh * ca,
            m21: sh * sa * cb + ch * sb,
            m22: -sh * sa * sb + ch * cb,
        }

        // Matrix3f {
        //     m00:  ch * ca,
        //     m01:  sh * sb       -  ch * sa * cb,
        //     m02:  ch * sa * sb  +  sh * cb,
        //     m10:  sa,
        //     m11:  ca * cb,
        //     m12: -ca * sb,
        //     m20: -sh * ca,
        //     m21:  sh * sa * cb  +  ch * sb,
        //     m22: -sh * sa * sb  +  ch * cb,
        // }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Matrix4f {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
}
impl Matrix4f {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn paris<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self, ()> {
        let (input, v) = count(le_f32::<&'a [u8], ()>, 16usize)(input).unwrap();
        Ok((input, Matrix4f::from_vec(v)))
    }
    pub fn from_vec(v: Vec<f32>) -> Self {
        Matrix4f {
            m00: v[0],
            m01: v[1],
            m02: v[2],
            m03: v[3],
            m10: v[4],
            m11: v[5],
            m12: v[6],
            m13: v[7],
            m20: v[8],
            m21: v[9],
            m22: v[10],
            m23: v[11],
            m30: v[12],
            m31: v[13],
            m32: v[14],
            m33: v[15],
        }
    }
    pub fn to_vec(&self) -> Vec<f32> {
        vec![
            self.m00, self.m01, self.m02, self.m03, self.m10, self.m11, self.m12, self.m13,
            self.m20, self.m21, self.m22, self.m23, self.m30, self.m31, self.m32, self.m33,
        ]
    }
    pub fn to_mat3f(&self) -> Matrix3f {
        let Matrix4f {
            m00,
            m01,
            m02,
            m03,
            m10,
            m11,
            m12,
            m13,
            m20,
            m21,
            m22,
            m23,
            m30,
            m31,
            m32,
            m33,
        } = self.clone();
        Matrix3f {
            m00,
            m01,
            m02,
            m10,
            m11,
            m12,
            m20,
            m21,
            m22,
        }
    }
    pub fn identity() -> Matrix4f {
        Matrix4f {
            m00: 1.,
            m01: 0.,
            m02: 0.,
            m03: 0.,
            m10: 0.,
            m11: 1.,
            m12: 0.,
            m13: 0.,
            m20: 0.,
            m21: 0.,
            m22: 1.,
            m23: 0.,
            m30: 0.,
            m31: 0.,
            m32: 0.,
            m33: 1.,
        }
    }
    pub fn row(&self, i: u32) -> Vec3f {
        match i {
            0 => Vec3f {
                x: self.m00,
                y: self.m01,
                z: self.m02,
            },
            1 => Vec3f {
                x: self.m10,
                y: self.m11,
                z: self.m12,
            },
            2 => Vec3f {
                x: self.m20,
                y: self.m22,
                z: self.m22,
            },
            3 => Vec3f {
                x: self.m30,
                y: self.m31,
                z: self.m32,
            },
            _ => panic!(""),
        }
    }
    pub fn matrix_along_x_axis(origin: Vec3f, target: Vec3f) -> Self {
        // basisMatrix.row(3) = origin;

        // Vector& xAxis = basisMatrix.Row(0);
        // Vector& yAxis = basisMatrix.Row(1);
        // Vector& zAxis = basisMatrix.Row(2);
        let mut x_axis = Vec3f {
            x: 1.,
            y: 0.,
            z: 0.,
        };
        let mut y_axis = Vec3f {
            x: 0.,
            y: 1.,
            z: 0.,
        };
        let mut z_axis = Vec3f {
            x: 0.,
            y: 0.,
            z: 1.,
        };

        // Vector yUp(0.0f,1.0f,0.0f);
        // Vector zUp(0.0f,0.0f,1.0f);
        let y_up = Vec3f {
            x: 0.,
            y: 1.,
            z: 0.,
        };
        let z_up = Vec3f {
            x: 0.,
            y: 0.,
            z: 1.,
        };

        x_axis = target.sub(&origin);
        x_axis = x_axis.normalize();

        let colinear_test = 0.95f32;
        if x_axis.y.abs() > colinear_test {
            // Use Z up for the building the matrix
            y_axis = Vec3f::cross(&z_up, &x_axis);
            y_axis = y_axis.normalize();
            z_axis = Vec3f::cross(&x_axis, &y_axis);
        } else {
            // Use Y up for the building the matrix
            z_axis = Vec3f::cross(&x_axis, &y_up);
            z_axis = z_axis.normalize();
            y_axis = Vec3f::cross(&z_axis, &x_axis);
        }

        let Vec3f {
            x: m00,
            y: m01,
            z: m02,
        } = x_axis;
        let Vec3f {
            x: m10,
            y: m11,
            z: m12,
        } = y_axis;
        let Vec3f {
            x: m20,
            y: m21,
            z: m22,
        } = z_axis;
        let Vec3f {
            x: m30,
            y: m31,
            z: m32,
        } = origin;

        // return basis_matrix;
        Matrix4f {
            m00,
            m01,
            m02,
            m03: 0.,
            m10,
            m11,
            m12,
            m13: 0.,
            m20,
            m21,
            m22,
            m23: 0.,
            m30,
            m31,
            m32,
            m33: 1.,
        }
    }
}
