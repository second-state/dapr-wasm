#![allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

mod grayscale;

#[wasmedge_bindgen]
pub fn grayscale(image_data: Vec<u8>) -> String {
    grayscale::grayscale_internal(&image_data)
}

#[wasmedge_bindgen]
pub fn grayscale_str(image_data: String) -> String {
    let image_bytes = image_data.split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    return grayscale::grayscale_internal(&image_bytes);
}