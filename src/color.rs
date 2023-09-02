use crate::vec3::Vec3;
use crate::Interval;
use crate::Result;
use std::io::Write;
pub type Color = Vec3;
// each value in color need to be from 0 to 1
const COLOR_INTERVAL: Interval = Interval {
    min: 0.0,
    max: 0.9999,
};
pub fn write_color<W>(writer: &mut W, c: &Color, samples_per_pixel: u32) -> Result<()>
where
    W: Write,
{
    // Divide the color by number of samples per pixel
    let r = c.x() / samples_per_pixel as f64;
    let g = c.y() / samples_per_pixel as f64;
    let b = c.z() / samples_per_pixel as f64;

    write!(
        writer,
        "{} {} {}\n",
        (COLOR_INTERVAL.clamp(r) * 256.0) as u32,
        (COLOR_INTERVAL.clamp(g) * 256.0) as u32,
        (COLOR_INTERVAL.clamp(b) * 256.0) as u32
    )?;
    Ok(())
}
