mod vec3;
pub use vec3::{Vec3, cross, dot, unit_vector};

fn main() {

    //Image

    let image_width = 5;
    let image_height = 5;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    let mut r: f64;
    let mut g: f64;
    let mut b: f64;

    let mut ir: i32;
    let mut ig: i32;
    let mut ib: i32;
    for i in 0..image_height {
        for j in 0..image_width {
            r = i as f64 / (image_width-1) as f64;
            g = j as f64 / (image_height-1) as f64;
            b = 0.25;

            ir = (255.999 * r) as i32;
            ig = (255.999 * g) as i32;
            ib = (255.999 * b) as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }

}
