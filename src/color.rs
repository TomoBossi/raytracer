use crate::vec3::Vec3;
use crate::interval::Interval;

pub fn write_color(v: Vec3) {
    let intensity: Interval = Interval{min: 0., max: 0.999};
    let r: u8 = (256.*intensity.clamp(v.0)) as u8;
    let g: u8 = (256.*intensity.clamp(v.1)) as u8;
    let b: u8 = (256.*intensity.clamp(v.2)) as u8;
    println!("{} {} {}", r, g, b)
}