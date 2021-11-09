const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines Remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / IMAGE_WIDTH as f64;
            let g = j as f64 / IMAGE_HEIGHT as f64;
            let b: f64 = 0.25;

            let ir: u32 = (255.999 * r).round() as u32;
            let ig: u32 = (255.999 * g).round() as u32;
            let ib: u32 = (255.999 * b).round() as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\nDone\n");
}
