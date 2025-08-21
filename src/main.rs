use humantime::format_duration;
use minifb::{Key, Window, WindowOptions};
use ppm::Image;
use std::{env, path::Path, process::exit, time::Instant};

const MAX_WIDTH: usize = 800;
const MAX_HEIGHT: usize = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ppm <input_file>");
        exit(1);
    }

    let start = Instant::now();

    let image = Image::from_file(Path::new(&args[1]))?;
    let buffer = image.to_minifb_buffer();

    let duration = start.elapsed();

    println!("Magic number: {}", image.magic_number);
    println!("Image width: {}", image.width);
    println!("Image height: {}", image.height);
    println!("Max value: {}", image.max_val);

    println!("\nDuration: {}", format_duration(duration));

    let window_width = image.width.min(MAX_WIDTH);
    let window_height = image.height.min(MAX_HEIGHT);

    let mut window = Window::new(
        &image.filename,
        window_width,
        window_height,
        WindowOptions::default(),
    )?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, image.width, image.height)?;
    }

    Ok(())
}
