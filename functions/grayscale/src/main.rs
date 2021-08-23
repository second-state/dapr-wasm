use image::{ImageFormat, ImageOutputFormat};
use std::io::{self, Read, Write};

fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    let image_format_detected: ImageFormat = image::guess_format(&buf).unwrap();
    let img = image::load_from_memory(&buf).unwrap();
    let filtered = img.grayscale();
    let mut buf = vec![];
    match image_format_detected {
        ImageFormat::Gif => {
            filtered.write_to(&mut buf, ImageOutputFormat::Gif).unwrap();
        }
        _ => {
            filtered.write_to(&mut buf, ImageOutputFormat::Png).unwrap();
        }
    };
    io::stdout().write_all(&buf).unwrap();
    io::stdout().flush().unwrap();
}
