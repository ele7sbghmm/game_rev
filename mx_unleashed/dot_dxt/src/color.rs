use nom::{number::complete::le_u8, sequence::tuple, Parser};

#[derive(Debug, Clone)]
pub struct Rgba {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}
impl Rgba {
    pub fn from_bytes_bgr565<'a>() -> impl Parser<&'a [u8], Rgba, ()> {
        move |input: &'a [u8]| {
            let (input, (b0, b1)) = tuple((le_u8::<&'a [u8], ()>, le_u8))(input).unwrap();
            let red = (b1 & 0b1111_1000) >> 3;
            let green = ((b1 & 0b111) << 3) + ((b0 & 0b1110_0000) >> 5);
            let blue = b0 & 0b1_1111;
            let alpha = std::u8::MAX;

            Ok((
                input,
                Rgba {
                    red,
                    green,
                    blue,
                    alpha,
                },
            ))
        }
    }
    pub fn from_bytes_rgb565<'a>() -> impl Parser<&'a [u8], Rgba, ()> {
        move |input: &'a [u8]| {
            let (input, (b0, b1)) = tuple((le_u8::<&'a [u8], ()>, le_u8))(input).unwrap();
            let red = (b0 & 0b1111_1000) >> 3;
            let green = ((b0 & 0b111) << 3) & ((b1 & 0b1110_0000) >> 5);
            let blue = b1 & 0b1_1111;
            let alpha = std::u8::MAX;

            Ok((
                input,
                Rgba {
                    red,
                    green,
                    blue,
                    alpha,
                },
            ))
        }
    }
    pub fn to_bytes(&self) -> [u8; 4] {
        let Rgba {
            red,
            green,
            blue,
            alpha,
        } = self.clone();
        [red, green, blue, alpha]
    }
}
