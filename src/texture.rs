use crate::color::*;
use crate::perlin::*;
use crate::ray::*;
use std::sync::Arc;

extern crate image;
use image::io::Reader as ImageReader;
use image::RgbImage;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> SolidColor {
        SolidColor { color_value: c }
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color_value: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new_solid(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1., 1., 1.)
            * 0.5
            * (1. + (self.scale * p.z() + 10. * self.noise.turb(p, 7)).sin())
        // Color::new(1., 1., 1.) * self.noise.turb(self.scale * p, 7)
    }
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: 1.,
        }
    }

    pub fn new_scaled(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: scale,
        }
    }
}

pub struct ImageTexture {
    image: RgbImage,
    width: u32,
    height: u32,
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let uc = u.clamp(0., 1.);
        let vc = 1. - v.clamp(0., 1.);

        let mut i: u32 = (uc * (self.width as f64)) as u32;
        let mut j: u32 = (vc * (self.height as f64)) as u32;
        i = if i >= self.width { self.width - 1 } else { i };
        j = if j >= self.height { self.height - 1 } else { j };

        let color_scale = 1. / 255.;
        let p = self.image.get_pixel(i, j);

        Color::new(
            color_scale * (p[0] as f64),
            color_scale * (p[1] as f64),
            color_scale * (p[2] as f64),
        )
    }
}

impl ImageTexture {
    pub fn new(p: &std::path::Path) -> ImageTexture {
        let i = ImageReader::open(p).unwrap().decode().unwrap().to_rgb8();
        let (w, h) = i.dimensions();
        ImageTexture {
            image: i,
            width: w,
            height: h,
        }
    }
}
