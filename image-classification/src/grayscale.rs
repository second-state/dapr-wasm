use image::{ImageFormat, ImageOutputFormat};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn grayscale(image: &[u8]) -> Vec<u8> {
    let image_format_detected: ImageFormat = image::guess_format(&image).unwrap();
    let img = image::load_from_memory(&image).unwrap();
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
    buf
}
