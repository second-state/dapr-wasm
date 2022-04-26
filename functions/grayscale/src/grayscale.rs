extern crate base64;
use image::{ImageFormat, ImageOutputFormat};
use std::io::{self, Read, Write};

pub fn grayscale_internal(image_data: &[u8]) -> String {
    let image_format_detected: ImageFormat = image::guess_format(&image_data).unwrap();
    let img = image::load_from_memory(&image_data).unwrap();
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
    let mut base64_encoded = String::new();
    base64::encode_config_buf(&buf, base64::STANDARD, &mut base64_encoded);
    return base64_encoded.to_string();
}
