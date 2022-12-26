mod vec3;
use std::fmt::Write as FmtWrite;
use std::io::{Result, Write};

pub fn write_color<T: Write>(out: &mut T, pixel_color: &vec3::Color) -> Result<usize> {
    let mut str = String::new();
    write!(
        str,
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32,
    )
    .expect("Error formatting write");

    out.write(str.as_bytes())
}
