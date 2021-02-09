mod vec3;
pub use vec3::{cross, dot, unit_vector, Vec3};
mod color;
pub use color::{write_color, Color};

fn main() {
    //Image

    let image_width = 256;
    let image_height = 256;

    //Render

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color: Color;
            pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            write_color(pixel_color);
        }
    }
}
