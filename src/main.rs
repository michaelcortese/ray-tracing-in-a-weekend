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
            let r: f64 = i as f64 / (image_width - 1) as f64;
            let g: f64 = j as f64 / (image_height - 1) as f64;
            let b: f64 = 0.0;

            // clamp to range 0-255
            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            // output
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\rDone!                   \n");
}
