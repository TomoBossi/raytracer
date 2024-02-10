use crate::vec3::Vec3;

pub fn write_color(v: Vec3) {
    let rgb = 255.99*v;
    println!("{} {} {}", rgb.0 as u8, rgb.1 as u8, rgb.2 as u8)
}