extern crate rand;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

mod prelude {
    pub use crate::camera::*;
    pub use crate::color::*;
    pub use crate::hittable::*;
    pub use crate::hittable_list::*;
    pub use crate::material::*;
    pub use crate::ray::*;
    pub use crate::sphere::*;
    pub use crate::vec3::*;
}

use crate::prelude::*;
use rand::prelude::*;
use std::rc::Rc;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(scatter) = rec.mat.scatter(r, &rec) {
            return scatter.attenuation * ray_color(&scatter.ray, world, depth - 1);
        }
        return Color::new(0., 0., 0.);
    }
    let unit_direction: Vec3 = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut rng = thread_rng();
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio).round() as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // world
    let mut world = HittableList::new();

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        mat_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        mat_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        mat_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        mat_right,
    )));

    // camera
    let camera = Camera::new();
    // render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines Remaining: {:04}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);

            for _s in 0..samples_per_pixel {
                let x: f64 = rng.gen();
                let y: f64 = rng.gen();
                let u = (i as f64 + x) / (image_width as f64 - 1.0);
                let v = (j as f64 + y) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            println!("{}", write_color(pixel_color, samples_per_pixel));
        }
    }
    eprint!("\nDone\n");
}
