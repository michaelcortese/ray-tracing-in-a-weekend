use ray_tracing_in_a_weekend::{Color, write_color};

use std::io::Write;

fn main() {
    let image_width = 256;
    let image_height = 256;
    //     std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";
    println!("P3\n{} {}\n255", image_width, image_height);
    // render as ppm
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        std::io::stderr().flush().unwrap();
        // std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
        for i in 0..image_width {
            let pixel_color: Color = Color::from(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            write_color(pixel_color);
        }
    }
    eprint!("\rDone!                   \n");
}
