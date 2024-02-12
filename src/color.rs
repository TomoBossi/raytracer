use std::fs::File;
use std::path::PathBuf;
use std::env::current_dir;
use std::io::{BufWriter, Write};
use image::{RgbImage, ImageBuffer, Rgb};

use crate::vec3::Vec3;
use crate::interval::Interval;

pub fn write_ppm<W: std::io::Write>(writer: &mut BufWriter<W>, v: Vec3) {
    let intensity: Interval = Interval{min: 0., max: 0.999};
    let r: u8 = (256.*intensity.clamp(v.0).sqrt()) as u8;
    let g: u8 = (256.*intensity.clamp(v.1).sqrt()) as u8;
    let b: u8 = (256.*intensity.clamp(v.2).sqrt()) as u8;
    writeln!(writer, "{} {} {}", r, g, b).expect("Writing failed")
}

pub fn write_png(path: String, img_matrix: Vec<Vec<Vec3>>) {
    let h: u32 = img_matrix.len() as u32;
    let w: u32 = img_matrix[0].len() as u32;
    let mut img: RgbImage = ImageBuffer::new(w, h);
    let intensity: Interval = Interval{min: 0., max: 0.999};
    let px_idx: u32 = 0;
    for (i, j, pixel) in img.enumerate_pixels_mut() {
        let v: Vec3 = img_matrix[j as usize][i as usize];
        let r: u8 = (256.*intensity.clamp(v.0).sqrt()) as u8;
        let g: u8 = (256.*intensity.clamp(v.1).sqrt()) as u8;
        let b: u8 = (256.*intensity.clamp(v.2).sqrt()) as u8;
        *pixel = Rgb([r, g, b]);
    }
    let root: PathBuf = current_dir().unwrap();
    let full_path: PathBuf = root.join(path);
    img.save(&full_path);
}