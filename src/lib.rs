#![feature(assoc_char_funcs)]

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageResult;

use std::path::Path;

mod braille;
use braille::*;

pub fn run(img: DynamicImage, scale: f64, invert: bool, threshold: u8) -> String {
    let (x, y) = img.dimensions();
    let (x, y) = (x as f64 * scale, y as f64 * scale);
    let (x, y) = (x as u32, y as u32);

    let resized = img.resize(x, y, FilterType::Lanczos3);
    let resized = resize_to_mod_braille(resized);

    let grayscale = resized.grayscale();

    let braille = to_braille(&grayscale, invert, threshold);
    braille.into()
}

pub fn get_image<P: AsRef<Path>>(filename: P) -> ImageResult<DynamicImage> {
    let reader = ImageReader::open(filename)?;
    reader.decode()
}

fn resize_to_mod_braille(img: DynamicImage) -> DynamicImage {
    let (x, y) = img.dimensions();
    let mut changed = false;
    let x = if x % 2 != 0 {
        changed = true;
        x - 1
    } else {
        x
    };

    let rem = y.rem_euclid(4);
    let y = if rem != 0 {
        if rem > 4 - rem {
            y + (4 - rem)
        } else {
            y - rem
        }
    } else {
        y
    };

    if changed {
        img.resize_exact(x, y, FilterType::Lanczos3)
    } else {
        img
    }
}

fn to_braille(img: &DynamicImage, invert: bool, threshold: u8) -> BasicMatrix<BrailleChar> {
    let (img_x, img_y) = img.dimensions();
    let mut matrix: BasicMatrix<BrailleChar> =
        BasicMatrix::generate(img_x as usize / 2, img_y as usize / 4, |_, _| {
            BrailleChar::default()
        })
        .unwrap();

    let (m, n) = matrix.dimensions();
    let luma8 = img.as_luma8().unwrap();

    for m_i in 0..m {
        for m_j in 0..n {
            let (i, j) = (m_i as u32 * 2, m_j as u32 * 4);

            for o_i in 0..2 {
                for o_j in 0..4 {
                    let is_dot = luma8.get_pixel(i + o_i as u32, j + o_j as u32).0[0] > threshold;
                    let is_dot = if invert { !is_dot } else { is_dot };

                    matrix[(m_i, m_j)].bits[o_i][o_j] = is_dot;
                }
            }
        }
    }

    matrix
}

impl Into<String> for BasicMatrix<BrailleChar> {
    fn into(self) -> String {
        let (m, n) = self.dimensions();
        let mut output = String::with_capacity(4 * m * n + m); // no. of braille chars + no. of newlines

        for j in 0..n {
            for i in 0..m {
                output.push(self[(i, j)].into());
            }
            if j != n - 1 {
                output.push('\n');
            }
        }

        output
    }
}
