use crate::vec3::Vec3;
use crate::Result;
use std::io::Write;
pub type Color = Vec3;
pub fn write_color<W>(writer: &mut W, c: &Color) -> Result<()>
where
    W: Write,
{
    write!(
        writer,
        "{} {} {}\n",
        (c.x().clone() * 255.999) as u32,
        (c.y().clone() * 255.999) as u32,
        (c.z() * 255.999) as u32
    )?;
    Ok(())
}
