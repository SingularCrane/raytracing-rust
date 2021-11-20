extern crate image;
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
    pub use crate::utils::*;
    pub use crate::vec3::*;
}

use crate::prelude::*;
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

struct ImageData {
    height: u32,
    width: u32,
    samples_per_pixel: usize,
    max_depth: usize,
    camera: Camera,
}

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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let mat_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        mat_ground.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0., random_range(0., 0.5), 0.);
                    world.add(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.,
                        1.,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_range(0., 0.5);
                    let sphere_materal = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_materal.clone())));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        material1.clone(),
    )));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.0,
        material2.clone(),
    )));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        material3,
    )));

    world
}

fn render_row(
    world: Arc<HittableList>,
    row_count: Arc<Mutex<u32>>,
    data: Arc<ImageData>,
    output: Arc<Mutex<image::RgbImage>>,
) {
    let w: &HittableList = &world;
    let mut rng = thread_rng();
    loop {
        let mut _current_row: u32 = 0;
        {
            let mut rc = row_count.lock().unwrap();
            _current_row = *rc;
            *rc += 1;
            if *rc > data.height {
                return;
            }
            eprint!("\rScanlines Remaining: {:04}", data.height - *rc);
        }
        for i in 0..data.width {
            let mut pixel_color = Color::new(0., 0., 0.);

            for _s in 0..data.samples_per_pixel {
                let x: f64 = rng.gen();
                let y: f64 = rng.gen();
                let u = (i as f64 + x) / (data.width as f64 - 1.0);
                let v = (_current_row as f64 + y) / (data.height as f64 - 1.0);
                let r = data.camera.get_ray(u, v);
                pixel_color += ray_color(&r, w, data.max_depth);
            }
            {
                let o = &output.clone();
                let mut o_mut = o.lock().unwrap();
                o_mut.put_pixel(
                    i,
                    data.height - _current_row - 1,
                    image::Rgb(write_color(pixel_color, data.samples_per_pixel)),
                )
            }
        }
    }
}

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 300;
    let image_height = (image_width as f64 / aspect_ratio).round() as u32;

    // world
    let world = Arc::new(random_scene());

    // camera
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.,
        1.,
    );

    let image_data = Arc::new(ImageData {
        height: image_height,
        width: image_width,
        samples_per_pixel: 50,
        max_depth: 50,
        camera: camera,
    });
    let image = Arc::new(Mutex::new(image::ImageBuffer::new(
        image_data.width,
        image_data.height,
    )));
    let row_count = Arc::new(Mutex::new(0));

    // render
    let mut handles = vec![];
    for _i in 0..4 {
        let w = world.clone();
        let rc = row_count.clone();
        let id = image_data.clone();
        let i = image.clone();
        let handle = thread::spawn(move || render_row(w, rc, id, i));
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    image.lock().unwrap().save("output.png").unwrap();
    eprint!("\nDone\n");
}
