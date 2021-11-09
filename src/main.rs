mod color;
mod vec3;

mod prelude {
    pub use crate::color::*;
    pub use crate::vec3::*;
}

use crate::prelude::*;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines Remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let color = Color {
                x: i as f64 / IMAGE_WIDTH as f64,
                y: j as f64 / IMAGE_HEIGHT as f64,
                z: 0.25,
            };
            println!("{}", write_color(color));
        }
    }
    eprint!("\nDone\n");
}
