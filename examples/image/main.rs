extern crate raytracer;

use std::fs::File;

use raytracer::{
    color::Color,
    ppm::PPM,
};

fn main() -> Result<(), std::io::Error> {
    let mut ppm = PPM::new();

    let nx = 200;
    let ny = 100;
    ppm.reserve(nx, ny);
    for i in 0..nx {
        for j in 0..ny {
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.2f64;
            let ir = (255.99*r) as u8;
            let ig = (255.99*g) as u8;
            let ib = (255.99*b) as u8;
            let x = i;
            let y = ny - j - 1;
            let color = Color::Bit8(ir, ig, ib);
            ppm.set_pixel(x, y, color);
        }
    }

    let mut file = File::create("test.ppm")?;
    ppm.write(file);

    Ok(())
}
