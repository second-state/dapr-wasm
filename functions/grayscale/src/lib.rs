#![allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
mod grayscale;

#[wasmedge_bindgen]
pub fn grayscale(image_data: Vec<u8>) -> String {
    grayscale::grayscale_internal(&image_data)
}
