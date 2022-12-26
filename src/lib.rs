mod vec3;
mod ray;

use std::fmt::Write as FmtWrite;
use std::io;
use std::io::{Result, Write};
use vec3::Color;

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

pub fn write_image() {
    // Image
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Render
    println!("P3\n{} {}\n255\n", &IMAGE_WIDTH, &IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", &j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );

            write_color(&mut io::stdout(), &pixel_color).expect("Error writing to output");
        }
        eprintln!("\nDone.\n")
    }
}
