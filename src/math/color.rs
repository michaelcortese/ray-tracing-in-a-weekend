use super::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(color: Color) -> () {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let ir = (255.999 * r) as u32;
    let ig = (255.999 * g) as u32;
    let ib = (255.999 * b) as u32;

    println!("{} {} {}", ir, ig, ib);
}
