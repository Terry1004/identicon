use std::fmt;
use std::result;

use image::Rgb;
use thiserror::Error;

pub const HUE_MAX: u16 = 360;
pub const SAT_MAX: u16 = 100;
pub const LUM_MAX: u16 = 100;
const RGB_MAX: u16 = 255;

#[derive(Debug, PartialEq, Clone)]
pub struct RGB(Rgb<u8>);

pub struct HSL {
    hue: f32, // range: [0, 360]
    sat: f32, // range: [0, 100]
    lum: f32, // range: [0, 100]
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("expect {name} between 0.0 and {max} but found {val}")]
    HSLOutOfBounds {
        name: &'static str,
        val: f32,
        max: f32,
    },
}

pub type Result<T> = result::Result<T, Error>;

impl RGB {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self(Rgb([red, green, blue]))
    }

    pub fn as_pixel(&self) -> Rgb<u8> {
        self.0
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.0 .0[0], self.0 .0[1], self.0 .0[2])
    }
}

impl HSL {
    pub fn new(hue: f32, sat: f32, lum: f32) -> Result<Self> {
        if hue < 0.0 || hue > f32::from(HUE_MAX) {
            Err(Error::HSLOutOfBounds {
                name: "hue",
                val: hue,
                max: f32::from(HUE_MAX),
            })
        } else if sat < 0.0 || sat > f32::from(SAT_MAX) {
            Err(Error::HSLOutOfBounds {
                name: "sat",
                val: sat,
                max: f32::from(SAT_MAX),
            })
        } else if lum < 0.0 || lum > f32::from(LUM_MAX) {
            Err(Error::HSLOutOfBounds {
                name: "lum",
                val: lum,
                max: f32::from(LUM_MAX),
            })
        } else {
            Ok(Self {
                hue: hue,
                sat: sat,
                lum: lum,
            })
        }
    }

    pub fn as_rgb(&self) -> RGB {
        let hue = self.hue / f32::from(HUE_MAX);
        let sat = self.sat / f32::from(SAT_MAX);
        let lum = self.lum / f32::from(LUM_MAX);

        let s = if lum < 0.5 { lum } else { 1.0 - lum };
        let c = 2.0 * s * sat;
        let m = lum - c / 2.0;
        let r = Self::compute_rgb(c, m, hue + 1.0 / 3.0);
        let g = Self::compute_rgb(c, m, hue);
        let b = Self::compute_rgb(c, m, hue - 1.0 / 3.0);

        RGB::new(
            (r * f32::from(RGB_MAX)).round() as u8,
            (g * f32::from(RGB_MAX)).round() as u8,
            (b * f32::from(RGB_MAX)).round() as u8,
        )
    }

    fn compute_rgb(c: f32, m: f32, h: f32) -> f32 {
        let h = if h < 0.0 {
            h + 1.0
        } else if h > 1.0 {
            h - 1.0
        } else {
            h
        };

        if h < 1.0 / 6.0 {
            6.0 * c * h + m
        } else if h < 1.0 / 2.0 {
            c + m
        } else if h < 2.0 / 3.0 {
            c * (4.0 - 6.0 * h) + m
        } else {
            m
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HSL, RGB};

    #[test]
    fn to_black() {
        let black = RGB::new(0, 0, 0);
        let rgb: RGB = HSL::new(0.0, 0.0, 0.0).unwrap().as_rgb();
        assert_eq!(black, rgb);
    }

    #[test]
    fn to_white() {
        let white = RGB::new(255, 255, 255);
        let rgb: RGB = HSL::new(0.0, 0.0, 100.0).unwrap().as_rgb();
        assert_eq!(white, rgb);
    }

    #[test]
    fn to_red() {
        let red = RGB::new(255, 0, 0);
        let rgb: RGB = HSL::new(0.0, 100.0, 50.0).unwrap().as_rgb();
        assert_eq!(red, rgb);
    }

    #[test]
    fn to_green() {
        let green = RGB::new(0, 255, 0);
        let rgb: RGB = HSL::new(120.0, 100.0, 50.0).unwrap().as_rgb();
        assert_eq!(green, rgb);
    }

    #[test]
    fn to_blue() {
        let blue = RGB::new(0, 0, 255);
        let rgb: RGB = HSL::new(240.0, 100.0, 50.0).unwrap().as_rgb();
        assert_eq!(blue, rgb);
    }

    #[test]
    fn to_random_1() {
        let color = RGB::new(130, 121, 23);
        let rgb: RGB = HSL::new(55.2, 70.0, 30.0).unwrap().as_rgb();
        assert_eq!(color, rgb)
    }

    #[test]
    fn to_random_2() {
        let color = RGB::new(75, 235, 72);
        let rgb: RGB = HSL::new(118.7, 80.4, 60.2).unwrap().as_rgb();
        assert_eq!(color, rgb);
    }

    #[test]
    fn to_random_3() {
        let color = RGB::new(32, 60, 75);
        let rgb: RGB = HSL::new(201.3, 40.2, 20.9).unwrap().as_rgb();
        assert_eq!(color, rgb);
    }

    #[test]
    fn to_random_4() {
        let color = RGB::new(242, 211, 220);
        let rgb: RGB = HSL::new(343.4, 55.3, 88.9).unwrap().as_rgb();
        assert_eq!(color, rgb);
    }
}
