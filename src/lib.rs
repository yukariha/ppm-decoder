use rayon::prelude::*;
use std::{fs, path::Path};

pub struct Image {
    pub filename: String,
    pub magic_number: String,
    pub width: usize,
    pub height: usize,
    pub max_val: u16,
    pixels: Vec<(u8, u8, u8)>,
}

impl Image {
    pub fn new(
        filename: String,
        magic_number: String,
        width: usize,
        height: usize,
        max_val: u16,
        pixels: Vec<(u8, u8, u8)>,
    ) -> Image {
        Image {
            filename,
            magic_number,
            width,
            height,
            max_val,
            pixels,
        }
    }

    pub fn from_file(path: &Path) -> Result<Image, Box<dyn std::error::Error>> {
        let buf: String = fs::read_to_string(path)?;

        let pieces: Vec<&str> = buf
            .par_lines()
            .filter(|line| {
                let line = line.trim();
                !line.is_empty() && !line.starts_with('#')
            })
            .flat_map_iter(|line| line.split_whitespace())
            .collect();

        let (magic_number, width, height, max_val, pixel_data) = (
            pieces[0],
            pieces[1]
                .parse::<usize>()
                .expect("Invalid image width found."),
            pieces[2]
                .parse::<usize>()
                .expect("Invalid image height found."),
            pieces[3].parse::<u16>().expect("Invalid max value found."),
            &pieces[4..],
        );

        let pixels: Vec<(u8, u8, u8)> = pixel_data
            .par_chunks(3)
            .map(|chunk| {
                let r = chunk[0].parse().unwrap();
                let g = chunk[1].parse().unwrap();
                let b = chunk[2].parse().unwrap();
                (r, g, b)
            })
            .collect();

        let filename = path
            .file_name()
            .expect("Failed to get file name")
            .to_string_lossy();

        let image = Image::new(
            filename.into_owned(),
            magic_number.to_string(),
            width,
            height,
            max_val,
            pixels,
        );

        Ok(image)
    }

    pub fn to_minifb_buffer(&self) -> Vec<u32> {
        self.pixels
            .par_iter()
            .map(|&(r, g, b)| (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b))
            .collect()
    }
}
