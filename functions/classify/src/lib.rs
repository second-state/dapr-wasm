#![allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
mod infer;

#[wasmedge_bindgen]
pub fn infer(image_data: Vec<u8>) -> String {
    infer::infer_internal(&image_data)
}
