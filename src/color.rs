use crate::vec3;

pub type Color = vec3::Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: usize) -> String {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();
    format!(
        "{} {} {}",
        (256. * r.clamp(0., 0.999)) as u32,
        (256. * g.clamp(0., 0.999)) as u32,
        (256. * b.clamp(0., 0.999)) as u32,
    )
}
