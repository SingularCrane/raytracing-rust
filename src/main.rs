mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

mod prelude {
    pub use crate::color::*;
    pub use crate::hittable::*;
    pub use crate::hittable_list::*;
    pub use crate::ray::*;
    pub use crate::sphere::*;
    pub use crate::vec3::*;
}

use crate::prelude::*;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0., f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1., 1., 1.));
    }
    let unit_direction: Vec3 = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio).round() as u32;

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);
    // render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines Remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel_color = ray_color(&r, &world);
            println!("{}", write_color(pixel_color));
        }
    }
    eprint!("\nDone\n");
}
