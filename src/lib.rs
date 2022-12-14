use std::io::Cursor;
use std::path::Path;
use std::result;

use image::{ImageError, ImageOutputFormat, RgbImage};
use md5::{Digest, Md5};
use thiserror::Error;

pub mod color;
mod base64;

const SAT_MIN: u16 = 45;
const SAT_MAX: u16 = 65;
const LUM_MIN: u16 = 55;
const LUM_MAX: u16 = 75;
const NUM_SQUARES: u8 = 7;
const JPEG_QUALITY: u8 = 255;

type HashBytes = [u8; 16];
type Paints = [bool; 15];

#[derive(Debug)]
pub struct Identicon {
    paints: Paints,
    size: u32,
    foreground: color::RGB,
    background: color::RGB,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    InvalidHSL(#[from] color::Error),
    #[error("encounter error saving image: {0}")]
    SaveImage(#[from] ImageError),
}

pub type Result<T> = result::Result<T, Error>;

impl Identicon {
    // name: the input string to generate identicon
    // size: the number of pixels of each square in the resulting image
    // background: the background color
    pub fn new(name: &str, size: u32, background: color::RGB) -> Result<Self> {
        let mut hasher = Md5::new();
        hasher.update(name);
        let hash: HashBytes = hasher.finalize().into();

        let foreground = Self::compute_fg(&hash)?;
        let mut paints: Paints = [false; 15];
        Self::paint(&hash, &mut paints);

        Ok(Self {
            paints: paints,
            size: size,
            foreground: foreground,
            background: background,
        })
    }

    pub fn render(&self, path: &Path) -> Result<()> {
        Ok(self.image().save(path)?)
    }

    // encode the formatted image using base64
    fn format(&self, format: ImageOutputFormat) -> Result<String> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut bytes);
        self.image().write_to(&mut cursor, format)?;
        Ok(base64::encode(&bytes))
    }

    pub fn png(&self) -> Result<String> {
        self.format(ImageOutputFormat::Png)
    }

    pub fn jpeg(&self) -> Result <String> {
        self.format(ImageOutputFormat::Jpeg(JPEG_QUALITY))
    }

    pub fn gif(&self) -> Result<String> {
        self.format(ImageOutputFormat::Gif)
    }

    fn image(&self) -> RgbImage {
        let size = self.size * u32::from(NUM_SQUARES);
        let mut img = RgbImage::from_pixel(size, size, self.background.as_pixel());
        let num_center_cols = NUM_SQUARES / 2;
        for (i, paint) in self.paints.iter().enumerate() {
            let row = 1 + i as u8 / num_center_cols;
            let col = 1 + i as u8 % num_center_cols;
            let row_pixel = u32::from(row) * self.size;
            let col_pixel = u32::from(col) * self.size;

            for x in col_pixel..col_pixel + self.size {
                for y in row_pixel..row_pixel + self.size {
                    if *paint {
                        img.put_pixel(x, y, self.foreground.as_pixel());
                        img.put_pixel(size - 1 - x, y, self.foreground.as_pixel());
                    }
                }
            }
        }

        img
    }

    fn compute_fg(hash: &HashBytes) -> Result<color::RGB> {
        let h1 = (u16::from(hash[12]) & 0x0f) << 8;
        let h2 = u16::from(hash[13]);

        let hue = h1 | h2; // max 12 bits
        let sat = hash[14];
        let lum = hash[15];

        let hue = Self::map(f32::from(hue), 0.0, 4095.0, 0.0, f32::from(color::HUE_MAX));
        let sat = Self::map(
            f32::from(sat),
            0.0,
            255.0,
            f32::from(SAT_MAX),
            f32::from(SAT_MIN),
        );
        let lum = Self::map(
            f32::from(lum),
            0.0,
            255.0,
            f32::from(LUM_MAX),
            f32::from(LUM_MIN),
        );

        Ok(color::HSL::new(hue, sat, lum)?.as_rgb())
    }

    // linearly map val in [vmin, vmax] to [dmin, dmax]
    fn map(val: f32, vmin: f32, vmax: f32, dmin: f32, dmax: f32) -> f32 {
        dmin + ((val - vmin) * (dmax - dmin)) / (vmax - vmin)
    }

    fn paint(hash: &HashBytes, paints: &mut Paints) {
        let len = paints.len();
        let nibbles = hash
            .iter()
            .flat_map(|b| [(b & 0xf0) >> 4, (b & 0x0f)])
            .take(len);

        let num_cols = usize::from(NUM_SQUARES / 2);
        let num_rows = usize::from(NUM_SQUARES - 2);
        for (i, nibble) in nibbles.enumerate() {
            let col = 2 - i / num_rows;
            let row = i % num_rows;
            let idx = row * num_cols + col;
            paints[idx] = nibble % 2 == 0;
        }
    }
}
