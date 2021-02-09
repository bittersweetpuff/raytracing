use super::vec3;
pub use vec3::{cross, dot, unit_vector, Vec3};
pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    );
}
