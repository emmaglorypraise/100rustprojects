use wasm_bindgen::prelude::*;
use image::{ImageOutputFormat, DynamicImage, RgbaImage};
use std::io::Cursor;
 
#[wasm_bindgen]
pub fn apply_grayscale(input: &[u8]) -> Vec<u8> {
    let img = image::load_from_memory(input).expect("Invalid image data");
    let gray = img.grayscale();
 
    let mut output = Cursor::new(Vec::new());
    gray.write_to(&mut output, ImageOutputFormat::Png).unwrap();
    output.into_inner()
}