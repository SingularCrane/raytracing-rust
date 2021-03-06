use crate::vec3;

pub type Color = vec3::Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: usize) -> [u8; 3] {
    let mut r = pixel_color.a[0];
    let mut g = pixel_color.a[1];
    let mut b = pixel_color.a[2];

    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();
    [
        (256. * r.clamp(0., 0.999)) as u8,
        (256. * g.clamp(0., 0.999)) as u8,
        (256. * b.clamp(0., 0.999)) as u8,
    ]
}
