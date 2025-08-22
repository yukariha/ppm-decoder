use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

pub struct Image {
    pub filename: String,
    pub magic_number: String,
    pub width: usize,
    pub height: usize,
    pub max_val: u16,
    pixels: Box<[u8]>,
}

impl Image {
    pub fn new(
        filename: String,
        magic_number: String,
        width: usize,
        height: usize,
        max_val: u16,
        pixels: Box<[u8]>,
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
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut iter = reader
            .by_ref()
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .flat_map(|line| {
                line.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .take(4);

        let magic_number: String = iter.next().ok_or("Missing magic number")?;
        if magic_number != "P3" && magic_number != "P6" {
            return Err("Unsupported PPM format".into());
        }

        let width: usize = iter.next().ok_or("Missing width")?.parse()?;
        let height: usize = iter.next().ok_or("Missing height")?.parse()?;
        let max_val: u16 = iter.next().ok_or("Missing max_val")?.parse()?;

        let pixels: Box<[u8]> = match magic_number.as_str() {
            "P6" => {
                let pixel_count = width * height * 3;
                let mut buf = vec![0u8; pixel_count];
                reader.read_exact(&mut buf)?;
                buf.into_boxed_slice()
            }
            "P3" => {
                let mut contents = String::new();
                reader.read_to_string(&mut contents)?;
                contents
                    .par_split_whitespace()
                    .map(|val| val.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
                    .into_boxed_slice()
            }
            _ => return Err("Unsupported magic number".into()),
        };

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
            .par_chunks(3)
            .map(|chunk| {
                let r = chunk[0] as u32;
                let g = chunk[1] as u32;
                let b = chunk[2] as u32;
                (r << 16) | (g << 8) | b
            })
            .collect()
    }
}
