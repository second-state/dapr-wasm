use wasm_bindgen::prelude::*;
mod infer;

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> String {
    infer::infer_internal(image_data)
}
