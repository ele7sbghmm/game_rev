use nom::{multi::count, number::complete::le_u8, IResult, Parser};
use std::cmp;

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    R,
    G,
    B,
    A,
}
impl Color {
    fn index(&self) -> usize {
        match self {
            Color::R => 0,
            Color::G => 1,
            Color::B => 2,
            Color::A => 3,
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Rgba {
    pub fn from_bytes_rgba<'a>(input: &'a [u8]) -> IResult<&'a [u8], Rgba, ()> {
        let (input, r) = le_u8::<&'a [u8], ()>(input).unwrap();
        let (input, g) = le_u8::<&'a [u8], ()>(input).unwrap();
        let (input, b) = le_u8::<&'a [u8], ()>(input).unwrap();
        let (input, a) = le_u8::<&'a [u8], ()>(input).unwrap();

        Ok((input, Rgba { r, g, b, a }))
    }
    pub fn from_bytes_rgb<'a>(input: &'a [u8]) -> IResult<&'a [u8], Rgba, ()> {
        let (input, r) = le_u8::<&'a [u8], ()>(input).unwrap();
        let (input, g) = le_u8::<&'a [u8], ()>(input).unwrap();
        let (input, b) = le_u8::<&'a [u8], ()>(input).unwrap();

        Ok((input, Rgba { r, g, b, a: 0xff }))
    }
    pub fn from_bytes_u8<'a>(input: &'a [u8]) -> IResult<&'a [u8], Rgba, ()> {
        let (input, r) = le_u8::<&'a [u8], ()>(input).unwrap();

        Ok((
            input,
            Rgba {
                r,
                g: r,
                b: r,
                a: 0xff,
            },
        ))
    }
    pub fn from_bytes_rrgg_bbaa<'a>(input: &'a [u8]) -> IResult<&'a [u8], Rgba, ()> {
        let (input, r) = le_u8::<&'a [u8], ()>(input).unwrap();

        Ok((
            input,
            Rgba {
                r: (r & 0b1100_0000) >> 6,
                g: (r & 0b11_0000) >> 4,
                b: (r & 0b1100) >> 2,
                a: (r & 0b11) >> 0,
            },
        ))
    }
    pub fn from_bytes_rgb565<'a>(input: &'a [u8]) -> IResult<&'a [u8], Rgba, ()> {
        let (input, b0) = le_u8::<&'a [u8], ()>(input).unwrap();
        let (input, b1) = le_u8::<&'a [u8], ()>(input).unwrap();

        //     b0        b1
        // rrrr-rggg gggb-bbbb
        // ____ _>>>              >3
        //       &&& ___> >>>> &3 >5
        //              & &&&& &5

        let r = b0 >> 3;
        let g = ((b0 & 0b111) << 3) + (b1 >> 5);
        let b = (b1 & 0b1_1111);

        Ok((input, Rgba { r, g, b, a: 0xff }))
    }
    pub fn from_bytes_bgr565<'a>(input: &'a [u8]) -> IResult<&'a [u8], Rgba, ()> {
        let (input, b0) = le_u8::<&'a [u8], ()>(input).unwrap();
        let (input, b1) = le_u8::<&'a [u8], ()>(input).unwrap();

        let b = b0 >> 3;
        let g = (b0 & 0b111) + (b1 >> 5);
        let r = (b1 & 0b1_1111);

        Ok((input, Rgba { r, g, b, a: 0xff }))
    }
    pub fn to_rbg(&self) -> Self {
        let Self { r, g, b, a } = self.clone();
        Self { r, g, b, a: 0xff }
    }
    fn swap(&self, x: &Color, y: &Color) -> Self {
        let mut t = self.to_tuple();

        let tx = t[x.index()];
        let ty = t[y.index()];

        t[y.index()] = tx;
        t[x.index()] = ty;

        Rgba::from_tuple(t)
    }
    fn to_tuple(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
    fn from_tuple(t: [u8; 4]) -> Self {
        let [r, g, b, a] = t;
        Rgba { r, g, b, a }
    }
    fn as_vec(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }
    fn scale(&self, factor: u32) -> Rgba {
        let r: u8 = cmp::min(factor * self.r as u32, 0xff) as u8;
        let g: u8 = cmp::min(factor * self.g as u32, 0xff) as u8;
        let b: u8 = cmp::min(factor * self.b as u32, 0xff) as u8;
        let a: u8 = cmp::min(factor * self.a as u32, 0xff) as u8;
        Rgba { r, g, b, a }
    }
    fn strip(&self, r: bool, g: bool, b: bool, a: bool) -> Self {
        let mut tmp = self.clone();
        if r == false {
            tmp.r = 0;
        }
        if g == false {
            tmp.g = 0;
        }
        if b == false {
            tmp.b = 0;
        }
        if a == false {
            tmp.a = 0;
        }
        tmp
    }
    fn to_byte(&self, color: &Color) -> u8 {
        let index = color.index();
        self.to_tuple()[index]
    }
    // var rgb = HSVtoRGB(p/100.0*0.85, 1.0, 1.0);

    /// -> https://stackoverflow.com/questions/32470555/javascript-algorithm-function-to-generate-rgb-values-for-a-color-along-the-visib
    // function HSVtoRGB(h, s, v) {
    //     var r, g, b, i, f, p, q, t;
    //     if (arguments.length === 1) {
    //         s = h.s, v = h.v, h = h.h;
    //     }
    //     i = Math.floor(h * 6);
    //     f = h * 6 - i;
    //     p = v * (1 - s);
    //     q = v * (1 - f * s);
    //     t = v * (1 - (1 - f) * s);
    //     switch (i % 6) {
    //         case 0: r = v, g = t, b = p; break;
    //         case 1: r = q, g = v, b = p; break;
    //         case 2: r = p, g = v, b = t; break;
    //         case 3: r = p, g = q, b = v; break;
    //         case 4: r = t, g = p, b = v; break;
    //         case 5: r = v, g = p, b = q; break;
    //     }
    //     return {
    //         r: Math.round(r * 255),
    //         g: Math.round(g * 255),
    //         b: Math.round(b * 255)
    //     };
    // }
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let (mut r, mut g, mut b, mut i, mut f, mut p, mut q, mut t) =
            (0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);

        i = (h * 6f32).floor();
        f = (h * 6f32) - i;
        p = v * (1f32 - s);
        q = v * (1f32 - (f * s));
        t = v * (1f32 - (1f32 - f) * s);

        match (i % 6f32) {
            0.0 => {
                (r, g, b) = (v, g, t);
            }
            1.0 => {
                (r, g, b) = (q, v, p);
            }
            2.0 => {
                (r, g, b) = (p, v, t);
            }
            3.0 => {
                (r, g, b) = (p, q, v);
            }
            4.0 => {
                (r, g, b) = (t, p, v);
            }
            5.0 => {
                (r, g, b) = (v, p, q);
            }
            _ => panic!(),
        }

        Rgba {
            r: (r * 255f32).round().to_le_bytes()[3],
            g: (g * 255f32).round().to_le_bytes()[3],
            b: (b * 255f32).round().to_le_bytes()[3],
            a: 0xff,
        }
    }
    pub fn hsv2rgb(&self) -> Self {
        todo!()
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct RgbaArray {
    pub inner: Vec<Rgba>,
}
impl RgbaArray {
    fn new() -> Self {
        RgbaArray { inner: Vec::new() }
    }
    pub fn from(inner: Vec<Rgba>) -> Self {
        RgbaArray { inner }
    }
    pub fn scale(&self, factor: u32) -> RgbaArray {
        let mut inner: Vec<Rgba> = vec![];
        for rgba in self.inner.iter() {
            inner.push(rgba.scale(factor));
        }
        RgbaArray { inner }
    }
    pub fn from_bytes_rgba<'a>(num: usize) -> impl Parser<&'a [u8], Self, ()> {
        move |input: &'a [u8]| {
            let (input, inner) = count(Rgba::from_bytes_rgba, num)(input)?;
            Ok((input, Self { inner }))
        }
    }
    pub fn from_bytes_rgb<'a>(num: usize) -> impl Parser<&'a [u8], Self, ()> {
        move |input: &'a [u8]| {
            let (input, inner) = count(Rgba::from_bytes_rgb, num)(input)?;
            Ok((input, Self { inner }))
        }
    }
    pub fn from_bytes_u8<'a>(num: usize) -> impl Parser<&'a [u8], Self, ()> {
        move |input: &'a [u8]| {
            let (input, inner) = count(Rgba::from_bytes_u8, num)(input)?;
            Ok((input, Self { inner }))
        }
    }
    pub fn from_bytes_rrgg_bbaa<'a>(num: usize) -> impl Parser<&'a [u8], Self, ()> {
        move |input: &'a [u8]| {
            let (input, inner) = count(Rgba::from_bytes_rrgg_bbaa, num)(input)?;
            Ok((input, Self { inner }))
        }
    }
    pub fn from_bytes_rgb565<'a>(num: usize) -> impl Parser<&'a [u8], Self, ()> {
        move |input: &'a [u8]| {
            let (input, inner) = count(Rgba::from_bytes_rgb565, num)(input)?;

            Ok((input, Self { inner }))
        }
    }
    pub fn from_bytes_bgr565<'a>(num: usize) -> impl Parser<&'a [u8], Self, ()> {
        move |input: &'a [u8]| {
            let (input, inner) = count(Rgba::from_bytes_bgr565, num)(input)?;

            Ok((input, Self { inner }))
        }
    }
    pub fn to_rgb(&self) -> Self {
        Self {
            inner: self.inner.iter().map(|rgba| rgba.to_rbg()).collect(),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        for rgba in self.inner.clone().iter() {
            bytes.append(&mut rgba.as_vec());
        }
        bytes
    }
    pub fn to_byte(&self, color: &Color) -> Vec<u8> {
        let mut byte_array: Vec<u8> = vec![];
        for rgba in self.inner.clone().iter() {
            byte_array.push(rgba.to_byte(color));
        }
        byte_array
    }
    pub fn swap(&self, x: &Color, y: &Color) -> Self {
        let mut inner: Vec<Rgba> = vec![];
        for rgba in self.inner.iter() {
            inner.push(rgba.swap(x, y));
        }
        RgbaArray { inner }
    }
    pub fn strip(&self, r: bool, g: bool, b: bool, a: bool) -> Self {
        let mut inner: Vec<Rgba> = vec![];
        for rgba in self.inner.iter() {
            inner.push(rgba.strip(r, g, b, a));
        }
        RgbaArray { inner }
    }
    pub fn set_max(&self, mut mx: &mut Vec<u8>, color: &Color) -> &Self {
        let last = mx[mx.len() - 1];
        for rgba in self.inner.iter() {
            let col: u8 = rgba.to_owned().to_tuple()[color.index()];
            if col > last {
                mx.push(col);
            }
        }
        self
    }
}
