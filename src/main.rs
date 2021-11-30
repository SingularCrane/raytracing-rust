extern crate image;
extern crate rand;

mod aabb;
mod aarect;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod utils;
mod vec3;

mod prelude {
    pub use crate::aabb::*;
    pub use crate::aarect::*;
    pub use crate::bvh::*;
    pub use crate::camera::*;
    pub use crate::color::*;
    pub use crate::hittable::*;
    pub use crate::hittable_list::*;
    pub use crate::material::*;
    pub use crate::perlin::*;
    pub use crate::ray::*;
    pub use crate::sphere::*;
    pub use crate::texture::*;
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
    background: Color,
}

fn ray_color(r: &Ray, background: Color, world: &dyn Hittable, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0.00001, f64::INFINITY) {
        let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);
        if let Some(scatter) = rec.mat.scatter(r, &rec) {
            return emitted + scatter.attenuation * ray_color(&scatter.ray, background, world, depth - 1);
        } else {
            return emitted;
        }
    } else {
        return background;
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker: Arc<CheckerTexture> = Arc::new(CheckerTexture::new_solid(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let mat_ground = Arc::new(Lambertian::new_textured(checker.clone()));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        mat_ground.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0., random_range(0., 0.5), 0.);
                    world.add(Arc::new(MovingSphere::new(
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
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_materal.clone())));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1.0, material1.clone())));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1.0, material2.clone())));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4., 1., 0.), 1.0, material3)));

    //HittableList::new_from_list(Arc::new(BVHNode::new_from_list(world, 0., 1.)))
    world
}

fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let checker: Arc<CheckerTexture> = Arc::new(CheckerTexture::new_solid(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Arc::new(Lambertian::new_textured(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        Arc::new(Lambertian::new_textured(checker.clone())),
    )));

    world
}

fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::new();

    let pertext: Arc<NoiseTexture> = Arc::new(NoiseTexture::new_scaled(4.));

    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_textured(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::new_textured(pertext.clone())),
    )));

    world
}

fn earth() -> HittableList {
    let mut world = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new(std::path::Path::new("earthmap.jpg")));
    let earth_surface = Arc::new(Lambertian::new_textured(earth_texture.clone()));
    world.add(Arc::new(Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface)));

    world
}

fn simple_light() -> HittableList {
    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new_scaled(4.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_textured(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_textured(pertext.clone())),
    )));

    let difflight = Arc::new(DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight.clone())));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 6.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world
}

fn cornell_box() -> HittableList {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.5, 0.5)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green.clone())));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red.clone())));
    world.add(Arc::new(XZRect::new(213.0, 343.0, 277.0, 332.0, 554.0, light.clone())));
    world.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    world.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.add(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

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
                pixel_color += ray_color(&r, data.background, w, data.max_depth);
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
    let threads = 8;
    // image
    let aspect_ratio = 1.;
    let image_width: u32 = 100;
    let image_height = (image_width as f64 / aspect_ratio).round() as u32;
    let scene = 5;

    // world
    let world = match scene {
        0 => Arc::new(random_scene()),
        1 => Arc::new(two_spheres()),
        2 => Arc::new(two_perlin_spheres()),
        3 => Arc::new(earth()),
        4 => Arc::new(simple_light()),
        _ => Arc::new(cornell_box()),
    };

    // camera
    let lookfrom = match scene {
        0 => Point3::new(13., 2., 3.),
        1 => Point3::new(13., 2., 2.),
        2 => Point3::new(13., 2., 2.),
        3 => Point3::new(13., 2., 2.),
        4 => Point3::new(23.0, 3.0, 6.0),
        _ => Point3::new(278.0, 278.0, -800.0),
    };
    let lookat = match scene {
        0 => Point3::new(0.0, 0.0, 0.0),
        1 => Point3::new(0.0, 0.0, 0.0),
        2 => Point3::new(0.0, 0.0, 0.0),
        3 => Point3::new(0.0, 0.0, 0.0),
        4 => Point3::new(0.0, 2.0, 0.0),
        _ => Point3::new(278.0, 278.0, 0.0),
    };
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = match scene {
        0 => 0.1,
        1 => 0.0,
        2 => 0.0,
        3 => 0.0,
        4 => 0.0,
        _ => 0.0,
    };
    let vfov = match scene {
        0 => 20.,
        1 => 20.,
        2 => 20.,
        3 => 20.,
        4 => 20.,
        _ => 40.,
    };
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.,
        1.,
    );

    let image_data = Arc::new(ImageData {
        height: image_height,
        width: image_width,
        samples_per_pixel: 200,
        max_depth: 50,
        camera: camera,
        background: match scene {
            0 => Color::new(0.7, 0.8, 1.0),
            1 => Color::new(0.7, 0.8, 1.0),
            2 => Color::new(0.7, 0.8, 1.0),
            3 => Color::new(0.7, 0.8, 1.0),
            4 => Color::new(0.0, 0.0, 0.0),
            _ => Color::new(0.0, 0.0, 0.0),
        },
    });
    let image = Arc::new(Mutex::new(image::ImageBuffer::new(image_data.width, image_data.height)));
    let row_count = Arc::new(Mutex::new(0));

    // render
    let mut handles = vec![];
    for _i in 0..threads {
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
